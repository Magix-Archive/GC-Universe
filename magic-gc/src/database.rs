use std::sync::Mutex;
use log::info;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use crate::options::Options;
use crate::structs::LoginData;

static DATABASE_CLIENT: Mutex<Option<Client>> = Mutex::new(None);

/// Creates a MongoDB client from the given options.
pub async fn setup_client(options: Options) {
    let client = Client::with_uri_str(&options.database.mongo_uri)
        .await.expect("Failed to create MongoDB client.");
    DATABASE_CLIENT.lock().unwrap().replace(client);

    info!("Connected to MongoDB.");
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    #[serde(rename = "_id")]
    pub id: String,

    pub email: String,
    pub username: String,

    // Passwords are currently un-used by Grasscutter.
    pub password: String,

    pub login_token: String, // The login token is used for account authentication.
    pub session_token: String, // The session token is used for game authentication.
}

impl Account {
    /// Generic find function.
    /// document: The document to find.
    pub async fn find(document: Document) -> Option<Account> {
        let client = DATABASE_CLIENT.lock()
            .unwrap().as_ref().unwrap().clone();

        let db = client.database("grasscutter");
        let collection = db.collection("accounts");

        collection.find_one(document, None)
            .await.expect("Failed to find object")
    }

    /// Generates a random ID for an account.
    /// The ID returned is unique.
    pub async fn generate_id() -> String {
        let client = DATABASE_CLIENT.lock()
            .unwrap().as_ref().unwrap().clone();

        let db = client.database("grasscutter");
        let collection: Collection<Account> = db.collection("accounts");

        let mut id: String;
        loop {
            id = crate::utils::random_string(32);
            if collection.find_one(doc! { "_id": &id }, None)
                .await.expect("Failed to find object").is_none() {
                break;
            }
        }

        id
    }

    /// Finds an account by its ID.
    /// id: The ID of the account to find.
    pub async fn find_id(id: &str) -> Option<Account> {
        Account::find(doc! { "_id": id }).await
    }

    /// Finds an account by its username.
    /// username: The username of the account to find.
    pub async fn find_username(username: &str) -> Option<Account> {
        Account::find(doc! { "username": username }).await
    }

    /// Finds an account by its email.
    /// email: The email of the account to find.
    pub async fn find_email(email: &str) -> Option<Account> {
        Account::find(doc! { "email": email }).await
    }

    /// Finds an account by its login token.
    /// login_token: The login token of the account to find.
    pub async fn find_login_token(login_token: &str) -> Option<Account> {
        Account::find(doc! { "loginToken": login_token }).await
    }

    /// Finds an account by its session token.
    /// session_token: The session token of the account to find.
    pub async fn find_session_token(session_token: &str) -> Option<Account> {
        Account::find(doc! { "sessionToken": session_token }).await
    }

    /// Creates a login data object from the account.
    /// This is used to send account information to the client.
    pub fn login_data(&self) -> LoginData {
        let mut data = LoginData::default();
        data.uid = Some(self.id.clone());
        data.token = Some(self.login_token.clone());
        data.email = self.email.clone();

        data
    }

    /// Creates the account in the database.
    /// This function is used to create accounts.
    /// This will not update an existing account.
    pub async fn save(&self) {
        let client = DATABASE_CLIENT.lock()
            .unwrap().as_ref().unwrap().clone();

        let db = client.database("grasscutter");
        let collection: Collection<Account> = db.collection("accounts");

        collection.insert_one(self, None)
            .await.expect("Failed to insert object");
    }
}
