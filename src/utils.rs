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

pub fn format_time_parts(dt: &glib::DateTime) -> (String, String, String) {
    let h = dt.format("%H").unwrap_or_default().to_string();
    let m = dt.format("%M").unwrap_or_default().to_string();
    let s = dt.format("%S").unwrap_or_default().to_string();
    (h, m, s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_time_parts() {
        // Create a fixed time: 2023-01-01 12:34:56 UTC
        let dt = glib::DateTime::from_unix_utc(1672576496).unwrap();
        let (h, m, s) = format_time_parts(&dt);
        
        // Note: from_unix_utc returns UTC time. 12:34:56 UTC.
        assert_eq!(h, "12");
        assert_eq!(m, "34");
        assert_eq!(s, "56");
    }
}
