use bitcoincore_rpc::bitcoincore_rpc_json::{
    GetBlockchainInfoResult, GetMempoolInfoResult, GetMiningInfoResult, GetNetworkInfoResult,
};
use bitcoincore_rpc::{Auth, RpcApi};
use dotenv::dotenv;
use std::str::FromStr;

pub mod test;
pub mod utils;

pub struct BitcoinWallet {
    // wallet name
    pub name: String,
    rpc: bitcoincore_rpc::Client,
}

impl BitcoinWallet {
    pub fn init_client() -> anyhow::Result<bitcoincore_rpc::Client> {
        let client = bitcoincore_rpc::Client::new(&Self::get_rpc(), Self::get_auth())?;
        Ok(client)
    }

    pub fn rpc_as_ref(&self) -> anyhow::Result<&bitcoincore_rpc::Client> {
        Ok(&self.rpc)
    }

    pub fn chain_info(&self) -> anyhow::Result<GetBlockchainInfoResult> {
        Ok(self.rpc.get_blockchain_info()?)
    }

    pub fn network_info(&self) -> anyhow::Result<GetNetworkInfoResult> {
        Ok(self.rpc.get_network_info()?)
    }
    pub fn mining_info(&self) -> anyhow::Result<GetMiningInfoResult> {
        Ok(self.rpc.get_mining_info()?)
    }

    pub fn mempool_info(&self) -> anyhow::Result<GetMempoolInfoResult> {
        Ok(self.rpc.get_mempool_info()?)
    }
    pub fn version(&self) -> anyhow::Result<usize> {
        Ok(self.network_info()?.version)
    }

    fn new_wallet_client(wallet_name: &str) -> anyhow::Result<Self> {
        let url = format!("{}{}{}", Self::get_rpc(), "/wallet/", wallet_name);
        let rpc = bitcoincore_rpc::Client::new(&url, Self::get_auth())?;

        // auto load or create wallet
        Self::load_or_create_wallet(&rpc, wallet_name);

        let wallet = Self {
            name: wallet_name.to_string(),
            rpc,
        };

        Ok(wallet)
    }

    // load or create wallet
    fn load_or_create_wallet(
        rpc: &bitcoincore_rpc::Client,
        wallet_name: &str,
    ) -> anyhow::Result<()> {
        let data_dir = std::env::var("DATADIR");

        let wallet_path = if data_dir.is_ok() {
            let data_dir = data_dir?;
            let path = std::path::Path::new(&data_dir);
            // assert!(path.exists());
            format!("{}{}{}", data_dir, "/wallets/", wallet_name)
        } else {
            panic!("No DATADIR found in .env file");
        };

        let path = std::path::Path::new(&wallet_path);
        println!("path: {:?}", path.to_path_buf());
        // TODO: failed to check exists of target wallet path.
        if path.exists() {
            // if true {
            println!("wallet exists, load it , {:?}", path.to_path_buf());
            let load_wallet = rpc.load_wallet(wallet_name);
            if load_wallet.is_err() {
                panic!(
                    "Fail to load_wallet: {:#?}, error: {}",
                    wallet_name,
                    load_wallet.err().unwrap()
                );
            }
        } else {
            println!("wallet isn't exists, create it , {:?}", path.to_path_buf());
            // let create_wallet = rpc.create_wallet(wallet_name, None, None, None, Some(false));
            // if create_wallet.is_err() {
            //     panic!(
            //         "Fail to create_wallet: {:#?}, error: {}",
            //         wallet_name,
            //         create_wallet.err().unwrap()
            //     );
            // }
        }
        Ok(())
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
