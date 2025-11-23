use crate::state::{GutterState, ViewMode};
use crate::views::update_view;
use gtk4::{prelude::*, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_sidebar_view(
    root: &GtkBox,
    window: &ApplicationWindow,
    state: Rc<RefCell<GutterState>>,
) {
    window.set_width_request(64); // 64px total, split 32/32 visually if needed, or just 64px content

    let content_box = GtkBox::new(Orientation::Horizontal, 0);

    let left_bar = GtkBox::new(Orientation::Vertical, 0);
    left_bar.set_hexpand(true);
    left_bar.add_css_class("sidebar-left");
    let l_label = Label::new(Some("LEFT"));
    l_label.set_vexpand(true);
    l_label.set_valign(gtk4::Align::Center);
    left_bar.append(&l_label);

    let right_bar = GtkBox::new(Orientation::Vertical, 0);
    right_bar.set_hexpand(true);
    right_bar.add_css_class("sidebar-right");
    let r_label = Label::new(Some("RIGHT"));
    r_label.set_vexpand(true);
    r_label.set_valign(gtk4::Align::Center);
    right_bar.append(&r_label);

    content_box.append(&left_bar);
    content_box.append(&right_bar);

    // Invisible button to switch back for demo
    let back_btn = Button::with_label("X");
    let state_clone = state.clone();
    back_btn.connect_clicked(move |_| {
        let mut s = state_clone.borrow_mut();
        s.mode = ViewMode::Default;
        drop(s);
        update_view(state_clone.clone());
    });
    content_box.append(&back_btn);

    root.append(&content_box);
}
