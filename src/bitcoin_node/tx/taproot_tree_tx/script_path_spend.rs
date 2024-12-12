use crate::bitcoin_node::tx::sign_tx_taproot::{GAS_FEE, SPEND_AMOUNT};
use crate::bitcoin_node::tx::taproot_tree_tx::{
    create_basic_single_sig_script, create_p2tr_address, create_taproot_tree,
    gen_one_of_two_multi_sig_scripts,
};
use crate::bitcoin_node::tx::{
    dummy_unspent_transaction_output, senders_keys, RECEIVER_ADDR_STR, USER_A_PRIVATE_KEY,
    USER_A_PUBLIC_KEY, USER_B_PRIVATE_KEY, USER_B_PUBLIC_KEY, USER_C_PUBLIC_KEY,
};
use bitcoin::bip32::Xpriv;
use bitcoin::consensus::encode;
use bitcoin::hashes::Hash;
use bitcoin::key::{TapTweak, TweakedKeypair};
use bitcoin::psbt::{Input, PsbtSighashType};
use bitcoin::sighash::{Prevouts, SighashCache};
use bitcoin::taproot::{LeafVersion, TaprootBuilder};
use bitcoin::{
    absolute, transaction, Address, Amount, Network, OutPoint, Psbt, PublicKey, ScriptBuf,
    Sequence, TapSighashType, Transaction, TxIn, TxOut, Txid, Witness,
};
use secp256k1::{Keypair, Message, Secp256k1};
use std::str::FromStr;

#[test]
fn test_a_to_taproot_tree_addr() -> anyhow::Result<()> {
    // bitcoin-cli -regtest -rpcwallet=benefactor listunspent 99 199 '["bcrt1phcnl4zcl2fu047pv4wx6y058v8u0n02at6lthvm7pcf2wrvjm5tqatn90k"]'
    let pre_txid = "a32d977e050272c508335597f5c4e9e78307841cbc0e3b912f99e46c0cbf40b3";
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
    // bcrt1pzevsdxn5ppkwmtsrud57cnx4uk6xc8yy39queuw2e7xmdk0rjysqvhfc3s
    println!("receiver_address:{:?}", receiver_address.to_string());
    assert_eq!(
        "bcrt1pzevsdxn5ppkwmtsrud57cnx4uk6xc8yy39queuw2e7xmdk0rjysqvhfc3s",
        receiver_address.to_string()
    );

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
    // tx_id 6a27fe160393f5fbb401cd948bd7585d7b236cbc8b2a55f86e5c237ae6676faa
    // tx_str "02000000000101b340bf0c6ce4992f913b0ebc1c840783e7e9c4f597553308c57202057e972da30000000000ffffffff02404b4c00000000002251201659069a74086cedae03e369ec4cd5e5b46c1c848941ccf1cacf8db6d9e39120d8a9b69400000000225120be27fa8b1f5278faf82cab8da23e8761f8f9bd5d5ebebbb37e0e12a70d92dd160140af0980159da913e6e719b98f813a539c691f5f9632a801e32dfc01e689a8473fe4dd36fe383d6edd3ff71b8af316040f40b0731a26938bbe5d6c074317944ca900000000"

    Ok(())
}

