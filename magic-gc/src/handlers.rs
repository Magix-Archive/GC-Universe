use rouille::{Request, Response};
use serde_json::json;

macro_rules! rsp {
    ($json:tt) => {
        Response::text(serde_json::to_string(&json!($json)).unwrap())
    };
}

/// Route: POST /account/risky/api/check
/// The client sends this request to check if a captcha should be served.
/// request: The request from the client.
pub fn serve_captcha(request: &Request) -> Response {
    Response::text("Hello, world!")
}

/// Route: POST /{game_id}/mdk/shield/api/login
/// The client provides a username & password in exchange for a session key.
/// request: The request from the client.
pub fn create_session(request: &Request, _: String) -> Response {
    Response::text("Hello, world!")
}

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
