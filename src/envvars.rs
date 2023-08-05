use std::env;

pub fn set_env_vars() {
    let usb1 = env::var("RUSIC_USB1");
    if usb1.is_err() {
        env::set_var("RUSIC_USB1", "/media/pi/FOO/media");
    };
    let usb2 = env::var("RUSIC_USB2");
    if usb2.is_err() {
        env::set_var("RUSIC_USB2", "/media/pi/USB2/media");
    };
    let usb3 = env::var("RUSIC_USB3");
    if usb3.is_err() {
        env::set_var("RUSIC_USB3", "/media/pi/USB3/media");
    };
    let usb4 = env::var("RUSIC_USB4");
    if usb4.is_err() {
        env::set_var("RUSIC_USB4", "None");
    };
    let pagination = env::var("RUSIC_PAGINATION");
    if pagination.is_err() {
        env::set_var("RUSIC_PAGINATION", "25");
    };
    let rusic = env::var("RUSIC_PATH");
    if rusic.is_err() {
        env::set_var("RUSIC_PATH", "/usr/share/rusic/rusic");
    };
    let rusic_thumbs = env::var("RUSIC_THUMBS");
    if rusic_thumbs.is_err() {
        env::set_var("RUSIC_THUMBS", "/usr/share/rusic/rusic/thumbs");
    };
    let rusic_raw_http = env::var("RUSIC_RAW_HTTP");
    if rusic_raw_http.is_err() {
        env::set_var("RUSIC_RAW_HTTP", "192.168.0.26");
    };
    let rusic_http = env::var("RUSIC_HTTP_ADDR");
    if rusic_http.is_err() {
        env::set_var("RUSIC_HTTP_ADDR", "http://192.168.0.26");
    };
    let rusic_port = env::var("RUSIC_PORT");
    if rusic_port.is_err() {
        env::set_var("RUSIC_PORT", ":8080");
    };
}
