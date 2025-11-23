pub mod active;
pub mod default;
pub mod divider;
pub mod sidebar;

use crate::state::{GutterState, ViewMode};
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

pub fn update_view(state: Rc<RefCell<GutterState>>) {
    let (mode, root, window) = {
        let s = state.borrow();
        if let (Some(root), Some(window)) = (&s.root_box, &s.window) {
            (s.mode, root.clone(), window.clone())
        } else {
            return;
        }
    };

    // Clear current content
    while let Some(child) = root.first_child() {
        root.remove(&child);
    }

    match mode {
        ViewMode::Default => default::build_default_view(&root, &window, state.clone()),
        ViewMode::Active => active::build_active_view(&root, &window, state.clone()),
        ViewMode::Divider => divider::build_divider_view(&root, &window, state.clone()),
        ViewMode::Sidebar => sidebar::build_sidebar_view(&root, &window, state.clone()),
    }
}
