use crate::state::{GutterState, ViewMode};
use crate::utils::start_clock_tick;
use crate::views::update_view;
use gtk4::{prelude::*, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_default_view(
    root: &GtkBox,
    window: &ApplicationWindow,
    state: Rc<RefCell<GutterState>>,
) {
    window.set_default_width(128);
    window.set_width_request(128);

    let content_box = GtkBox::new(Orientation::Vertical, 8);
    content_box.set_margin_top(8);
    content_box.set_margin_bottom(8);
    content_box.set_margin_start(8);
    content_box.set_margin_end(8);
    content_box.set_vexpand(true);

    // 1. Clock
    let clock_box = GtkBox::new(Orientation::Vertical, 0);
    let hour_label = Label::new(None);
    hour_label.add_css_class("clock-digit");
    let minute_label = Label::new(None);
    minute_label.add_css_class("clock-digit");
    let second_label = Label::new(None);
    second_label.add_css_class("clock-digit");

    start_clock_tick(
        hour_label.clone(),
        minute_label.clone(),
        second_label.clone(),
    );

    clock_box.append(&hour_label);
    clock_box.append(&minute_label);
    clock_box.append(&second_label);
    content_box.append(&clock_box);

    // 2. Quick Start Buttons
    let launcher_box = GtkBox::new(Orientation::Vertical, 4);
    let term_btn = Button::with_label("Term");
    term_btn.add_css_class("gutter-btn");
    let browser_btn = Button::with_label("Web");
    browser_btn.add_css_class("gutter-btn");

    // Toggle view button for testing
    let toggle_btn = Button::with_label("Expand");
    toggle_btn.add_css_class("gutter-btn");
    let state_clone = state.clone();
    toggle_btn.connect_clicked(move |_| {
        let mut s = state_clone.borrow_mut();
        s.mode = ViewMode::Active;
        drop(s); // release borrow
        update_view(state_clone.clone());
    });

    launcher_box.append(&term_btn);
    launcher_box.append(&browser_btn);
    launcher_box.append(&toggle_btn);
    content_box.append(&launcher_box);

    // 3. Open Windows List (Placeholder)
    let windows_list = Label::new(Some("Windows:\n- Term\n- Browser"));
    windows_list.set_vexpand(true); // Push stats down
    content_box.append(&windows_list);

    // 4. System Status
    let stats_box = GtkBox::new(Orientation::Vertical, 0);
    let cpu_label = Label::new(Some("CPU: 12%"));
    let mem_label = Label::new(Some("MEM: 45%"));
    stats_box.append(&cpu_label);
    stats_box.append(&mem_label);
    content_box.append(&stats_box);

    root.append(&content_box);
}
