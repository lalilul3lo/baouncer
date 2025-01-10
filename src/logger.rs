pub fn init(debug_flag: bool, verbose_flag: bool) {
    if debug_flag {
        std::env::set_var("RUST_LOG", "debug");
    }

    if verbose_flag {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
}
