use crate::state::{GutterState, ViewMode};
use crate::views::update_view;
use gtk4::{prelude::*, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use std::cell::RefCell;
use std::rc::Rc;

pub fn build_active_view(
    root: &GtkBox,
    window: &ApplicationWindow,
    state: Rc<RefCell<GutterState>>,
) {
    window.set_width_request(512);

    let content_box = GtkBox::new(Orientation::Vertical, 16);
    content_box.set_margin_top(16);
    content_box.set_margin_bottom(16);
    content_box.set_margin_start(16);
    content_box.set_margin_end(16);

    let header = GtkBox::new(Orientation::Horizontal, 8);
    let search_entry = gtk4::Entry::new();
    search_entry.set_placeholder_text(Some("Search apps..."));
    search_entry.set_hexpand(true);

    let close_btn = Button::with_label("Collapse");
    let state_for_close = state.clone();
    close_btn.connect_clicked(move |_| {
        let mut s = state_for_close.borrow_mut();
        s.mode = ViewMode::Default;
        drop(s);
        update_view(state_for_close.clone());
    });

    header.append(&search_entry);
    header.append(&close_btn);
    content_box.append(&header);

    let preview_label = Label::new(Some("Live Previews"));
    preview_label.add_css_class("h2");
    content_box.append(&preview_label);

    let preview_grid = gtk4::FlowBox::new();
    preview_grid.set_valign(gtk4::Align::Start);
    preview_grid.set_max_children_per_line(3);
    preview_grid.set_selection_mode(gtk4::SelectionMode::None);

    for i in 1..=6 {
        let card = GtkBox::new(Orientation::Vertical, 4);
        card.add_css_class("preview-card");
        card.set_size_request(140, 100);
        let label = Label::new(Some(&format!("Window {}", i)));
        card.append(&label);
        preview_grid.insert(&card, -1);
    }
    content_box.append(&preview_grid);

    let stats_label = Label::new(Some("System Status"));
    content_box.append(&stats_label);

    let cpu_bar = gtk4::ProgressBar::new();
    cpu_bar.set_fraction(0.45);
    cpu_bar.set_show_text(true);
    cpu_bar.set_text(Some("CPU 45%"));
    cpu_bar.add_css_class("chart-bar");
    content_box.append(&cpu_bar);

    let mem_bar = gtk4::ProgressBar::new();
    mem_bar.set_fraction(0.72);
    mem_bar.set_show_text(true);
    mem_bar.set_text(Some("MEM 72%"));
    mem_bar.add_css_class("chart-bar");
    content_box.append(&mem_bar);

    // Test buttons for other views
    let nav_box = GtkBox::new(Orientation::Horizontal, 8);
    let divider_btn = Button::with_label("Divider Mode");
    let sidebar_btn = Button::with_label("Sidebar Mode");

    let state_clone = state.clone();
    divider_btn.connect_clicked(move |_| {
        let mut s = state_clone.borrow_mut();
        s.mode = ViewMode::Divider;
        drop(s);
        update_view(state_clone.clone());
    });

    let state_clone2 = state.clone();
    sidebar_btn.connect_clicked(move |_| {
        let mut s = state_clone2.borrow_mut();
        s.mode = ViewMode::Sidebar;
        drop(s);
        update_view(state_clone2.clone());
    });

    nav_box.append(&divider_btn);
    nav_box.append(&sidebar_btn);
    content_box.append(&nav_box);

    root.append(&content_box);
}
