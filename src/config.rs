use jsonrpc_core::serde_json;
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    #[serde(default="default_chain_id")]
    pub chain_id: String,
    #[serde(default="default_host_client")]
    pub host_client: Client,
    #[serde(default="default_clients")]
    pub clients: Vec<Client>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Client {
    pub id: String,
}

pub fn load<P: AsRef<Path>>(path: P) -> Config {
    let config = fs::read_to_string(&path)
        .expect(&format!("Could not find file: {}", path.as_ref().display()));
    serde_json::from_str(&config).expect("Could not parse config file")
}

pub fn default() -> Config {
    Config {
        chain_id: String::from("tendermock"),
        host_client: default_host_client(),
        clients: default_clients(),
    }
}

fn default_chain_id() -> String {
    String::from("tendermock")
}

fn default_host_client() -> Client {
    Client {
        id: String::from("donald_duck"),
    }
}

fn default_clients() -> Vec<Client> {
    vec![]
}
