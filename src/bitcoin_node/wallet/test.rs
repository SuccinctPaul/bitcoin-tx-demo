//! Reference:
//!     https://developer.bitcoin.org/reference/rpc/index.html
//!     https://github.com/rust-bitcoin/rust-bitcoincore-rpc/tree/master/integration_test
use crate::assert_error_message;
use crate::bitcoin_node::wallet::utils::btc;
use crate::bitcoin_node::wallet::BitcoinWallet;
use bitcoin::{Address, CompressedPublicKey};
use bitcoincore_rpc::jsonrpc::error::Error as JsonRpcError;
use bitcoincore_rpc::{json, Auth, Error, RpcApi};

pub fn default_wallet() -> anyhow::Result<BitcoinWallet> {
    let wallet_name = "test_wallet_default";

    let wallet = BitcoinWallet::new_wallet_client(wallet_name)?;

    // auto load or create wallet
    BitcoinWallet::load_or_create_wallet(&wallet.rpc, &wallet.name)?;

    Ok(wallet)
}

#[test]
fn test_load_or_create_wallet_wallet_rpc() -> anyhow::Result<()> {
    default_wallet()?;

    Ok(())
}

#[test]
fn test_block_info() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;

    let init_block_num = rpc.get_blockchain_info()?.blocks;

    let block_num = rpc.get_block_count()?;
    assert_eq!(block_num, init_block_num);

    Ok(())
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

    let address = rpc.list_received_by_address(None, None, None, None)?;
    println!("list_received_by_address: {:?}", address);

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

#[test]
fn test_generate_to_address() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;

    let addr = rpc
        .get_new_address(None, Some(json::AddressType::Legacy))?
        .assume_checked();
    let initial_amt = rpc.get_balance(None, None)?;
    println!("initial_amt: {}", initial_amt);

    let init_block_num = rpc.get_blockchain_info()?.blocks;
    println!("init_block_num: {}", init_block_num);

    let blocks = rpc.generate_to_address(4, &addr)?;
    assert_eq!(blocks.len(), 4);
    let block_num = rpc.get_blockchain_info()?.blocks;
    assert_eq!(init_block_num + 4, block_num);

    let amt = rpc.get_balance(None, None)?;
    // TODO: regtest won't mine btc until 1000 blocks.
    // assert_ne!(initial_amt,amt );
    Ok(())
}

#[test]
#[ignore]
fn test_send_to_address() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;

    let addr = rpc.get_new_address(None, None)?.assume_checked();
    let est = json::EstimateMode::Conservative;
    let _ = rpc.send_to_address(&addr, btc(1), Some("cc"), None, None, None, None, None)?;
    let _ = rpc.send_to_address(&addr, btc(1), None, Some("tt"), None, None, None, None)?;
    let _ = rpc.send_to_address(&addr, btc(1), None, None, Some(true), None, None, None)?;
    let _ = rpc.send_to_address(&addr, btc(1), None, None, None, Some(true), None, None)?;
    let _ = rpc.send_to_address(&addr, btc(1), None, None, None, None, Some(3), None)?;
    let _ = rpc.send_to_address(&addr, btc(1), None, None, None, None, None, Some(est))?;
    Ok(())
}

#[test]
#[ignore]
fn test_list_unspent() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;

    let addr = rpc.get_new_address(None, None)?;
    let addr_checked = addr.clone().assume_checked();

    let options = json::ListUnspentQueryOptions {
        minimum_amount: Some(btc(7)),
        maximum_amount: Some(btc(7)),
        ..Default::default()
    };
    let unspent = rpc.list_unspent(Some(0), None, Some(&[&addr_checked]), None, Some(options))?;
    println!("unspent_utxo: {:?}", unspent);
    Ok(())
}

#[test]
#[ignore]
fn test_get_tx() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;

    // get tx
    // get tx_out
    Ok(())
}

#[test]
#[ignore]
fn test_lock_and_unlock_spent() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;

    // get tx
    // get tx_out
    // assert!(rpc.lock_unspent(&[OutPoint::new(txid, 0)]).unwrap());
    // assert!(rpc.unlock_unspent(&[OutPoint::new(txid, 0)]).unwrap());

    Ok(())
}
#[test]
fn test_generate() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;

    println!("version: {:?}", wallet.version());
    // Bitcoin Core v0.21 appears to return this with a generic -1 error code,
    // rather than the expected -32601 code (RPC_METHOD_NOT_FOUND).
    // Bitcoin-v28.0
    assert_error_message!(
        rpc.generate(4, None),
        -1,
        "replaced by the -generate cli option"
    );

    Ok(())
}

#[test]
#[ignore]
fn test_dump_private_key() -> anyhow::Result<()> {
    let wallet = default_wallet()?;
    let rpc = wallet.rpc_as_ref()?;
    let sep: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();

    let addr = rpc
        .get_new_address(None, Some(json::AddressType::Legacy))?
        .assume_checked();
    let sk = rpc.dump_private_key(&addr)?;
    // called `Result::unwrap()` on an `Err` value: JsonRpc(Rpc(RpcError { code: -4, message: "Only legacy wallets are supported by this command", data: None }))
    let pk = CompressedPublicKey::from_private_key(&sep, &sk)?;

    assert_eq!(addr, Address::p2wpkh(&pk, wallet.chain_info()?.chain));
    Ok(())
}
