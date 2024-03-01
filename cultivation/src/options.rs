use serde::{Deserialize, Serialize};
use crate::{str, utils};

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Server {
    pub host: String,
    pub port: u16
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase", deserialize = "PascalCase"))]
pub struct Proxy {
    pub host: String,
    pub port: u16,
    pub cert_path: String,
    pub urls: Vec<String>
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Game {
    pub path: String,
    pub proxy: Server
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct Options {
    pub genshin: Game,
    pub starrail: Game,
    pub proxy: Proxy,
}

impl Options {
    /// Fetches the game configuration options by name.
    /// game: The name of the game.
    pub fn game_from_name(&self, game: &str) -> Game {
        match game {
            "genshin" => self.genshin.clone(),
            "starrail" => self.starrail.clone(),
            _ => panic!("Invalid game provided.")
        }
    }

    /// Sets the game path for a game.
    /// game: The name of the game.
    /// path: The path to the game's executable.
    pub fn set_game_path(&self, game: &str, path: String) {
        let options = self.clone();
        let mut game = options.game_from_name(game);

        game.path = path;

        utils::write_json("config.json", options).unwrap();
    }

    /// Sets the server data for a game.
    /// game: The name of the game.
    /// host: The server's address.
    /// port: The server's port.
    pub fn set_server(&self, game: &str, host: String, port: u16) {
        let options = self.clone();
        let mut game = options.game_from_name(game);

        game.proxy.host = host;
        game.proxy.port = port;

        utils::write_json("config.json", options).unwrap();
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            genshin: Game {
                path: str!(r#"C:\Program Files\Genshin Impact\Genshin Impact game\GenshinImpact.exe"#),
                proxy: Server {
                    host: str!("127.0.0.1"),
                    port: 8080
                }
            },
            starrail: Game {
                path: str!(r#"C:\Program Files\Honkai Star Rail\Star Rail game\GenshinImpact.exe"#),
                proxy: Server {
                    host: str!("127.0.0.1"),
                    port: 8080
                }
            },
            proxy: Proxy {
                host: str!("127.0.0.1"),
                port: 2024,
                cert_path: str!("certs"),
                urls: vec!(
                    str!("hoyoverse.com"),
                    str!("mihoyo.com"),
                    str!("yuanshen.com"),
                    str!("starrails.com"),
                    str!("bhsr.com"),
                    str!("bh3.com"),
                    str!("honkaiimpact3.com"),
                    str!("zenlesszonezero.com")
                )
            }
        }
    }
}
