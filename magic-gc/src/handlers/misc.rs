use rouille::{Request, Response};
use serde_json::json;

/// Route: POST /account/risky/api/check
/// The client sends this request to check if a captcha should be served.
/// request: The request from the client.
pub fn serve_captcha(_: &Request) -> Response {
    rsp!({ "retcode": 0, "message": "OK", "data": { "id": "none", "action": "ACTION_NONE" } })
}
