use bitcoin::address::AddressData::P2sh;
use bitcoin::bip32::Xpriv;
use bitcoin::hashes::{hash160, Hash};
use bitcoin::opcodes::all::OP_CHECKSIG;
use bitcoin::{
    Address, CompressedPublicKey, KnownHrp, Network, PrivateKey, PubkeyHash, PublicKey, Script,
    ScriptBuf,
};
use lazy_static::lazy_static;
use secp256k1::XOnlyPublicKey;
use std::str::FromStr;

lazy_static! {
    static ref SECP: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();
}

pub struct Keygen;
impl Keygen {
    pub fn gen_sk(network: Network) -> PrivateKey {
        PrivateKey {
            network: network.into(),
            inner: secp256k1::SecretKey::new(&mut secp256k1::rand::thread_rng()),
            compressed: true,
        }
    }

    pub fn parsing_private_key(private_key_str: &str) -> anyhow::Result<PrivateKey> {
        let private_key = if let Ok(pk) = PrivateKey::from_wif(private_key_str) {
            pk
        } else if let Ok(pk) = Xpriv::from_str(private_key_str) {
            pk.to_priv()
        } else {
            panic!("Invalid private key")
        };
        Ok(private_key)
    }

    pub fn pk_from_sk(sk: &PrivateKey) -> PublicKey {
        PublicKey::from_private_key(&SECP, &sk)
    }

    ////////////////////////////////////////////////
    //////////// gen address from public key
    ////////////////////////////////////////////////
    pub(crate) fn p2sh_addr_from_pk(
        public_key: &PublicKey,
        network: Network,
    ) -> anyhow::Result<Address> {
        // TODO: needs test.
        // Create a P2WSH address
        let p2pk_script = Script::builder()
            .push_key(&public_key)
            .push_opcode(OP_CHECKSIG)
            .into_script();

        let addr = Address::p2sh(&p2pk_script, network)?;
        Ok(addr)
    }

    pub fn p2wpkh_addr_from_pk(pk: &PublicKey, network: Network) -> anyhow::Result<Address> {
        let pk = CompressedPublicKey::try_from(*pk)?;
        let addr = Address::p2wpkh(&pk, network);

        Ok(addr)
    }

    pub fn p2pkh_addr_from_pk(pk: PublicKey, network: Network) -> anyhow::Result<Address> {
        let addr = Address::p2pkh(pk, network);
        Ok(addr)
    }

    pub fn p2tr_addr_from_pk(pk: PublicKey, network: Network) -> anyhow::Result<Address> {
        let internal_key = XOnlyPublicKey::from(pk);
        let addr = Address::p2tr(&SECP, internal_key, None, KnownHrp::from(network));
        Ok(addr)
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gen_regtest() {
        let PRIVATE_KEY: PrivateKey = Keygen::gen_sk(Network::Regtest);
        let PUBLIC_KEY: PublicKey = Keygen::pk_from_sk(&PRIVATE_KEY);
        println!("private_key: {}", PRIVATE_KEY.to_string());
        println!("public_key: {}", PUBLIC_KEY.to_string());

        println!("");
        println!("test_gen_regtest");
        let p2tr_addr = Keygen::p2tr_addr_from_pk(PUBLIC_KEY.clone(), Network::Regtest).unwrap();
        let p2wpkh_addr = Keygen::p2wpkh_addr_from_pk(&PUBLIC_KEY, Network::Regtest).unwrap();
        let p2pkh_addr = Keygen::p2pkh_addr_from_pk(PUBLIC_KEY.clone(), Network::Regtest).unwrap();
        println!("p2tr_addr: {}", p2tr_addr.to_string());
        println!("p2wpkh_addr: {}", p2wpkh_addr.to_string());
        println!("p2pkh_addr: {}", p2pkh_addr.to_string());

        println!("");
        println!("test_gen_testnet");
        let p2tr_addr = Keygen::p2tr_addr_from_pk(PUBLIC_KEY.clone(), Network::Testnet).unwrap();
        let p2wpkh_addr = Keygen::p2wpkh_addr_from_pk(&PUBLIC_KEY, Network::Testnet).unwrap();
        let p2pkh_addr = Keygen::p2pkh_addr_from_pk(PUBLIC_KEY.clone(), Network::Testnet).unwrap();
        println!("p2tr_addr: {}", p2tr_addr.to_string());
        println!("p2wpkh_addr: {}", p2wpkh_addr.to_string());
        println!("p2pkh_addr: {}", p2pkh_addr.to_string());

        println!("");
        println!("test_gen_signet");
        let p2tr_addr = Keygen::p2tr_addr_from_pk(PUBLIC_KEY.clone(), Network::Signet).unwrap();
        let p2wpkh_addr = Keygen::p2wpkh_addr_from_pk(&PUBLIC_KEY, Network::Signet).unwrap();
        let p2pkh_addr = Keygen::p2pkh_addr_from_pk(PUBLIC_KEY.clone(), Network::Signet).unwrap();
        println!("p2tr_addr: {}", p2tr_addr.to_string());
        println!("p2wpkh_addr: {}", p2wpkh_addr.to_string());
        println!("p2pkh_addr: {}", p2pkh_addr.to_string());

        println!("");
        println!("test_gen_miannet");
        let p2tr_addr = Keygen::p2tr_addr_from_pk(PUBLIC_KEY.clone(), Network::Bitcoin).unwrap();
        let p2wpkh_addr = Keygen::p2wpkh_addr_from_pk(&PUBLIC_KEY, Network::Bitcoin).unwrap();
        let p2pkh_addr = Keygen::p2pkh_addr_from_pk(PUBLIC_KEY.clone(), Network::Bitcoin).unwrap();
        println!("p2tr_addr: {}", p2tr_addr.to_string());
        println!("p2wpkh_addr: {}", p2wpkh_addr.to_string());
        println!("p2pkh_addr: {}", p2pkh_addr.to_string());
    }

    #[test]
    fn test_address_from_str() {
        // generate with:
        //      bitcoin-cli -regtest getnewaddress
        let addr_str = "bc1qj638mlpa967p2s893cglz4y4cpk4qvce5zdv4uqzlc7vqxhju5gsa2nm57";

        let addr = Address::from_str(addr_str).unwrap().assume_checked();
        // let addr = Address::from_str(addr_str).unwrap();

        println!("addr: {}", addr.to_string());
        println!("addr type: {}", addr.address_type().unwrap()); // p2wpkh
    }

    #[test]
    fn test_gen_regtest_addr_by_sk() -> anyhow::Result<()> {
        let sk = "tprv8jzau9CfsdkXPkVBGi313RjQvsXggNwC4SZEBm3ohYAHQrHvBBG9GrPwMRWmzvB2UgkH7vEEjoMwia8kiY1jo6FzeshAfEw8d95ziJHYSTp";
        let private_key = Keygen::parsing_private_key(sk)?;
        let public_key = Keygen::pk_from_sk(&private_key);

        println!("");
        println!("test_gen_regtest");
        let p2tr_addr = Keygen::p2tr_addr_from_pk(public_key.clone(), Network::Regtest).unwrap();
        let p2wpkh_addr = Keygen::p2wpkh_addr_from_pk(&public_key, Network::Regtest).unwrap();
        let p2pkh_addr = Keygen::p2pkh_addr_from_pk(public_key.clone(), Network::Regtest).unwrap();
        println!("p2tr_addr: {}", p2tr_addr.to_string());
        println!("p2wpkh_addr: {}", p2wpkh_addr.to_string());
        println!("p2pkh_addr: {}", p2pkh_addr.to_string());

        Ok(())
    }
}
