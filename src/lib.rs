// extern crate bitcoin;

mod client;
mod faucet;
mod tx;

#[cfg(test)]
mod test {
    use bitcoin::hashes::hash160::Hash;
    use bitcoin::{network::Network, OutPoint, Script, Transaction, TxIn, TxOut};

    #[test]
    fn test_tx() {}
}
