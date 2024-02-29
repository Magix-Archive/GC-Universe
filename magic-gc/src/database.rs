use std::sync::Mutex;
use log::info;
use mongodb::bson::{doc, Document};
use mongodb::{Client, Collection};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
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

/// Generic MongoDB find document function.
/// query: The query to pass to MongoDB to find the document.
/// collection: The collection to find the document in.
async fn find<T>(query: Document, collection: &str) -> Option<T>
    where T: DeserializeOwned + Unpin + Send + Sync {
    let client = DATABASE_CLIENT.lock()
        .unwrap().as_ref().unwrap().clone();

    let db = client.database("grasscutter");
    let collection: Collection<T> = db.collection(collection);

    collection.find_one(query, None)
        .await.expect("Failed to find object")
}

/// Generic MongoDB update document function.
/// query: The query to pass to MongoDB to find the document.
/// value: The value to update the document with.
/// collection: The collection to update the document in.
async fn update<T>(value: T, collection: &str)
    where T: Serialize + Unpin + Send + Sync {
    let client = DATABASE_CLIENT.lock()
        .unwrap().as_ref().unwrap().clone();

    let db = client.database("grasscutter");
    let collection: Collection<T> = db.collection(collection);

    collection.insert_one(value, None)
        .await.expect("Failed to update object");
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
        find(document, "accounts").await
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
        update(self, "accounts").await;
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Counter {
    #[serde(rename = "_id")]
    pub id: String,
    pub count: i32
}

impl Counter {
    /// Generic find function.
    /// document: The document to find.
    pub async fn find(document: Document) -> Option<Counter> {
        find(document, "counters").await
    }

    /// Gets and increments the next ID for the given counter.
    /// counter_name: The name of the counter to get the next ID for.
    pub async fn next_id(counter_name: &str, default: i32) -> i32 {
        // Check if the counter exists.
        let counter: Option<Counter> = Counter::find(
            doc! { "_id": counter_name }).await;
        let mut counter = match counter {
            Some(counter) => counter,
            None => {
                let counter = Counter {
                    id: counter_name.to_string(),
                    count: default
                };
                update(counter, "counters").await;
                return default;
            }
        };

        // Increment the counter.
        let prev_count = counter.count.clone();
        counter.count += 1;
        update(counter, "counters").await;

        prev_count
    }
}
