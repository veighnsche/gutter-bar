use serde::Deserialize;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use log::{debug, error, warn};

#[derive(Debug, Deserialize)]
pub enum NiriEvent {
    WindowOpenedOrChanged { window: WindowInfo },
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct WindowInfo {
    pub id: u64,
    pub app_id: Option<String>,
    pub title: Option<String>,
}

pub fn listen_events() -> impl Iterator<Item = NiriEvent> {
    let child = Command::new("niri")
        .args(["msg", "-j", "event-stream"])
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to start niri event stream");

    let stdout = child.stdout.expect("Failed to capture stdout");
    let reader = BufReader::new(stdout);

    reader.lines().filter_map(|line| {
        match line {
            Ok(l) => {
                debug!("Raw Niri Event: {}", l);
                // Parse as a generic Value first to handle the external tagging and unknown events
                match serde_json::from_str::<serde_json::Value>(&l) {
                    Ok(val) => {
                        // Check for the specific event we care about
                        if let Some(window_obj) = val.get("WindowOpenedOrChanged") {
                            // The inner object is {"window": {...}}
                            // We want to deserialize that into our struct structure
                            // But wait, our enum variant expects { window: WindowInfo }
                            // The JSON is {"WindowOpenedOrChanged": {"window": {...}}}
                            // So window_obj is {"window": {...}}
                            
                            match serde_json::from_value::<WindowOpenedPayload>(window_obj.clone()) {
                                Ok(payload) => Some(NiriEvent::WindowOpenedOrChanged { window: payload.window }),
                                Err(e) => {
                                    warn!("Failed to parse WindowOpenedOrChanged payload: {}", e);
                                    None
                                }
                            }
                        } else {
                            // It's some other event, ignore it without warning
                            Some(NiriEvent::Unknown)
                        }
                    },
                    Err(e) => {
                        warn!("Failed to parse JSON line: {} | Error: {}", l, e);
                        None
                    }
                }
            }
            Err(e) => {
                error!("Error reading line from niri: {}", e);
                None
            }
        }
    })
}

#[derive(Deserialize)]
struct WindowOpenedPayload {
    window: WindowInfo,
}

pub fn spawn_gutter_bar() {
    // Spawn the gutter-bar view detached
    let _ = Command::new(std::env::current_exe().unwrap_or_else(|_| "gutter-bar".into()))
        .arg("view")
        .spawn()
        .map_err(|e| error!("Failed to spawn gutter bar view: {}", e));
}

pub fn focus_window(id: u64) -> std::io::Result<()> {
    debug!("Focusing window ID={}", id);
    let status = Command::new("niri")
        .args(["msg", "action", "focus-window", "--id", &id.to_string()])
        .status()?;
    
    if !status.success() {
        warn!("Failed to focus window {}", id);
    }
    Ok(())
}

pub fn consume_window_into_column() -> std::io::Result<()> {
    debug!("Consuming window into column");
    let status = Command::new("niri")
        .args(["msg", "action", "consume-window-into-column"])
        .status()?;

    if !status.success() {
        warn!("Failed to consume window into column");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_window_opened() {
        // This matches the actual Niri output format (external tagging)
        let json = r#"{"WindowOpenedOrChanged":{"window":{"id":123,"app_id":"org.gnome.Terminal","title":"Terminal"}}}"#;
        
        // We need to simulate the logic inside listen_events since we changed the parsing strategy
        // But for unit testing the structs, we can verify the payload parsing
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        let window_obj = val.get("WindowOpenedOrChanged").unwrap();
        let payload: WindowOpenedPayload = serde_json::from_value(window_obj.clone()).unwrap();
        
        assert_eq!(payload.window.id, 123);
        assert_eq!(payload.window.app_id.as_deref(), Some("org.gnome.Terminal"));
    }

    #[test]
    fn test_parse_unknown_event() {
        let json = r#"{"SomeNewEvent":{"data":{}}}"#;
        let val: serde_json::Value = serde_json::from_str(json).unwrap();
        assert!(val.get("WindowOpenedOrChanged").is_none());
    }
}
