use gtk4::{ApplicationWindow, Box as GtkBox};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ViewMode {
    Default,
    Active,
    Divider,
    Sidebar,
}

pub struct GutterState {
    pub mode: ViewMode,
    pub window: Option<ApplicationWindow>,
    pub root_box: Option<GtkBox>,
}

impl GutterState {
    pub fn new(mode: ViewMode) -> Self {
        Self {
            mode,
            window: None,
            root_box: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state() {
        let state = GutterState::new(ViewMode::Default);
        assert_eq!(state.mode, ViewMode::Default);
        assert!(state.window.is_none());
        assert!(state.root_box.is_none());
    }
}
