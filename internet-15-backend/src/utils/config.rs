//core storage
use std::env;

pub struct Config {
    pub storage_path: String,
    pub node_address: String,
    pub blockchain_rpc: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        Self {
            storage_path: env::var("STORAGE_PATH").unwrap_or("./data".to_string()),
            node_address: env::var("NODE_ADDRESS").unwrap_or("127.0.0.1:4000".to_string()),
            blockchain_rpc: env::var("BLOCKCHAIN_RPC").unwrap_or("https://api.mainnet-beta.solana.com".to_string()),
        }
    }
}
