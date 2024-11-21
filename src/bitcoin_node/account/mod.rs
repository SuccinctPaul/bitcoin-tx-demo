mod test;

use crate::keygen::Keygen;
use bitcoin::{Address, Network, PrivateKey, PublicKey};

pub struct BitcoinAccount {
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
}

impl BitcoinAccount {
    pub fn gen(network: Network) -> anyhow::Result<BitcoinAccount> {
        let sk = Keygen::gen_sk(network);
        let pk = Keygen::pk_from_sk(&sk);

        Ok(BitcoinAccount {
            private_key: sk,
            public_key: pk,
        })
    }
}
