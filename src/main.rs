use env_logger::{Builder, Target};
use std::time::Instant;

use crate::setup::rusic_utils::is_db_check_file_present;

pub mod envvars;
pub mod setup;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    Builder::new().target(Target::Stdout).init();

    log::info!("Rusic setup started");

    let _set_envvars = envvars::set_env_vars();

    if !is_db_check_file_present() {
        let _setup = setup::setup();
    }



    let duration = start.elapsed();
    log::info!("Setup completed in: {} seconds", duration.as_secs());
    println!("Setup completed in: {} seconds", duration.as_secs());

    Ok(())
}