#[test]
#[ignore]
fn test_script_path_spend_taproot_tree_addr_to_a() -> anyhow::Result<()> {
    // 1. pre tx
    let prev_tx_id = "6a27fe160393f5fbb401cd948bd7585d7b236cbc8b2a55f86e5c237ae6676faa";
    let prev_tx_hex_str = "02000000000101b340bf0c6ce4992f913b0ebc1c840783e7e9c4f597553308c57202057e972da30000000000ffffffff02404b4c00000000002251201659069a74086cedae03e369ec4cd5e5b46c1c848941ccf1cacf8db6d9e39120d8a9b69400000000225120be27fa8b1f5278faf82cab8da23e8761f8f9bd5d5ebebbb37e0e12a70d92dd160140af0980159da913e6e719b98f813a539c691f5f9632a801e32dfc01e689a8473fe4dd36fe383d6edd3ff71b8af316040f40b0731a26938bbe5d6c074317944ca900000000";
    let prev_tx = encode::deserialize_hex::<Transaction>(prev_tx_hex_str)?;
    let taproot_addr_utxo = prev_tx.output[0].clone();

    // 2. sender&receiver addr
    let secp = Secp256k1::new();
    // unlock user
    let taproot_internal_keypair = senders_keys(&secp, USER_B_PRIVATE_KEY);
    let tweaked_keypair: TweakedKeypair = taproot_internal_keypair.tap_tweak(&secp, None);
    // sender info
    // Get a keypair we control. In a real application these would come from a stored secret.
    // let taproot_internal_sk = Xpriv::from_str(&USER_A_PRIVATE_KEY).unwrap();
    // let taproot_internal_keypair =
    //     Keypair::from_secret_key(&secp, &taproot_internal_sk.private_key); // Get an address to send to.

    // receiver addr
    let receiver_addr = Address::from_str(RECEIVER_ADDR_STR)?.assume_checked();
    // taproot tree, and related tree leaves
    let tree_leaves_scripts = gen_one_of_two_multi_sig_scripts(&secp);
    let tree = create_taproot_tree(&secp);
    let sender_addr = create_p2tr_address(tree.clone());
    assert_eq!(
        "bcrt1pzevsdxn5ppkwmtsrud57cnx4uk6xc8yy39queuw2e7xmdk0rjysqvhfc3s",
        sender_addr.to_string()
    );
    let selected_lock_script = tree_leaves_scripts.first().unwrap().to_owned();
    println!("sender_addr:{:?}", sender_addr.to_string());
    let control_block = tree
        .control_block(&(selected_lock_script.clone(), LeafVersion::TapScript))
        .unwrap();

    // 3. key path spend
    // 3.1: create psbt for key path spend.
    let spend_utxo = TxOut {
        value: SPEND_AMOUNT.checked_div(2).unwrap(),
        script_pubkey: receiver_addr.script_pubkey(),
    };
    let change_utxo = TxOut {
        value: taproot_addr_utxo
            .value
            .checked_sub(spend_utxo.value)
            .unwrap()
            .checked_sub(GAS_FEE)
            .unwrap(),
        script_pubkey: sender_addr.script_pubkey(),
    };
    let mut unsigned_tx = Transaction {
        version: transaction::Version::TWO,  // Post BIP-68.
        lock_time: absolute::LockTime::ZERO, // Ignore the locktime.
        input: vec![TxIn {
            previous_output: OutPoint {
                txid: prev_tx_id.parse().unwrap(),
                vout: 0,
            },
            script_sig: ScriptBuf::new(),
            sequence: Sequence::MAX, // Ignore nSequence.
            witness: Witness::default(),
        }],
        output: vec![spend_utxo, change_utxo],
    };

    // Get the sighash to sign.
    let input_index = 0;
    let sighash_type = TapSighashType::All;
    let prevouts = vec![taproot_addr_utxo];
    let prevouts = Prevouts::All(&prevouts);

    let mut sighasher = SighashCache::new(&mut unsigned_tx);
    let sighash = sighasher
        .taproot_script_spend_signature_hash(
            input_index,
            &prevouts,
            selected_lock_script.tapscript_leaf_hash(),
            sighash_type,
        )
        .expect("failed to construct sighash");

    // Sign the sighash using the secp256k1 library (exported by rust-bitcoin).
    let msg = Message::from(sighash);
    let signature = secp.sign_schnorr(&msg, &tweaked_keypair.to_inner());

    // Update the witness stack.
    let signature = bitcoin::taproot::Signature {
        signature,
        sighash_type,
    };
    *sighasher.witness_mut(input_index).unwrap() = {
        let mut witness = Witness::new();
        witness.push(&signature.to_vec()); // unlock script
                                           // witness.push(&signature.serialize()); // unlock script
        witness.push(selected_lock_script.to_bytes());
        witness.push(control_block.serialize());
        witness
    };

    let tx = sighasher.into_transaction().to_owned();
    let txid = tx.compute_txid();
    let tx_hex_str = encode::serialize_hex(&tx);
    println!("tx_hex_str {:?}", tx_hex_str);
    println!("txid {:?}", txid.to_string());
    // tx_hex_str "02000000000101aa6f67e67a235c6ef8552a8bbc6c237b5d58d78b94cd01b4fbf5930316fe276a0000000000ffffffff02a0252600000000002251208cda4c3c6a856d7c02dda303922defb123e3ceded8f4bdee5df139a92155e430b8212600000000002251201659069a74086cedae03e369ec4cd5e5b46c1c848941ccf1cacf8db6d9e39120044199fe8e3abcdf80fad612fe6404c2f11886b9fc9860caa4edf03c08aadad22a93acc81a61a7d439c74ba7d591a7f48589aa77226a4c68ec91c18aef4129757e4b014199fe8e3abcdf80fad612fe6404c2f11886b9fc9860caa4edf03c08aadad22a93acc81a61a7d439c74ba7d591a7f48589aa77226a4c68ec91c18aef4129757e4b012220259ea961fd6bf615c7328ec9538cfc911d50c44f07cbe71bad0f9367e566cc1bac41c1a6ac32163539c16b6b5dbbca01b725b8e8acaa5f821ba42c80e7940062140d1942d6c91687e10a377f24c22b81a356a3c8349883ddd47aac01f38b84e71921f600000000"
    // txid "4513cfca75b8e998748938013907173cb1d9016a7f9b498596f3cc19acac90e6"

    Ok(())
}
