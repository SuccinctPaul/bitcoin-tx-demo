use crate::bitcoin_node::config::BitcoinConfig;

pub mod config;
pub mod regtest;
mod test;
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
}
