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
    let tx_str= "020000000001017dbfa31f3f060373b5527fff7329d9fc142ff0cbaa0cf9163eda8648b96cd2ea5000000000ffffffff02bc04000000000000220020d4425a56d5c2497f19cc1d9033f31077a7a8fc8eaaf212dea3ceff0c54acddad1025000000000000225120b15a638284b2df7fbb19c5f5aa4974bf4a2032dbfc079286e9ee4ea2901c09ce014150de0c092479c76e91eff1f586a653c11ec74e62254d6449ca77a2a74d2c0b4dda30918b08f9710fcc1324593e2ad5bb54c78e7ec662948c227bd8b09d399f980100000000";

    let mut tx = encode::deserialize_hex::<Transaction>(tx_str).unwrap();
    let tx_id = tx.compute_txid();
    println!("tx_id: {:?}", tx_id.to_string());
}
