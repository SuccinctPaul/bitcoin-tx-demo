//! Reference:
//!     https://developer.bitcoin.org/reference/rpc/index.html
//!     https://github.com/rust-bitcoin/rust-bitcoincore-rpc/tree/master/integration_test
use crate::bitcoin_node::BitcoinWallet;
use bitcoincore_rpc::{json, RpcApi};

#[test]
fn test_blockchains_rpc() -> anyhow::Result<()> {
    let rpc = BitcoinWallet::init_client()?;
    let best_block_hash = rpc.get_best_block_hash()?;
    println!("best block hash: {}", best_block_hash);
    let chaininfo = rpc.get_blockchain_info()?;
    println!("block_info: {:#?}", chaininfo);

    let network = rpc.get_network_info()?;
    println!("network: {:?}", network);
    println!("network.version: {:?}", network.version);
    println!("network.subversion: {:?}", network.subversion);
    println!("network.networks: {:?}", network.networks);

    Ok(())
}

#[test]
fn test_load_or_create_wallet_wallet_rpc() -> anyhow::Result<()> {
    default_wallet()?;

    Ok(())
}

pub fn default_wallet() -> anyhow::Result<BitcoinWallet> {
    let wallet_name = "test_wallet_default";

    let wallet = BitcoinWallet::new_wallet_client(wallet_name)?;

    Ok(wallet)
}

#[test]
fn test_get_new_address() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;

    let address = rpc.get_new_address(None, Some(json::AddressType::Legacy))?;
    println!("Legacy address: {:?}", address);
    let addr = address.assume_checked();
    assert_eq!(addr.address_type(), Some(bitcoin::AddressType::P2pkh));
    println!("Legacy address: {}", addr.to_string());
    println!("Legacy address: {}", addr.script_pubkey());
    println!("Legacy address: {}", addr.script_pubkey().as_script());

    let address = rpc.get_new_address(None, Some(json::AddressType::Bech32))?;
    println!("Bech32 address: {:?}", address);
    let addr = address.assume_checked();
    assert_eq!(addr.address_type(), Some(bitcoin::AddressType::P2wpkh));
    println!("Bech32 address: {}", addr.to_string());
    println!("Bech32 address: {}", addr.script_pubkey());
    println!("Bech32 address: {}", addr.script_pubkey().as_script());
    println!("");

    let address = rpc.get_new_address(None, Some(json::AddressType::Bech32m))?;
    println!("Bech32m address: {:?}", address);
    let addr = address.assume_checked();
    println!("Bech32m address: {}", addr.to_string());
    println!("Bech32m address: {}", addr.script_pubkey());
    println!("Bech32m address: {}", addr.script_pubkey().as_script());
    println!("");

    let address = rpc.get_new_address(None, Some(json::AddressType::P2shSegwit))?;
    println!("P2shSegwit address: {:?}", address);
    let addr = address.assume_checked();
    assert_eq!(addr.address_type(), Some(bitcoin::AddressType::P2sh));
    println!("P2shSegwit address: {}", addr.to_string());
    println!("P2shSegwit address: {}", addr.script_pubkey());
    println!("P2shSegwit address: {}", addr.script_pubkey().as_script());
    println!("");

    Ok(())
}

#[test]
fn test_get_raw_change_address() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;

    let addr = rpc
        .get_raw_change_address(Some(json::AddressType::Legacy))?
        .assume_checked();
    assert_eq!(addr.address_type(), Some(bitcoin::AddressType::P2pkh));

    let addr = rpc
        .get_raw_change_address(Some(json::AddressType::Bech32))?
        .assume_checked();
    assert_eq!(addr.address_type(), Some(bitcoin::AddressType::P2wpkh));

    let addr = rpc
        .get_raw_change_address(Some(json::AddressType::P2shSegwit))?
        .assume_checked();
    assert_eq!(addr.address_type(), Some(bitcoin::AddressType::P2sh));

    Ok(())
}
