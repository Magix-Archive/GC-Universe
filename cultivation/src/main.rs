mod options;
mod utils;
mod custom;
mod proxy;
mod system;
mod structs;

use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use clap::{arg, Arg, Command};
use config::{*, ext::*};
use once_cell::sync::Lazy;
use crate::options::Options;
use crate::structs::State;

macro_rules! global {
    ($name:tt, $type:tt) => {
        pub static $name: Lazy<Arc<Mutex<$type>>> = Lazy::new(|| Arc::new(Mutex::new($type::default())));
    };
}

#[macro_export]
macro_rules! str {
    ($str:tt) => {
        $str.parse().unwrap()
    };
}

#[macro_export]
macro_rules! fetch {
    ($global:tt,$var:tt) => {
        let mut $var = $global.deref().lock().unwrap();
    };
}

global!(STATE, State);

fn game() -> Arg {
    arg!([GAME] "The game to configure")
        .required(true)
        .default_value("genshin")
        .value_parser(["genshin", "starrail"])
}

fn clap() -> Command {
    Command::new("cultivation")
        .about("Anime game launching utility")
        .arg(arg!(-A --no_admin "Run the launcher without requiring admin")
            .required(false)
            .default_value("false")
            .default_missing_value("true")
            .value_parser(clap::value_parser!(bool)))
        .subcommand(
            Command::new("launch")
                .about("Launches the game")
                .arg(game())
                .arg(arg!(--custom "Connect to a private server")
                    .value_parser(clap::value_parser!(bool))
                    .default_value("false")
                    .require_equals(true))
        )
        .subcommand(
            Command::new("proxy")
                .about("Starts the proxy server")
                .arg(game())
        )
        .subcommand(
            Command::new("set")
                .about("Set a configuration option")
                .arg_required_else_help(true)
                .subcommand(
                    Command::new("game-path")
                        .about("Set the game path")
                        .arg_required_else_help(true)
                        .arg(game())
                        .arg(arg!([PATH] "The path to the game executable")
                            .required(true)
                            .value_parser(clap::value_parser!(PathBuf)))
                )
                .subcommand(
                    Command::new("server")
                        .about("Set the server to connect to")
                        .arg_required_else_help(true)
                        .arg(game())
                        .arg(arg!([HOST] "The host to connect to")
                            .required(true))
                        .arg(arg!([PORT] "The port to connect to")
                            .value_parser(clap::value_parser!(u16))
                            .required(true))
                )
        )
}

#[tokio::main]
async fn main() {
    // Check if the configuration exists.
    if !utils::file_exists("config.json") {
        utils::write_json("config.json", Options::default())
            .expect("Failed to write configuration.");
    }

    // Load the configuration.
    let config = DefaultConfigurationBuilder::new()
        .add_json_file("config.json")
        .add_env_vars()
        .build()
        .expect("Failed to build configuration.");
    let app: Options = config.reify();

    // Set the global state.
    fetch!(STATE, state);
    state.options = app.clone();

    // Parse the command line arguments.
    let matches = clap().get_matches();

    // Quickly update the state.
    state.require_admin = !*matches.get_one::<bool>("no_admin").unwrap_or(&false);
    // Unlock the state.
    drop(state);

    // Handle sub-commands.
    match matches.subcommand() {
        Some(("launch", matches)) => {
            let game = matches.get_one::<String>("GAME");
            match game {
                Some(game) => {
                    // Check for elevation.
                    system::elevate();

                    let game = app.game_from_name(game);
                    let custom = matches.get_one::<bool>("custom")
                        .unwrap_or(&false);

                    // Enable the proxy & patch the game if required.
                    if *custom {
                        custom::patch_game(&game);
                        custom::enable_proxy(&game);
                    }

                    // Launch the game.
                    std::process::Command::new(&game.path)
                        .spawn()
                        .expect("Failed to launch game.");
                },
                None => println!("No game provided.")
            }
        },
        Some(("proxy", matches)) => {
            let game = matches.get_one::<String>("GAME");
            match game {
                Some(game) => {
                    // Check for elevation.
                    system::elevate();

                    State::instance().selected_game = game.clone();
                    proxy::create_proxy(app.clone());
                    println!("Started proxy on {}:{}!",
                             app.proxy.host, app.proxy.port);
                    println!("Press Ctrl+C to stop the proxy.");

                    // Wait for the user to stop the proxy.
                    tokio::signal::ctrl_c()
                        .await.expect("Failed to install CTRL+C signal handler");
                },
                None => println!("No game provided.")
            }
        },
        Some(("set", matches)) => {
            match matches.subcommand() {
                Some(("game-path", matches)) => {
                    let game = matches.get_one::<String>("GAME").unwrap();
                    let path = matches.get_one::<PathBuf>("PATH").unwrap();
                    app.set_game_path(&game, path.to_str().unwrap().to_string());
                }
                Some(("server", matches)) => {
                    let game = matches.get_one::<String>("GAME").unwrap();
                    let host = matches.get_one::<String>("HOST").unwrap();
                    let port = matches.get_one::<u16>("PORT").unwrap();
                    app.set_server(&game, host.clone(), port.clone());
                }
                _ => println!("No config option provided.")
            }
        },
        _ => {
            loop {

            }
        }
    }
}
