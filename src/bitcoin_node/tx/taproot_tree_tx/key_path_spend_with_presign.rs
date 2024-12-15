use crate::bitcoin_node::tx::sign_tx_taproot::{GAS_FEE, SPEND_AMOUNT};
use crate::bitcoin_node::tx::taproot_tree_tx::{
    create_p2tr_address, create_taproot_tree, gen_one_of_two_multi_sig_scripts,
};
use crate::bitcoin_node::tx::{
    dummy_unspent_transaction_output, senders_keys, RECEIVER_ADDR_STR, USER_A_PRIVATE_KEY,
    USER_A_PUBLIC_KEY, USER_B_PRIVATE_KEY, USER_B_PUBLIC_KEY, USER_C_PUBLIC_KEY,
};
use anyhow::anyhow;
use bitcoin::bip32::{DerivationPath, Fingerprint, Xpriv};
use bitcoin::consensus::encode;
use bitcoin::key::{TapTweak, TweakedKeypair};
use bitcoin::psbt::{Input, PsbtSighashType};
use bitcoin::sighash::{Prevouts, SighashCache};
use bitcoin::taproot::LeafVersion;
use bitcoin::transaction::Version;
use bitcoin::Network::Regtest;
use bitcoin::{
    absolute, transaction, Address, Amount, Network, OutPoint, Psbt, PublicKey, ScriptBuf,
    Sequence, TapSighashType, Transaction, TxIn, TxOut, Witness,
};
use secp256k1::{Keypair, Message, Secp256k1};
use std::collections::BTreeMap;
use std::str::FromStr;

#[test]
fn test_a_to_taproot_tree_addr() -> anyhow::Result<()> {
    // bitcoin-cli -regtest -rpcwallet=benefactor listunspent 99 199 '["bcrt1phcnl4zcl2fu047pv4wx6y058v8u0n02at6lthvm7pcf2wrvjm5tqatn90k"]'
    let pre_txid = "8a98e00760c4101a3c6a7f7eb81d3309e8694c8bb9716c11486758b315310e6a";
    let pre_vout = 0;
    let amount_in_sats = Amount::from_btc(25.0).unwrap();

    let secp = Secp256k1::new();
    // Get a keypair we control. In a real application these would come from a stored secret.
    let keypair = senders_keys(&secp, USER_A_PRIVATE_KEY);
    let (internal_key, _parity) = keypair.x_only_public_key();
    // let sender_address = Keygen::p2tr_addr_from_pk(*keypair.public_key(), Network::Regtest)?;

    // Get an address to send to.
    let tree = create_taproot_tree(&secp);
    let receiver_address = create_p2tr_address(tree);

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
        script_pubkey: receiver_address.script_pubkey(),
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
    // tx_id 5013596ac7b408f8eda62251989bd2872790b852b5992892f4301dc985f37573
    // tx_str "020000000001016a0e3115b3586748116c71b98b4c69e809331db87e7f6a3c1a10c46007e0988a0000000000ffffffff02404b4c000000000022512092eb55895873bc9a200002ee94b9d65ccff9a133b147b0be481cb9caeb9cc8b9d8a9b69400000000225120be27fa8b1f5278faf82cab8da23e8761f8f9bd5d5ebebbb37e0e12a70d92dd1601401bf7d1b447097cf875d7fcd519dd704d2cf79fcaec3583c05886933beb16301ea98e7a0d6a7568c56ef5027e4d39350199a0f8bfd2057b19455d83857723e38a00000000"
    Ok(())
}

