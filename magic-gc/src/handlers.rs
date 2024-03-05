use rouille::{Request, Response};
use serde_json::json;
use crate::crypto::decrypt_password;
use crate::database::{Account, Counter};
use crate::structs::{AccountCreate, AccountLogin};

macro_rules! rsp {
    ($json:tt) => {
        Response::text(serde_json::to_string(&json!($json)).unwrap())
    };
}

macro_rules! body {
    ($req:tt,$rsp:tt) => {
        match $req.data() {
            Some(data) => data,
            None => return rsp!($rsp)
        }
    }
}

macro_rules! decode {
    ($strc:tt, $body:tt, $rsp:tt) => {
        match serde_json::from_reader::<_, $strc>($body) {
            Ok(body) => body,
            Err(_) => return rsp!($rsp)
        }
    };
}

/// Route: POST /magic-gc/api/account/create
/// Unique route for creating accounts.
/// request: The HTTP request.
pub async fn create_account(request: &Request) -> Response {
    // Attempt to decode the request's JSON body.
    let body = body!(request, { "code": -1, "message": "No body provided." });
    let body = decode!(AccountCreate, body, { "code": -1, "message": "Invalid JSON." });

    // Check if the account with the username exists.
    if Account::find_username(&body.account).await.is_some() {
        return rsp!({ "code": -2, "message": "Account already exists." });
    }
    if Account::find_email(&body.email).await.is_some() {
        return rsp!({ "code": -3, "message": "Email already exists." });
    }

    // Create the account.
    let account = Account {
        id: Counter::next_id("accounts", 100_000_000)
            .await.to_string(),
        username: body.account.clone(),
        email: body.email.clone(),
        password: body.password.clone(),
        login_token: Default::default(),
        session_token: Default::default()
    };
    account.save().await;

    return rsp!({ "code": 0, "message": "Account created successfully." })
}

/// Route: POST /account/risky/api/check
/// The client sends this request to check if a captcha should be served.
/// request: The request from the client.
pub fn serve_captcha(request: &Request) -> Response {
    rsp!({
        "retcode": 0,
        "message": "OK",
        "data": {
            "id": "none",
            "action": "ACTION_NONE"
        }
    })
}

/// Route: POST /{game_id}/mdk/shield/api/login
/// The client provides a username & password in exchange for a session key.
/// request: The request from the client.
pub async fn create_session(request: &Request, _: String) -> Response {
    // Attempt to decode the request's JSON body.
    let body = body!(request, { "retcode": -202, "message": "No body provided." });
    let body = decode!(AccountLogin, body, { "retcode": -202, "message": "Invalid JSON." });

    // Find the account.
    let account = match body.account.contains("@") {
        true => Account::find_email(&body.account).await,
        false => Account::find_username(&body.account).await
    };

    // Attempt to decrypt the password.
    let password = match body.is_crypto {
        false => body.password,
        true => decrypt_password(body.password)
    };
    println!("Password for account is {}", password);

    // Check if the account exists.
    if account.is_none() {
        return rsp!({ "retcode": -201, "message": "Game account cache information error" });
    }

    // Send back the account information.
    let account = account.unwrap();
    rsp!({ "message": "OK", "data": account.login_data() })
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
