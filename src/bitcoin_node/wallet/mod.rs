use crate::bitcoin_node::config::BitcoinConfig;
use crate::bitcoin_node::BitcoinClient;
use bitcoincore_rpc::bitcoincore_rpc_json::{
    GetBlockchainInfoResult, GetMempoolInfoResult, GetMiningInfoResult, GetNetworkInfoResult,
};
use bitcoincore_rpc::{Auth, RpcApi};
use dotenv::dotenv;

pub mod chain_info;
mod default;
pub mod test;
pub mod utils;

pub struct BitcoinWallet {
    // wallet name
    pub name: String,
    rpc: bitcoincore_rpc::Client,
}

impl BitcoinWallet {
    pub fn new_wallet_client(wallet_name: &str) -> anyhow::Result<Self> {
        let url = format!(
            "{}{}{}",
            BitcoinConfig::bitcoin_network(),
            "/wallet/",
            wallet_name
        );
        let rpc = bitcoincore_rpc::Client::new(&url, BitcoinConfig::bitcoin_auth())?;

        let wallet = Self {
            name: wallet_name.to_string(),
            rpc,
        };

        Ok(wallet)
    }

    // load or create wallet
    pub(crate) fn load_or_create_wallet(
        rpc: &bitcoincore_rpc::Client,
        wallet_name: &str,
    ) -> anyhow::Result<()> {
        let data_dir = std::env::var("DATADIR").map_err(|e| {
            anyhow::anyhow!("No DATADIR found in .env file, error: {}", e.to_string())
        })?;
        let wallet_path = format!("{}{}{}", data_dir, "/wallets/", wallet_name);

        let path = std::path::Path::new(&wallet_path);
        if path.exists() && path.is_dir() {
            println!("wallet exists, load it , {:?}", path.to_path_buf());
            rpc.unload_wallet(Some(wallet_name)).map_err(|e| {
                anyhow::anyhow!(
                    "Fail to unload_wallet: {}, error: {}",
                    wallet_name,
                    e.to_string()
                )
            })?;
            rpc.load_wallet(wallet_name).map_err(|e| {
                anyhow::anyhow!(
                    "Fail to load_wallet: {}, error: {}",
                    wallet_name,
                    e.to_string()
                )
            })?;
        } else {
            println!("wallet isn't exists, create it , {:?}", path.to_path_buf());
            rpc.create_wallet(wallet_name, None, None, None, Some(false))
                .map_err(|e| {
                    anyhow::anyhow!(
                        "Fail to create_wallet: {}, error: {}",
                        wallet_name,
                        e.to_string()
                    )
                })?;
        }
        Ok(())
    }
    pub fn rpc_as_ref(&self) -> anyhow::Result<&bitcoincore_rpc::Client> {
        Ok(&self.rpc)
    }
}
