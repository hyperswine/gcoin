use std::fmt::Display;

// USE API
use hex_literal::hex;
use log::info;
use openssl::{rsa::Rsa, hash::MessageDigest, pkey::PKeyRef};
use pem::{encode, Pem};
use ripemd::{Digest, Ripemd256};
use openssl::sign::{Signer, Verifier};

pub struct Wallet {
    gcoin_addresses: Vec<GCoinAddress>,
}

type RSAPubKey = [u8; 2048];
type RSAPrivKey = [u8; 4096];

pub struct Blockchain {}

type Hash32 = [u8; 32];

#[derive(Debug, Default)]
pub struct InputCoin(u64);

#[derive(Debug, Default)]
pub struct OutputCoin(u64);

#[derive(Debug, Default)]
pub struct Transaction {
    // timestamp of creation of the transaction
    timestamp: u32,
    inputs: Vec<InputCoin>,
    output: OutputCoin,
}

impl Transaction {
    pub fn new(timestamp: u32, inputs: Vec<InputCoin>, output: OutputCoin) -> Self {
        Self {
            timestamp,
            inputs,
            output,
        }
    }

    pub fn serialise(&mut self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];

        res.extend_from_slice(&self.timestamp.to_be_bytes());
        res.extend_from_slice(&self.output.0.to_be_bytes());
        self.inputs.iter().for_each(|i| {
            res.extend_from_slice(&i.0.to_be_bytes());
        });

        res
    }
}

pub struct Block {
    // header
    prev_block_hash: Hash32,
    merkle_root_hash: Hash32,
    // timestamp of creation of the block
    timestamp: u32,
    validator_signature: Hash32,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        prev_block_hash: Hash32,
        merkle_root_hash: Hash32,
        timestamp: u32,
        validator_signature: Hash32,
        transactions: Vec<Transaction>,
    ) -> Self {
        Self {
            prev_block_hash,
            merkle_root_hash,
            timestamp,
            validator_signature,
            transactions,
        }
    }

    // serialise all fields except signature
    pub fn serialise(&mut self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        // combine
        res.extend_from_slice(&self.prev_block_hash);
        res.extend_from_slice(&self.merkle_root_hash);
        // idk if its big endian or not. But as long as everything is the same its prob fine
        res.extend_from_slice(&self.timestamp.to_be_bytes());
        // res.append();
        self.transactions.iter_mut().for_each(|transaction| {
            let mut ser = transaction.serialise();
            res.append(&mut ser);
        });

        res
    }

    // sign a block as the validator
    pub fn sign(block: &mut Block, key_pair: &PKeyRef<Rsa>) {
        // take all the fields of the bitcoin and serialise into bytes
        // then hash it with your private key
        let fields = block.serialise();

        // SIGN!

        let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
        signer.update(data).unwrap();
        signer.update(data2).unwrap();
        let signature = signer.sign_to_vec().unwrap();
    }

    // given a block and the supposed validators public key, verify (dehash) the block fields to see if the signature matches
    pub fn verify_signature(block: Block, pub_key: &[u8]) {}
}

#[derive(Debug, Clone, Copy)]
pub struct GCoinAddress {
    address: Hash32,
}

impl GCoinAddress {
    pub fn new(address: Hash32) -> Self {
        Self { address }
    }

    pub fn from_rsa_pub_key(pub_key: &[u8]) -> Self {
        // RIPEMD256
        let mut hasher = Ripemd256::new();
        hasher.update(pub_key);
        let res: Vec<u8> = hasher.finalize().to_vec();

        let address: Hash32 = res
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
