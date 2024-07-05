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
    use bitcoin::{OutPoint, Transaction, TxIn, TxOut, Txid};
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
            Txid::from_str("b1bd786dc982c5f2febf75e86c725967e0afb13e38ea2515194a8fc1615646b3")
                .unwrap();
        let status = CLIENT.get_tx_status(&txid).await.unwrap();
        println!("current tx is_confirmed: {:?}", status.confirmed);
    }

    #[test]
    fn test_pre_output() {
        let addr = "tb1ql9mjwcp9swms3hm6kyvp832myv4ujmqcpmn7az";
        // let balance = CLIENT.
        let previous_output = OutPoint::new(
            Txid::from_str("b1bd786dc982c5f2febf75e86c725967e0afb13e38ea2515194a8fc1615646b3")
                .unwrap(),
            0,
        );
        println!("{:?}", previous_output);
    }

    // #[tokio::test]
    // async fn test_create_trx() {
    //     let addr = "tb1ql9mjwcp9swms3hm6kyvp832myv4ujmqcpmn7az";
    //     let addr_2 = "tb1ql8mjwcp9swms3hm6kyvp832myv2ujmqcpmn7az";
    //
    //     let previous_output = OutPoint::new(
    //         Txid::from_hex("b1bd786dc982c5f2febf75e86c725967e0afb13e38ea2515194a8fc1615646b3")
    //             .unwrap(),
    //         0,
    //     );
    //
    //     let tx_input = TxIn {
    //         previous_output,
    //         script_sig: Default::default(),
    //         sequence: 0xffffffff,
    //         witness: Default::default(),
    //     };
    //
    //     let tx_output = TxOut {
    //         value: 5000,
    //         script_pubkey: output_details.script_pubkey(),
    //     };
    //
    //     let mut transaction = Transaction {
    //         version: 2,
    //         lock_time: 0,
    //         input: vec![tx_input],
    //         output: vec![tx_output],
    //     };
    //     let res = CLIENT.broadcast(&tx).unwrap();
    // }
}
