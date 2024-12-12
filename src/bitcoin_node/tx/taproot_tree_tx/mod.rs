use crate::bitcoin_node::tx::{
    USER_A_PRIVATE_KEY, USER_A_PUBLIC_KEY, USER_B_PRIVATE_KEY, USER_C_PRIVATE_KEY,
};
use bitcoin::bip32::Xpriv;
use bitcoin::opcodes::all::OP_CHECKSIG;
use bitcoin::taproot::{TaprootBuilder, TaprootSpendInfo};
use bitcoin::{script, Address, Network, Script, ScriptBuf};
use secp256k1::{Keypair, Secp256k1, XOnlyPublicKey};
use std::str::FromStr;

pub mod key_path_spend;
pub mod script_path_spend;

pub fn create_p2tr_address(tree: TaprootSpendInfo) -> Address {
    let output_key = tree.output_key();
    Address::p2tr_tweaked(output_key, Network::Testnet)
}

// create a 1-2 multi-sig(B+C) taproot tree for
// Taproot output corresponds to a combination of a single public key condition (known as the
// internal key), and zero or more general conditions encoded in scripts organized in the form of a
// binary tree.
pub fn create_taproot_tree(secp: &Secp256k1<secp256k1::All>) -> TaprootSpendInfo {
    // Taproot can be spent by either:
    // - Spending using the key path i.e., with secret key corresponding to the tweaked `output_key`.
    let sk_a = Xpriv::from_str(&USER_A_PRIVATE_KEY).unwrap();
    let kp = Keypair::from_secret_key(secp, &sk_a.private_key);
    let internal_key = kp.x_only_public_key().0; // Ignore the parity.

    // - By satisfying any of the scripts in the script spend path. Each script can be satisfied by
    //   providing a witness stack consisting of the script's inputs, plus the script itself and the
    //   control block.
    let scripts = gen_one_of_two_multi_sig_scripts(secp);
    let builder = TaprootBuilder::new();
    let builder = builder.add_leaf(1, scripts[0].clone()).unwrap();
    let builder = builder.add_leaf(1, scripts[1].clone()).unwrap();

    // Create the taproot output.
    builder.finalize(secp, internal_key).unwrap()
}

// Create two basic scripts to test script path spend.
pub fn gen_one_of_two_multi_sig_scripts(secp: &Secp256k1<secp256k1::All>) -> Vec<ScriptBuf> {
    let user_a_single_sig_scipt = create_basic_single_sig_script(secp, USER_B_PRIVATE_KEY); // m/86'/1'/0'/0/0
    let user_b_single_sig_scipt = create_basic_single_sig_script(secp, USER_C_PRIVATE_KEY); // m/86'/1'/0'/0/0
    vec![user_a_single_sig_scipt, user_b_single_sig_scipt]
}
fn create_basic_single_sig_script(secp: &Secp256k1<secp256k1::All>, sk: &str) -> ScriptBuf {
    let sk = Xpriv::from_str(sk).unwrap();
    let kp = Keypair::from_secret_key(secp, &sk.private_key);
    let x_only_pubkey = kp.x_only_public_key().0;
    script::Builder::new()
        .push_slice(x_only_pubkey.serialize())
        .push_opcode(OP_CHECKSIG)
        .into_script()
}
