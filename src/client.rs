use esplora_client::{AsyncClient, Builder};
use once_cell::sync::Lazy;

const ESPLORA_URL: &str = "https://mutinynet.com/api";

pub const CLIENT: Lazy<AsyncClient> = Lazy::new(|| {
    let builder = Builder::new(crate::client::ESPLORA_URL);

    let client = builder.build_async().unwrap();
    client
});

#[cfg(test)]
mod test {
    use crate::client::CLIENT;
    use bitcoin::{Address, Network, OutPoint, ScriptHash, Transaction, TxIn, TxOut, Txid};
    use std::str::FromStr;

    #[tokio::test]
    async fn test_get_block_height() {
        // let builder = Builder::new(crate::client::ESPLORA_URL);
        // let client = builder.build_async().unwrap();

        let hight = CLIENT.get_height().await.unwrap();
        println!("current block height: {:?}", hight);
    }

    #[tokio::test]
    async fn test_get_tx_status() {
        let txid =
            Txid::from_str("c6756eaebb68c09ed66911438b1639529b18556f717498bb8fbb070802fa9ef0,")
                .unwrap();
        let status = CLIENT.get_tx_status(&txid).await.unwrap();
        println!("current tx is_confirmed: {:?}", status.confirmed);
    }

    #[tokio::test]
    async fn test_get_address_unspend_utxo() {
        // let addr = Address::from_str("bc1p0dq0tzg2r780hldthn5mrznmpxsxc0jux5f20fwj0z3wqxxk6fpqm7q0va")
        //     .expect("a valid address")
        //     .require_network(Network::Testnet)
        //     .expect("valid address for mainnet");
        let addr = "tb1ql9mjwcp9swms3hm6kyvp832myv4ujmqcpmn7az";

        let utxo = CLIENT.get_address_utxo(addr).await.unwrap();

        println!("utxo: {:?}", utxo.len());
    }

    #[tokio::test]
    async fn test_address() {
        let addr = Address::from_str("tb1ql9mjwcp9swms3hm6kyvp832myv4ujmqcpmn7az").unwrap();
    }
}
