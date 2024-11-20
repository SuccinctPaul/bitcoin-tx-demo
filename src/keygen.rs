use bitcoin::bip32::Xpriv;
use bitcoin::{Address, CompressedPublicKey, KnownHrp, Network, PrivateKey, PublicKey};
use lazy_static::lazy_static;
use secp256k1::XOnlyPublicKey;
use std::str::FromStr;

lazy_static! {
    static ref SECP: secp256k1::Secp256k1<secp256k1::All> = secp256k1::Secp256k1::new();
}

pub struct Keygen;
impl Keygen {
    pub fn gen_sk() -> PrivateKey {
        PrivateKey {
            network: Network::Regtest.into(),
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
    pub fn p2wpkh_addr_from_pk(pk: &PublicKey, network: Network) -> anyhow::Result<Address> {
        let pk = CompressedPublicKey::try_from(*pk)?;
        let addr = Address::p2wpkh(&pk, network);

        Ok(addr)
    }

    pub fn p2pkh_addr_from_pk(pk: PublicKey, network: Network) -> anyhow::Result<Address> {
        let addr = Address::p2pkh(pk, Network::Regtest);
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

    lazy_static! {
        static ref PRIVATE_KEY: PrivateKey = Keygen::gen_sk();
        static ref PUBLIC_KEY: PublicKey = { Keygen::pk_from_sk(&PRIVATE_KEY) };
    }

    #[test]
    fn test_gen_regtest() {
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
}
