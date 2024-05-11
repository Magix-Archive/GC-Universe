use std::error::Error;
use std::sync::Mutex;
use handlebars::Handlebars;
use rouille::{Request, Response};
use serde_json::{json, Value};

static HANDLEBARS: Mutex<Option<Handlebars>> = Mutex::new(None);

/// Creates an instance of 'Handlebars' and registers templates.
pub fn init_handlebars() -> Result<(), Box<dyn Error>> {
    let handlebars = Handlebars::new();

    HANDLEBARS.lock().unwrap().replace(handlebars);
    Ok(())
}

/// Responds with a 404 Not Found error.
/// Provides the '404.html' file as a response.
pub fn page_404() -> Response {
    Response::html(include_str!("../resources/404.html"))
        .with_status_code(404)
}

/// Responds with a 500 Internal Server Error.
/// Provides the '500.html' file as a response.
pub fn page_500() -> Response {
    Response::html(include_str!("../resources/500.html"))
        .with_status_code(500)
}

/// Renders a file as a response.
/// file: The file to render.
pub fn render_file(file: &str) -> Response {
    match std::fs::read_to_string(file) {
        Ok(data) => Response::html(data),
        Err(_) => page_500()
    }
}

/// Renders a template with the given data.
/// template: The name of the template. Must be registered.
/// data: The data to render the template with.
pub fn render_template(template: &str, data: Option<Value>) -> Response {
    let handlebars = HANDLEBARS.lock()
        .unwrap().as_ref().unwrap().clone();
    match handlebars.render(template, &data.unwrap_or(json!({}))) {
        Ok(rendered_data) => Response::html(rendered_data),
        Err(_) => page_500()
    }
}

#[macro_export]
macro_rules! rsp {
    ($json:tt) => {
        Response::text(serde_json::to_string(&json!($json)).unwrap())
    };
}

#[macro_export]
macro_rules! body {
    ($req:tt,$rsp:tt) => {
        match $req.data() {
            Some(data) => data,
            None => return rsp!($rsp)
        }
    }
}

#[macro_export]
macro_rules! decode {
    ($strc:tt, $body:tt, $rsp:tt) => {
        match serde_json::from_reader::<_, $strc>($body) {
            Ok(body) => body,
            Err(_) => return rsp!($rsp)
        }
    };
}

pub mod login;
pub mod logging;
pub mod payment;
pub mod config;
pub mod account;
pub mod verify;
pub mod misc;

/// Route: POST /{game_id}/mdk/shield/api/verify
/// The client provides a cached session key to validate its lifetime.
/// request: The request from the client.
pub fn verify_token(request: &Request, _: String) -> Response {
    Response::text("Hello, world!")
}

/// Route: POST /{game_id}/combo/granter/login/v2/login
/// The client provides a session key in exchange for a login token.
/// request: The request from the client.
pub fn key_exchange(request: &Request, _: String) -> Response {
    Response::text("Hello, world!")
}

/// Route: POST /sdk/dataUpload
/// The client sends crash reports and other data over this route.
/// request: The request from the client.
pub fn data_upload(request: &Request) -> Response {
    rsp!({ "code": 0 })
}
