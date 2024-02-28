use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AccountLogin {
    pub account: String,
    pub password: String,
    pub is_crypto: bool
}

#[derive(Deserialize)]
pub struct AccountCreate {
    pub email: String,
    pub account: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct LoginData {
    pub uid: Option<String>,
    pub name: String,
    pub email: String,
    pub mobile: String,
    pub is_email_verify: String,
    #[serde(rename = "realname")]
    pub real_name: String,
    pub identity_card: String,
    pub token: Option<String>,
    pub safe_mobile: String,
    pub facebook_name: String,
    pub twitter_name: String,
    pub game_center_name: String,
    pub google_name: String,
    pub apple_name: String,
    pub sony_name: String,
    pub tap_name: String,
    pub country: String,
    pub reactivate_ticket: String,
    pub area_code: String,
    pub device_grant_ticket: String,
}

impl Default for LoginData {
    fn default() -> Self {
        LoginData {
            uid: None,
            name: Default::default(),
            email: Default::default(),
            mobile: Default::default(),
            is_email_verify: "0".to_string(),
            real_name: Default::default(),
            identity_card: Default::default(),
            token: None,
            safe_mobile: Default::default(),
            facebook_name: Default::default(),
            twitter_name: Default::default(),
            game_center_name: Default::default(),
            google_name: Default::default(),
            apple_name: Default::default(),
            sony_name: Default::default(),
            tap_name: Default::default(),
            country: "US".to_string(),
            reactivate_ticket: Default::default(),
            area_code: "**".to_string(),
            device_grant_ticket: Default::default(),
        }
    }
}
