use std::fmt::Display;

// USE API
use log::info;
use openssl::rsa::Rsa;
use pem::{encode, Pem};
use ripemd::{Digest, Ripemd256};
use hex_literal::hex;

pub struct Wallet {
    gcoin_addresses: Vec<GCoinAddress>,
}

pub struct Blockchain {}

pub struct Block {}

#[derive(Debug, Clone, Copy)]
pub struct GCoinAddress {
    address: [u8; 32],
}

impl GCoinAddress {
    pub fn new(address: [u8; 32]) -> Self {
        Self { address }
    }

    pub fn from_rsa_pub_key(pub_key: &[u8]) -> Self {
        // RIPEMD256
        let mut hasher = Ripemd256::new();
        hasher.update(pub_key);
        let res: Vec<u8> = hasher.finalize().to_vec();

        let address: [u8; 32] = res
            .try_into()
            .expect("Something went wrong. Is the key hashable?");

        Self { address }
    }
}

impl Display for GCoinAddress {
    // hex output
    // would be better if it displayed it like 0xFFFF_FFFF...
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#04X?}", self.address)
    }
}

fn generate_key_pair(bits: u32) -> (Vec<u8>, Vec<u8>) {
    // rsa 4096 by default
    let rsa = Rsa::generate(bits).unwrap();

    let public_key = rsa.public_key_to_der().unwrap();
    let private_key = rsa.private_key_to_der().unwrap();

    let private_pem = Pem {
        tag: String::from("RSA PRIVATE KEY"),
        contents: private_key.clone(),
    };
    let private = encode(&private_pem);

    let public_pem = Pem {
        tag: String::from("RSA PUBLIC KEY"),
        contents: public_key.clone(),
    };
    let public = encode(&public_pem);

    info!("{}", private);
    info!("{}", public);

    (public_key, private_key)
}

type KeyPair = (Vec<u8>, Vec<u8>);

pub fn generate_rsa_4096_key_pair() -> KeyPair {
    generate_key_pair(4096)
}

pub fn pay(gcoin_address: GCoinAddress) {}

// TESTS
#[test]
fn test_basics() {
    let key_pair = generate_rsa_4096_key_pair();
    let pub_key = key_pair.1;
    let gcoin_addr = GCoinAddress::from_rsa_pub_key(&pub_key);

    println!("gcoin_addr (RIPEMD256) = {}", gcoin_addr);
}
