use env_logger::{Builder, Target};
use std::time::Instant;

pub mod envvars;
pub mod server;
pub mod setup;
pub mod rusicdb;
pub mod types;

fn main() -> std::io::Result<()> {
    let start = Instant::now();
    Builder::new().target(Target::Stdout).init();

    log::info!("Rusic setup started");

    let _set_envvars = envvars::set_env_vars();

    let _setup = setup::setup();

    let duration = start.elapsed();
    log::info!("Setup completed in: {} seconds", duration.as_secs());
    println!("Setup completed in: {} seconds", duration.as_secs());

    println!("Starting server...");
    let _server = server::rusic_server_main();

    Ok(())
}
