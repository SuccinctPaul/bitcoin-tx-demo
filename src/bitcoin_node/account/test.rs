use crate::bitcoin_node::account::BitcoinAccount;
use crate::bitcoin_node::regtest::genesis_101;
use crate::bitcoin_node::BitcoinClient;
use crate::keygen::Keygen;
use bitcoincore_rpc::RpcApi;

#[test]
fn test_generate_to_address() -> anyhow::Result<()> {
    let rpc = BitcoinClient::init_client()?;
    let chain_info = rpc.get_blockchain_info()?;
    println!("block len: {:?}", chain_info.blocks);
    let accout = BitcoinAccount::gen(chain_info.chain)?;
    let address = Keygen::p2sh_addr_from_pk(&accout.public_key, chain_info.chain)?;

    // rpc.import_private_key(&accout.private_key, None, Some(false))?;
    // rpc.import_address(&address, None, Some(false))?;
    // rpc.import_address_script(&address.script_pubkey(), None, None, Some(false))?;
    let pre_unspent_utxos = rpc.list_unspent(None, None, Some(&vec![&address]), None, None)?;

    // generate
    genesis_101(&rpc)?;
    let blocks = rpc.generate_to_address(3, &address)?;
    println!("blocks len:{:?}", blocks.len());
    let later_unspent_utxos = rpc.list_unspent(None, None, Some(&vec![&address]), None, None)?;

    // TODO
    println!(
        "generated unspent_utxos: {:?}",
        later_unspent_utxos.len() - pre_unspent_utxos.len()
    );

    Ok(())
}
