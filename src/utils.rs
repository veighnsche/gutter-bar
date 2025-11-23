use gtk4::{glib, prelude::*, Label};

pub fn start_clock_tick(h: Label, m: Label, s: Label) {
    let tick = move || {
        let dt = glib::DateTime::now_local().unwrap_or_else(|_| glib::DateTime::from_unix_utc(0).unwrap());
        h.set_text(dt.format("%H").unwrap_or_default().as_str());
        m.set_text(dt.format("%M").unwrap_or_default().as_str());
        s.set_text(dt.format("%S").unwrap_or_default().as_str());
        glib::ControlFlow::Continue
    };
    tick();
    glib::timeout_add_seconds_local(1, tick);
}
