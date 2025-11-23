use crate::state::{GutterState, ViewMode};
use crate::views::update_view;
use gtk4::{prelude::*, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_divider_view(
    root: &GtkBox,
    window: &ApplicationWindow,
    state: Rc<RefCell<GutterState>>,
) {
    window.set_width_request(64);

    let content_box = GtkBox::new(Orientation::Vertical, 8);
    content_box.set_margin_top(4);
    content_box.set_margin_bottom(4);
    content_box.set_margin_start(4);
    content_box.set_margin_end(4);

    let art_label = Label::new(Some("ART"));
    art_label.set_vexpand(true);
    content_box.append(&art_label);

    let back_btn = Button::with_label("Back");
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
