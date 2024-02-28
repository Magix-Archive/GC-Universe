use serde::Deserialize;

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Database {
    pub mongo_uri: String
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Server {
    pub host: String,
    pub port: u16
}

#[derive(Clone, Default, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Options {
    pub database: Database,
    pub server: Server
}
