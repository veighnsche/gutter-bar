use crate::niri::{self, NiriEvent};
use std::thread;
use std::time::Duration;

pub fn start() {
    println!("Starting Gutter Bar Daemon...");

    for event in niri::listen_events() {
        match event {
            NiriEvent::WindowOpenedOrChanged { window } => {
                // Check if the window is a gutter bar to avoid infinite loops
                let is_gutter_bar = window.app_id.as_deref() == Some("com.veighnsche.gutter-bar")
                    || window.title.as_deref() == Some("Gutter Bar");

                if !is_gutter_bar {
                    println!("New window detected: ID={} AppID={:?}", window.id, window.app_id);
                    
                    // Spawn a new gutter bar
                    niri::spawn_gutter_bar();

                    // Wait a bit for the window to appear
                    // In a robust implementation, we would wait for the specific window ID event
                    thread::sleep(Duration::from_millis(500));

                    // Group it into the column
                    niri::consume_window_into_column();
                }
            }
            _ => {}
        }
    }
}
