mod cli;
mod daemon;
mod niri;
mod state;
mod utils;
mod views;

use clap::Parser;
use cli::{Cli, Commands};
use gtk4::gdk;
use gtk4::prelude::*;
use gtk4::{
    Application, ApplicationWindow, Box as GtkBox, CssProvider, Orientation,
    STYLE_PROVIDER_PRIORITY_APPLICATION,
};
use state::{GutterState, ViewMode};
use std::cell::RefCell;
use std::rc::Rc;
use views::update_view;

fn main() {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Daemon => {
            daemon::start();
        }
        Commands::View { mode } => {
            let app = Application::builder()
                .application_id("com.veighnsche.gutter-bar")
                .build();

            let view_mode = match mode.as_deref() {
                Some("active") => ViewMode::Active,
                Some("divider") => ViewMode::Divider,
                Some("sidebar") => ViewMode::Sidebar,
                _ => ViewMode::Default,
            };

            let state = Rc::new(RefCell::new(GutterState::new(view_mode)));

            app.connect_activate(move |app| {
                build_ui(app, state.clone());
            });

            app.run();
        }
    }
}

fn build_ui(app: &Application, state: Rc<RefCell<GutterState>>) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gutter Bar")
        .decorated(false)
        .build();

    let provider = CssProvider::new();
    provider.load_from_data(
        "
        .clock-digit { font-family: monospace; font-size: 32px; }
        .gutter-btn { margin: 4px; padding: 8px; }
        .view-container { background-color: #2e3440; color: #eceff4; }
        .preview-card { background-color: #3b4252; margin: 4px; padding: 8px; border-radius: 4px; }
        .chart-bar { margin: 4px; }
        .sidebar-left { background-color: #434c5e; margin-right: 2px; }
        .sidebar-right { background-color: #4c566a; margin-left: 2px; }
        ",
    );
    let display = gdk::Display::default().expect("Failed to get default display");
    gtk4::style_context_add_provider_for_display(
        &display,
        &provider,
        STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let root = GtkBox::new(Orientation::Vertical, 0);
    root.add_css_class("view-container");
    window.set_child(Some(&root));

    {
        let mut s = state.borrow_mut();
        s.window = Some(window.clone());
        s.root_box = Some(root.clone());
    }

    update_view(state.clone());

    window.present();
}
