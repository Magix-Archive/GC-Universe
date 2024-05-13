mod keys;
mod sniffer;
mod options;

use std::error::Error;

use clap::Command;
use colored::Colorize;
use common::utils;
use config::{ext::{ConfigurationBinder, EnvironmentVariablesExtensions, JsonConfigurationExtensions}, ConfigurationBuilder, DefaultConfigurationBuilder};
use log::{error, info};
use options::Options;
use pcap::Device;

fn clap() -> Command {
    Command::new("ayylmao")
        .about("Anime game packet capture utility")
        .subcommand(Command::new("devices")
            .about("Lists all available network devices."))
        .subcommand(Command::new("sniff")
            .about("Runs the application in 'sniffer' mode."))
        .subcommand(Command::new("deobfu")
            .about("Runs the application in 'deobfuscation' mode."))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger.
    if !std::env::var("RUST_LOG").is_ok() {
        std::env::set_var("RUST_LOG", "info");
    }
    pretty_env_logger::init();

    // Check if the configuration exists.
    if !utils::file_exists("config.json") {
        utils::write_json("config.json", Options::default())?;
    }

    // Load the configuration.
    let config = DefaultConfigurationBuilder::new()
        .add_json_file("config.json")
        .add_env_vars()
        .build()
        .unwrap();
    let app: Options = config.reify();

    // Parse the command line arguments.
    let matches = clap().get_matches();

    // Handle sub-commands.
    match matches.subcommand() {
        Some(("devices", _)) => list_devices(),
        Some(("sniff", _)) => {
            let device = sniffer::get_device(&app);
            sniffer::capture(device).await;
        },
        _ => {
            error!("Invalid sub-command.");
            info!("See 'ayylmao help' for more information.");
        }
    };

    Ok(())
}

/// Lists all network devices for sniffing.
/// Taken from: https://github.com/TheLostTree/evergreen/blob/master/evergreen/src/iridium_backend/main.rs
fn list_devices() {
    let devices = Device::list().unwrap();
    for (i, device) in devices.iter().enumerate() {
        info!(
            "{}: {}, {}, Status: {}",
            i.to_string().bold(), 
            device.name.bright_blue(),
            device.desc.as_ref().unwrap_or(&"N/A".to_string()),
            format!("{:?}", device.flags.connection_status).bright_green()
        );
    }
}