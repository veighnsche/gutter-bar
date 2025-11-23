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
    pub fn new() -> Self {
        Self {
            mode: ViewMode::Default,
            window: None,
            root_box: None,
        }
    }
}
