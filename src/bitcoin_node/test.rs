//! Reference:
//!     https://developer.bitcoin.org/reference/rpc/index.html
//!     https://github.com/rust-bitcoin/rust-bitcoincore-rpc/tree/master/integration_test
use crate::bitcoin_node::BitcoinClient;
use bitcoincore_rpc::RpcApi;
use rand::{thread_rng, RngCore};

#[test]
fn test_blockchains_rpc() -> anyhow::Result<()> {
    let rpc = BitcoinClient::init()?;
    let best_block_hash = rpc.get_best_block_hash()?;
    println!("best block hash: {}", best_block_hash);

    let chaininfo = rpc.get_blockchain_info()?;
    println!("block_info: {:#?}", chaininfo);

    Ok(())
}

#[test]
fn test_wallet_rpc() -> anyhow::Result<()> {
    // random wallet name
    let mut rng = thread_rng();
    let index = rng.next_u32() % 1000;
    let wallet_name = format!("test_wallet_a_{}", index);

    let rpc = BitcoinClient::new_wallet_client(&wallet_name)?;
    // only can create once a time locally.
    let create_wallet = rpc.create_wallet(&wallet_name, None, None, None, None)?;

    let new_address = rpc.get_new_address(None, None)?;

    Ok(())
}
