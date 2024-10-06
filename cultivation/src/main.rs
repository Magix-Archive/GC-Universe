mod options;
mod custom;
mod proxy;
mod system;
mod structs;

use libc::c_char;
use std::ffi::CString;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use clap::{arg, Arg, Command};
use config::{*, ext::*};
use once_cell::sync::Lazy;
use common::utils;
use crate::custom::snowflake_path;
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

extern "C" {
    fn open_game(game_path: *const c_char, dll_path: *const c_char, skip_driver: bool);
}

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
                .arg(arg!(--custom <ENABLED> "Connect to a private server")
                    .value_parser(clap::value_parser!(bool))
                    .num_args(0..=1)
                    .default_value("false")
                    .default_missing_value("true"))
                .arg(arg!(--proxy <ENABLED> "Use the Cultivation proxy")
                    .value_parser(clap::value_parser!(bool))
                    .num_args(0..=1)
                    .default_value("true")
                    .default_missing_value("true"))
                .arg(arg!(--skip_driver <ENABLED> "Skips checking for the anti-cheat driver")
                    .value_parser(clap::value_parser!(bool))
                    .num_args(0..=1)
                    .default_value("false")
                    .default_missing_value("false"))
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
                        .arg(arg!([ENCRYPTED] "Whether to use HTTPS or HTTP")
                            .value_parser(clap::value_parser!(bool))
                            .required(false)
                            .default_value("false"))
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
                    let with_proxy = matches.get_one::<bool>("proxy")
                        .unwrap_or(&true);
                    let skip_driver = matches.get_one::<bool>("skip_driver")
                        .unwrap_or(&false);

                    // Check if the game path exists.
                    if !utils::file_exists(&game.path) {
                        println!("Game path does not exist.");
                        return;
                    }

                    // Create and enable the proxy.
                    if *custom {
                        if *with_proxy {
                            proxy::create_proxy(app.clone());
                            custom::enable_proxy(&app);
                        }

                        // Launch the game.
                        let game_path = CString::new(game.path.clone()).unwrap();
                        let dll_path = CString::new(snowflake_path()).unwrap();

                        unsafe {
                            open_game(game_path.as_ptr(), dll_path.as_ptr(), *skip_driver);
                        }
                    } else {
                        // Launch the game.
                        std::process::Command::new(&game.path)
                            .spawn()
                            .expect("Failed to launch game.");
                    }

                    // Wait for the game to close or until the user stops the app.
                    let process_name = game.path.replace("/", "\\");
                    let process_name = process_name.split("\\").last()
                        .expect("No executable name provided.");

                    println!("------------------------------");
                    println!("Waiting for game to close...");
                    println!("You can also stop by pressing Ctrl + C");
                    system::wait_for_action(process_name.to_string()).await;

                    if *with_proxy {
                        custom::disable_proxy();
                    }
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

                    println!("------------------------------");
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
                    let encrypted = matches.get_one::<bool>("ENCRYPTED").unwrap();
                    app.set_server(&game, host.clone(), port.clone(), *encrypted);
                }
                _ => println!("No config option provided.")
            }
        },
        _ => {
            println!("This should launch the Cultivation UI, but it's not done yet.");
            println!("You should do 'cultivation help' instead!");
        }
    }
}
