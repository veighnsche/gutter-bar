use gtk4::prelude::*;
use gtk4::{Application, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.example.gutter-bar")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Gutter Bar")
        .default_width(400)
        .default_height(40)
        .build();

    window.present();
}
