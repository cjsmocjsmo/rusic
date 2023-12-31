use std::env;

pub fn set_env_vars() {
    let usb = env::var("RUSIC_USB");
    if usb.is_err() {
        env::set_var("RUSIC_USB", "/media/pi/C052-0E64/Music/K");
        // env::set_var("RUSIC_USB", "/media/pi/C052-0E64/");
        // env::set_var("RUSIC_USB", "/home/pipi/Music");
    };
    let db_path = env::var("RUSIC_DB_PATH");
    if db_path.is_err() {
        env::set_var("RUSIC_DB_PATH", "/usr/share/rusic/rusic/db/rusic.db");
    };
    let db_check_file_path = env::var("RUSIC_DB_CHECK_FILE_PATH");
    if db_check_file_path.is_err() {
        env::set_var("RUSIC_DB_CHECK_FILE_PATH", "/usr/share/rusic/rusic/db/db_check_file.txt");
    };
    let no_art_pic = env::var("RUSIC_NO_ART_PIC");
    if no_art_pic.is_err() {
        env::set_var("RUSIC_NO_ART_PIC", "/usr/share/rusic/rusic/no_art_pic.jpg");
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
    let rusic_nfo = env::var("RUSIC_NFOS");
    if rusic_nfo.is_err() {
        env::set_var("RUSIC_NFOS", "/usr/share/rusic/rusic/nfo");
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
