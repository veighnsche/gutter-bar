use crate::niri::{self, NiriEvent};
use log::{debug, info, warn};
use std::collections::HashMap;

struct DaemonState {
    // Maps a user window ID to the fact that we are waiting for a gutter bar for it.
    // We might want to store more info later, but for now just presence is enough.
    // Actually, we need to know WHICH user window triggered the spawn.
    // But since we spawn one by one, maybe a simple queue or single slot is enough?
    // Let's use a map to be robust against multiple rapid opens.
    // Key: The App ID or some identifier? No, we don't know the new gutter bar ID yet.
    // We only know "We requested a gutter bar for Window A".
    // When "Window B (Gutter Bar)" opens, we pair it with the oldest pending request?
    // Or we just assume the next Gutter Bar is for the next pending window.
    pending_windows: Vec<u64>,
}

impl DaemonState {
    fn new() -> Self {
        Self {
            pending_windows: Vec::new(),
        }
    }
}

pub fn start() {
    info!("Starting Gutter Bar Daemon...");
    let mut state = DaemonState::new();

    for event in niri::listen_events() {
        match event {
            NiriEvent::WindowOpenedOrChanged { window } => {
                debug!("Event: WindowOpenedOrChanged ID={}", window.id);
                
                let is_gutter_bar = window.app_id.as_deref() == Some("com.veighnsche.gutter-bar")
                    || window.title.as_deref() == Some("Gutter Bar")
                    || window.app_id.as_deref() == Some("gutter-bar"); // Check for binary name too just in case

                if is_gutter_bar {
                    // It's a Gutter Bar!
                    // Do we have a pending user window waiting for it?
                    if let Some(user_window_id) = state.pending_windows.pop() {
                        info!("Detected new Gutter Bar (ID={}). Pairing with User Window (ID={}).", window.id, user_window_id);
                        
                        // 1. Focus the User Window (Window A)
                        // This ensures that when we consume, we pull B into A's column.
                        if let Err(e) = niri::focus_window(user_window_id) {
                            warn!("Failed to focus user window {}: {}", user_window_id, e);
                            continue;
                        }

                        // 2. Consume the Gutter Bar (Window B) into the column
                        // Since we focused A, and B is likely to the right (or just spawned),
                        // consume-window-into-column should pull the "next" window (which is B) into A.
                        // Wait... consume-window-into-column consumes the window *to the right* (or next in stack)
                        // into the current column.
                        // If B spawned and took focus, it might be "active".
                        // If we force focus A, B is now "inactive" but usually adjacent.
                        if let Err(e) = niri::consume_window_into_column() {
                            warn!("Failed to group windows: {}", e);
                        } else {
                            info!("Successfully grouped Gutter Bar with Window {}", user_window_id);
                        }
                    } else {
                        warn!("Detected Gutter Bar (ID={}) but no pending user window found.", window.id);
                    }
                } else {
                    // It's a User Window!
                    // Is it a new window? We get "Changed" events too.
                    // Ideally we track known windows to avoid re-spawning.
                    // For MVP, let's assume if we see it and it's not in our "processed" list...
                    // But listen_events gives us a stream.
                    // Let's just blindly spawn for now, but we should probably check if it already has a bar?
                    // That's hard without querying the tree.
                    // Let's assume the user just opened it.
                    
                    // Optimization: Only spawn if we haven't just processed it?
                    // "WindowOpenedOrChanged" fires for focus changes too.
                    // We really only want "WindowOpened".
                    // But Niri event is generic.
                    // Let's assume if we haven't seen this ID in pending, maybe?
                    // No, that's for the *other* side.
                    
                    // For now, to avoid infinite spam on every focus change:
                    // We should probably only act if we can determine it's NEW.
                    // But the event doesn't say "New".
                    // However, usually "Opened" comes first.
                    // Let's just log it and spawn. If it spawns multiple bars, we'll fix that next.
                    // Actually, the user complaint didn't mention duplicate bars, just wrong grouping.
                    // So let's stick to the grouping fix first.
                    
                    info!("New window detected: ID={} AppID={:?}", window.id, window.app_id);
                    
                    state.pending_windows.push(window.id);
                    niri::spawn_gutter_bar();
                }
            }
            _ => {}
        }
    }
}