#[test]
fn test_key_path_spend_taproot_tree_addr_to_receiver() -> anyhow::Result<()> {
    // bitcoin-cli -regtest -rpcwallet=benefactor listunspent 99 199 '["bcrt1phcnl4zcl2fu047pv4wx6y058v8u0n02at6lthvm7pcf2wrvjm5tqatn90k"]'
    let pre_txid_a = "e541bb6234a521740b9fc730f42103a7216e2c2b51c09d45122b7351d39ba390";
    let pre_vout_a = 0;
    let amount_in_sats_a = Amount::from_btc(25.0).unwrap();

    // 1. pre taproot addr
    let prev_taproot_tx_id = "5013596ac7b408f8eda62251989bd2872790b852b5992892f4301dc985f37573";
    let prev_taproot_tx_hex_str = "020000000001016a0e3115b3586748116c71b98b4c69e809331db87e7f6a3c1a10c46007e0988a0000000000ffffffff02404b4c000000000022512092eb55895873bc9a200002ee94b9d65ccff9a133b147b0be481cb9caeb9cc8b9d8a9b69400000000225120be27fa8b1f5278faf82cab8da23e8761f8f9bd5d5ebebbb37e0e12a70d92dd1601401bf7d1b447097cf875d7fcd519dd704d2cf79fcaec3583c05886933beb16301ea98e7a0d6a7568c56ef5027e4d39350199a0f8bfd2057b19455d83857723e38a00000000";
    let prev_taproot_tx = encode::deserialize_hex::<Transaction>(prev_taproot_tx_hex_str)?;
    let taproot_addr_utxo = prev_taproot_tx.output[0].clone();

    // 2. sender&receiver addr
    // 2.1 user A's info
    let secp = Secp256k1::new();
    // Get a keypair we control. In a real application these would come from a stored secret.
    let keypair_a = senders_keys(&secp, USER_A_PRIVATE_KEY);
    let (internal_key_a, _parity) = keypair_a.x_only_public_key();
    let (dummy_out_point_a, dummy_utxo_a) = dummy_unspent_transaction_output(
        &secp,
        internal_key_a,
        pre_txid_a,
        pre_vout_a,
        amount_in_sats_a,
    );
    let keypair_a_tweaked: TweakedKeypair = keypair_a.tap_tweak(&secp, None);

    // 2.2 taproot's info
    let taproot_tree = create_taproot_tree(&secp);
    // let taproot_addr = create_p2tr_address(taproot_tree.clone());
    // taproot key internal keypair
    let taproot_internal_keypair = senders_keys(&secp, USER_A_PRIVATE_KEY);
    let taproot_internal_keypair_tweaked: TweakedKeypair =
        taproot_internal_keypair.tap_tweak(&secp, Some(taproot_tree.merkle_root().unwrap()));
    // 2.3 receiver addr
    let receiver_addr = Address::from_str(RECEIVER_ADDR_STR)?.assume_checked();

    // 3. key path spend
    // 3.1: create psbt for key path spend.
    let spend_utxo = TxOut {
        value: dummy_utxo_a
            .value
            .checked_add(taproot_addr_utxo.value)
            .unwrap()
            .checked_sub(GAS_FEE)
            .unwrap(),
        script_pubkey: receiver_addr.script_pubkey(),
    };

    let mut unsigned_tx = Transaction {
        version: transaction::Version::TWO,  // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![
            TxIn {
                previous_output: dummy_out_point_a,
                script_sig: ScriptBuf::new(),
                sequence: Sequence::MAX, // Ignore nSequence.
                witness: Witness::default(),
            },
            TxIn {
                previous_output: OutPoint {
                    txid: prev_taproot_tx_id.parse().unwrap(),
                    vout: 0,
                },
                script_sig: ScriptBuf::new(),
                sequence: Sequence::MAX, // Ignore nSequence.
                witness: Witness::default(),
            },
        ],
        output: vec![spend_utxo],
    };
    ///////////////////////////////////////////////////
    ////////// sign a
    ///////////////////////////////////////////////////
    let input_index = 0;
    let sighash_type = TapSighashType::All;
    let prevouts = vec![dummy_utxo_a, taproot_addr_utxo];
    let prevouts = Prevouts::All(&prevouts);

    let mut sighasher = SighashCache::new(&mut unsigned_tx);
    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    // Sign the sighash using the secp256k1 library (exported by rust-bitcoin).
    let msg = Message::from(sighash);
    let signature = secp.sign_schnorr(&msg, &keypair_a_tweaked.to_inner());

    // Update the witness stack.
    let signature = bitcoin::taproot::Signature {
        signature,
        sighash_type,
    };

    *sighasher.witness_mut(input_index).unwrap() = Witness::p2tr_key_spend(&signature);

    ///////////////////////////////////////////////////
    ////////// sign taproot
    ///////////////////////////////////////////////////
    // Get the sighash to sign.
    let input_index = 1;
    let sighash_type = TapSighashType::All;
    // let prevouts = vec![dummy_utxo_a, taproot_addr_utxo];
    // let prevouts = Prevouts::All(&prevouts);

    let mut sighasher = SighashCache::new(&mut unsigned_tx);
    let sighash = sighasher
        .taproot_key_spend_signature_hash(input_index, &prevouts, sighash_type)
        .expect("failed to construct sighash");

    // Sign the sighash using the secp256k1 library (exported by rust-bitcoin).
    let msg = Message::from(sighash);
    let signature = secp.sign_schnorr(&msg, &taproot_internal_keypair_tweaked.to_inner());

    // Update the witness stack.
    let signature = bitcoin::taproot::Signature {
        signature,
        sighash_type,
    };

    *sighasher.witness_mut(input_index).unwrap() = Witness::p2tr_key_spend(&signature);

    let tx = sighasher.into_transaction().to_owned();
    let txid = tx.compute_txid();
    let tx_hex_str = encode::serialize_hex(&tx);
    println!("tx_hex_str {:?}", tx_hex_str);
    println!("txid {:?}", txid.to_string());
    // tx_hex_str "0200000000010184508bb2c88d1ac8486b0817862865624ab4bcf470b68e1ca83ae8915a61ddea0000000000ffffffff02a0252600000000002251208cda4c3c6a856d7c02dda303922defb123e3ceded8f4bdee5df139a92155e430b82126000000000022512092eb55895873bc9a200002ee94b9d65ccff9a133b147b0be481cb9caeb9cc8b901413fda560e05ff3757d5b23947e2a095a6e6faf605aeb645c1f3dc4990a79377b032db52c5c491b7ef9149fe54f4c5d204b93795101c7ec8d8b03bf1399a4c27910100000000"
    // txid "55600b1e07b12171e71e2c26f7fb71b6114ed76755c4584a7a776c6890f8f3d1"

    Ok(())
}
