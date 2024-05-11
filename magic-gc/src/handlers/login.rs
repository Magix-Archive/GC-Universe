use serde_json::json;
use rouille::{Request, Response};
use crate::crypto::{decrypt_password, verify_password};
use crate::database::Account;
use crate::structs::{AccountLogin};

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

    // Check if the account exists.
    if account.is_none() {
        return rsp!({ "retcode": -201, "message": "Game account cache information error" });
    }

    // Attempt to decrypt the password.
    let password = match body.is_crypto {
        false => body.password,
        true => decrypt_password(body.password)
    };

    // Check if the password matches the account's password.
    let account = account.unwrap();
    if !verify_password(&password, &account.salt, &account.password) {
        return rsp!({ "retcode": -201, "message": "Incorrect or invalid password" });
    }

    // Send back the account information.
    rsp!({ "retcode": 0, "message": "OK", "data": account.login_data() })
}
