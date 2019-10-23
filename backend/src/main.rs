use backend::server::Server;
use clap::{crate_version, load_yaml, App};
use failure::{format_err, Fallible};
use std::env;
use webapp::config::Config;

fn main() -> Fallible<()> {
    let yaml = load_yaml!("../cli.yaml");
    let matches = App::from_yaml(yaml).version(crate_version!()).get_matches();

    let config_filename = matches
        .value_of("config")
        .ok_or_else(|| format_err!("No 'config' provided"))?;

    let config = Config::from_file(config_filename)?;

    let log_string = format!(
        "actix_web={},webapp={},backend={}",
        config.log.actix_web, config.log.webapp, config.log.webapp
    );
    env::set_var("RUST_LOG", &log_string);

    env_logger::init();
    log::info!(
        "Starting server from config file {} for url {}",
        config_filename,
        config.server.url
    );

    let server = Server::from_config(&config)?;

    server.start()?;

    Ok(())
}
