use crate::bitcoin_node::tx::taproot_tree_tx::{create_p2tr_address, create_taproot_tree};
use crate::bitcoin_node::tx::{
    dummy_unspent_transaction_output, senders_keys, USER_A_PRIVATE_KEY, USER_B_PRIVATE_KEY,
    USER_B_PUBLIC_KEY, USER_C_PUBLIC_KEY,
};
use crate::keygen::Keygen;
use bitcoin::consensus::encode;
use bitcoin::key::{TapTweak, TweakedKeypair};
use bitcoin::sighash::{Prevouts, SighashCache};
use bitcoin::{
    absolute, transaction, Amount, Network, PublicKey, ScriptBuf, Sequence, TapSighashType,
    Transaction, TxIn, TxOut, Witness,
};
use secp256k1::{Message, Secp256k1};

#[test]
fn test_a_to_taproot_tree_addr() -> anyhow::Result<()> {
    // bitcoin-cli -regtest -rpcwallet=benefactor listunspent 99 199 '["bcrt1phcnl4zcl2fu047pv4wx6y058v8u0n02at6lthvm7pcf2wrvjm5tqatn90k"]'
    let pre_txid = "60508d60a95fad286f36b75d7484a7493291254ab720f1606d78ae005913042b";
    let pre_vout = 0;
    let amount_in_sats = Amount::from_btc(25.0).unwrap();

    let secp = Secp256k1::new();
    // Get a keypair we control. In a real application these would come from a stored secret.
    let keypair = senders_keys(&secp, USER_A_PRIVATE_KEY);
    let (internal_key, _parity) = keypair.x_only_public_key();
    // let sender_address = Keygen::p2tr_addr_from_pk(*keypair.public_key(), Network::Regtest)?;

    // Get an address to send to.
    let tree = create_taproot_tree(&secp);
    let reciever_address = create_p2tr_address(tree);

    // Get an unspent output that is locked to the key above that we control.
    // In a real application these would come from the chain.
    let (dummy_out_point, dummy_utxo) =
        dummy_unspent_transaction_output(&secp, internal_key, pre_txid, pre_vout, amount_in_sats);

    // The input for the transaction we are constructing.
    let input = TxIn {
        previous_output: dummy_out_point, // The dummy output we are spending.
        script_sig: ScriptBuf::default(), // For a p2tr script_sig is empty.
        sequence: Sequence::MAX,
        witness: Witness::default(), // Filled in after signing.
    };

    // The spend output is locked to a key controlled by the receiver.
    let spend = TxOut {
        value: crate::bitcoin_node::tx::sign_tx_taproot::SPEND_AMOUNT,
        script_pubkey: reciever_address.script_pubkey(),
    };

    // The change output is locked to a key controlled by us.
    let change = TxOut {
        value: dummy_utxo
            .value
            .unchecked_sub(crate::bitcoin_node::tx::sign_tx_taproot::SPEND_AMOUNT)
            .unchecked_sub(crate::bitcoin_node::tx::sign_tx_taproot::GAS_FEE),
        script_pubkey: ScriptBuf::new_p2tr(&secp, internal_key, None), // Change comes back to us.
    };

    // The transaction we want to sign and broadcast.
    let mut unsigned_tx = Transaction {
        version: transaction::Version::TWO,  // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![input],                  // Input goes into index 0.
        output: vec![spend, change],         // Outputs, order does not matter.
    };
    let input_index = 0;

    // Get the sighash to sign.

    let sighash_type = TapSighashType::Default;
    let prevouts = vec![dummy_utxo];
    let prevouts = Prevouts::All(&prevouts);

    let mut sighasher = SighashCache::new(&mut unsigned_tx);
    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    // Sign the sighash using the secp256k1 library (exported by rust-bitcoin).
    let tweaked: TweakedKeypair = keypair.tap_tweak(&secp, None);
    let msg = Message::from(sighash);
    let signature = secp.sign_schnorr(&msg, &tweaked.to_inner());

    // Update the witness stack.
    let signature = bitcoin::taproot::Signature {
        signature,
        sighash_type,
    };
    *sighasher.witness_mut(input_index).unwrap() = Witness::p2tr_key_spend(&signature);

    // Get the signed transaction.
    let tx = sighasher.into_transaction();
    let txid = tx.compute_txid();
    let tx_hex_str = encode::serialize_hex(&tx);

    // BOOM! Transaction signed and ready to broadcast.
    println!("tx_id {:?}", txid);
    println!("tx_str {:?}", tx_hex_str);
    // tx_id 7ed4eb181f0ceb3ca00d01fac6945127adac960127ce3838f9b94a9efa67aa79
    // tx_str "020000000001012b04135900ae786d60f120b74a25913249a784745db7366f28ad5fa9608d50600000000000ffffffff02404b4c00000000002251201659069a74086cedae03e369ec4cd5e5b46c1c848941ccf1cacf8db6d9e39120d8a9b69400000000225120be27fa8b1f5278faf82cab8da23e8761f8f9bd5d5ebebbb37e0e12a70d92dd160140e2ff1536fc27776fad7a9208fe0097a7c4b23a399624bfaec82419f67cb2fb45ebcb6930e0099d645a1424dc5675977bde44182d882f21dff817b0790146d6d700000000"

    Ok(())
}
