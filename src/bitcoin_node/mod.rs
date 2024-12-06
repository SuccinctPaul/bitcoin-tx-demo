use crate::bitcoin_node::config::BitcoinConfig;
use bitcoin::PrivateKey;

pub mod account;
pub mod config;
pub mod regtest;
mod test;
mod tx;
pub mod wallet;

pub struct BitcoinClient;

impl BitcoinClient {
    pub fn init_client() -> anyhow::Result<bitcoincore_rpc::Client> {
        let client = bitcoincore_rpc::Client::new(
            &BitcoinConfig::bitcoin_network(),
            BitcoinConfig::bitcoin_auth(),
        )?;
        Ok(client)
    }

    pub fn init_client_with_url(url: &str) -> anyhow::Result<bitcoincore_rpc::Client> {
        let client = bitcoincore_rpc::Client::new(url, BitcoinConfig::bitcoin_auth())?;
        Ok(client)
    }
}
