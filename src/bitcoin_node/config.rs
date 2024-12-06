use crate::bitcoin_node::wallet::BitcoinWallet;
use crate::bitcoin_node::BitcoinClient;
use bitcoincore_rpc::Auth;
use dotenv::dotenv;

pub struct BitcoinConfig;

impl BitcoinConfig {
    pub fn bitcoin_network() -> String {
        dotenv().ok();
        let network = std::env::var("BITCOIN_NETWORK");
        if network.is_ok() {
            network.unwrap()
        } else {
            panic!("No network found in .env file");
        }
    }

    pub fn bitcoin_auth() -> Auth {
        dotenv().ok();

        let username = std::env::var("BITCOIN_USERNAME");
        let pswd = std::env::var("BITCOIN_PASSWORD");

        if username.is_ok() && pswd.is_ok() {
            Auth::UserPass(username.unwrap(), pswd.unwrap())
        } else {
            println!("No username or password found in .env file");
            Auth::None
        }
    }
}
