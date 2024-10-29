use bitcoincore_rpc::Auth;
use dotenv::dotenv;

pub mod test;

pub struct BitcoinClient;

impl BitcoinClient {
    pub fn init() -> anyhow::Result<bitcoincore_rpc::Client> {
        let client = bitcoincore_rpc::Client::new(&Self::get_rpc(), Self::get_auth())?;
        Ok(client)
    }

    fn new_wallet_client(wallet_name: &str) -> anyhow::Result<bitcoincore_rpc::Client> {
        let url = format!("{}{}{}", Self::get_rpc(), "/wallet/", wallet_name);
        let client = bitcoincore_rpc::Client::new(&url, Self::get_auth())?;
        Ok(client)
    }

    pub fn get_rpc() -> String {
        dotenv().ok();

        let network = std::env::var("BITCOIN_NETWORK");
        if network.is_ok() {
            network.unwrap()
        } else {
            panic!("No network found in .env file");
        }
    }

    pub fn get_auth() -> Auth {
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
