use bitcoin::Network;
use bitcoincore_rpc::{json, RpcApi};

// Generate 101 blocks using a special RPC which is only available in regtest mode.
// This takes less than a second on a generic PC.
// Because this is a new block chain using Bitcoin’s default rules, the first blocks pay a block reward of 50 bitcoins.
//
// Unlike mainnet, in regtest mode only the first 150 blocks pay a reward of 50 bitcoins.
// However, a block must have 100 confirmations before that reward can be spent,
// so we generate 101 blocks to get access to the coinbase transaction from block #1.
//
// Reference:
//      https://developer.bitcoin.org/examples/testing.html#regtest-mode
pub(crate) fn genesis_101(rpc: &bitcoincore_rpc::Client) -> anyhow::Result<()> {
    const GENESIS_BLOCKS: u64 = 101;
    let mut chain_info = rpc.get_blockchain_info()?;
    if chain_info.chain != Network::Regtest {
        println!("Not the Regtest, skip");
        return Ok(());
    }
    if chain_info.blocks >= GENESIS_BLOCKS {
        println!(
            "current blocks {} >= {}, skip",
            chain_info.blocks, GENESIS_BLOCKS
        );
        return Ok(());
    }

    // generate 100 blocks.
    let addr = rpc
        .get_new_address(None, Some(json::AddressType::Legacy))?
        .assume_checked();
    let start_amt = rpc.get_balance(None, None)?;
    let blocks = rpc.generate_to_address(GENESIS_BLOCKS, &addr)?;
    assert_eq!(blocks.len() as u64, GENESIS_BLOCKS);
    let end_amt = rpc.get_balance(None, None)?;
    println!(
        "genesis_101: start_amt: {}, end_amt: {}",
        start_amt, end_amt
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bitcoin_node::BitcoinClient;

    #[test]
    fn test_genesis_101() -> anyhow::Result<()> {
        let rpc = BitcoinClient::init_client()?;
        genesis_101(&rpc)?;

        Ok(())
    }
}
