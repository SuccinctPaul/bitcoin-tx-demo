use crate::bitcoin_node::wallet::BitcoinWallet;
use crate::bitcoin_node::BitcoinClient;
use bitcoin::consensus::encode;
use bitcoin::Transaction;
use bitcoincore_rpc::RpcApi;

#[test]
fn test_blockchains_rpc() -> anyhow::Result<()> {
    let rpc = BitcoinClient::init_client()?;
    let best_block_hash = rpc.get_best_block_hash()?;
    println!("best block hash: {}", best_block_hash);
    let chaininfo = rpc.get_blockchain_info()?;
    println!("block_info: {:#?}", chaininfo);

    let network = rpc.get_network_info()?;
    println!("network: {:?}", network);
    println!("network.version: {:?}", network.version);
    println!("network.subversion: {:?}", network.subversion);
    println!("network.networks: {:?}", network.networks);

    let mempool = rpc.get_raw_mempool()?;
    println!("mempool: {:?}", mempool);

    Ok(())
}

#[test]
fn compute_tx_id() {
    let tx_str= "020000000001012fbbc9da1c98a0483c229c49a957dbe65d79fd2a750e5de12f51625b4a58d27f0100000000ffffffff02102700000000000022002085f1940c71a1e1a852db646fa0f79cf1e5defc9e4bda671ad4cf9000ada74b412fb2080000000000225120e1382c1cb56e91bc45683199f550261b4a2da8a6db7454f3e236a4e3dfba890c0140bb31038df9af1eeb3ef348c685fa71ca6270c171178bbd3886dfea948e0855617b6fba6dcfdfea8b177d0083e8b2da55ffc3a8ff513585d44e9653fcdd85db7e00000000";

    let mut tx = encode::deserialize_hex::<Transaction>(tx_str).unwrap();
    let tx_id = tx.compute_txid();
    println!("tx_id: {:?}", tx_id.to_string());
}
