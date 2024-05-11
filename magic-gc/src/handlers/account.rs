use serde_json::json;
use rouille::{Request, Response};
use crate::crypto::hash_password;
use crate::database::{Account, Counter};
use crate::handlers::render_file;
use crate::structs::AccountCreate;

/// Route: POST /account/register
/// Unique route for creating accounts.
/// request: The HTTP request.
pub async fn create_account(request: &Request) -> Response {
    // Send the account form.
    if request.method() == "GET" {
        return render_file("static/account/register.html");
    }

    // Attempt to decode the request's JSON body.
    let body = body!(request, { "code": -1, "message": "No body provided." });
    let body = decode!(AccountCreate, body, { "code": -1, "message": "Invalid JSON." });

    // Check if the account with the username exists.
    if Account::find_username(&body.account).await.is_some() {
        return rsp!({ "code": -2, "message": "Account with username already exists." });
    }
    if Account::find_email(&body.email).await.is_some() {
        return rsp!({ "code": -3, "message": "Account with email already exists." });
    }

    // Hash the password.
    let (password, salt) = hash_password(body.password);
    // Create the account.
    let account = Account {
        id: Counter::next_id("accounts", 100_000_000)
            .await.to_string(),
        username: body.account.clone(),
        email: body.email.clone(),
        password, salt,
        login_token: Default::default(),
        session_token: Default::default()
    };
    account.save().await;

    return rsp!({ "code": 0, "message": "Account created successfully." })
}
