use serde::Deserialize;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum NiriEvent {
    WindowOpenedOrChanged { window: WindowInfo },
    #[serde(other)]
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
        if let Ok(l) = line {
            serde_json::from_str(&l).ok()
        } else {
            None
        }
    })
}

pub fn spawn_gutter_bar() {
    // Spawn the gutter-bar view detached
    let _ = Command::new(std::env::current_exe().unwrap_or_else(|_| "gutter-bar".into()))
        .arg("view")
        .spawn()
        .expect("Failed to spawn gutter bar view");
}

pub fn consume_window_into_column() {
    // This action consumes the window to the right into the focused column.
    // We assume the new gutter bar spawned and took focus or is to the right.
    // Actually, when we spawn a new window, it usually takes focus.
    // If we are the daemon, we need to be careful.
    
    // Strategy:
    // 1. New window opens (User App).
    // 2. Daemon spawns Gutter Bar.
    // 3. Gutter Bar opens.
    // 4. We want to group them.
    
    // If the Gutter Bar opens to the right of the User App, we can focus the User App and call "consume-window-into-column".
    
    let _ = Command::new("niri")
        .args(["msg", "action", "consume-window-into-column"])
        .output();
}
