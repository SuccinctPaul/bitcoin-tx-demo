use crate::bitcoin_node::wallet::BitcoinWallet;
use crate::bitcoin_node::BitcoinClient;
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
