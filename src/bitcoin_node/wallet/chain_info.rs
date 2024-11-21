use crate::bitcoin_node::wallet::BitcoinWallet;
use bitcoincore_rpc::bitcoincore_rpc_json::{
    GetBlockchainInfoResult, GetMempoolInfoResult, GetMiningInfoResult, GetNetworkInfoResult,
};
use bitcoincore_rpc::{Auth, RpcApi};

impl BitcoinWallet {
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
}
