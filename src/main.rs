use log::info;
use openssl::rsa::Rsa;
use pem::{encode, Pem};

fn main() {
    /*
    GCOIN

    1. pay someone => broadcast this out
    2. take a broadcast => update blockchain

    Interface

    - TUI for quick and simple wallet/address management
    - quickly pay someone by pasting their address and incrementing the amount of coins you want to pay them
    - receive notifications when the blockchain changes. Especially if you get paid or your payment seems to have been accepted by a lot of people
    */

    // if option --generate, generate a key pair and place them in ~/.gcoin/rsa.pub and ~/.gcoin/rsa.priv

    // if option --send, broadcast signal via the gcoin port 69000. All N nodes (network nodes) connected to your system should receive that signal from that socket
    // since you should have socket.bind() to them already

    // if option --find, find N nodes near you and attempt to create and bind sockets to them. If at least one accepts, send a test message to them

    // NOTE: you are also technically a 'server' for the network. The network nodes also broadcast new stuff to you. If you arent online, it wont send it. But when you do go online, it will send all pending changes to you
    // multiple nodes may send the same changes. Or if one of them is out of date, etc. it may send an older or newer copy of the blockchain. Always follow the longest chain and the blockchain that most nodes are on
    // have a reference count for each 'acceptable' blockchain
}

struct Blockchain {}

struct Block {}

struct GCoinAddress {
    address: u64,
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

fn generate_rsa_4096_key_pair() -> KeyPair {
    generate_key_pair(4096)
}

fn pay(gcoin_address: GCoinAddress) {}
