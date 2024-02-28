#![feature(async_fn_in_trait)]

mod options;
mod handlers;
mod database;
mod structs;
mod utils;

use log::info;
use config::{*, ext::*};
use rouille::{Response, router};
use crate::options::Options;

macro_rules! awaitexpr {
    ($func:expr) =>{
        tokio::runtime::Builder::new_current_thread()
        .enable_all().build()
        .expect("Failed to build Tokio runtime")
        .block_on(async { $func.await })
    };
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // Load the configuration.
    let config = DefaultConfigurationBuilder::new()
        .add_json_file("config.json")
        .add_env_vars()
        .build()
        .expect("Failed to build configuration.");
    let app: Options = config.reify();

    // Build the database.
    database::setup_client(app.clone()).await;

    // Build the web server.
    let address = format!("{}:{}", app.server.host, app.server.port);
    info!("Starting server on {}...", address);

    rouille::start_server(address, move |request| {
        router!(request,
            (POST) (/account/risky/api/check) => { handlers::serve_captcha(request) },
            (POST) (/{game_id: String}/mdk/shield/api/login) => { awaitexpr!(handlers::create_session(request, game_id)) },
            (POST) (/{game_id: String}/mdk/shield/api/verify) => { handlers::verify_token(request, game_id) },
            (POST) (/{game_id: String}/combo/granter/login/v2/login) => { handlers::key_exchange(request, game_id) },

            (POST) (/sdk/dataUpload) => { handlers::data_upload(request) },
            (POST) (/crashdump/dataUpload) => { handlers::data_upload(request) },
            (POST) (/apm/dataUpload) => { handlers::data_upload(request) },

            (POST) (/magic-gc/api/account/create) => { awaitexpr!(handlers::create_account(request)) },

            _ => {
                Response::text("404 Not Found")
                    .with_status_code(404)
            }
        )
    });
}
