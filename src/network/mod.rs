// ---------------
// GCOIN NETWORK API
// ---------------

use crate::core::{Blockchain, Wallet};
use std::{
    io::{Read, Write},
    net::{Ipv6Addr, TcpStream},
};
use tokio::net::UdpSocket;

pub const GCOIN_PORT: u64 = 6900;

pub trait Connection {
    type Address;
    /// Connect to a node. Just establish the connection. To do more ACK or tests, use their functions
    /// Uses TCP and GCOIN handshake
    fn connect_to_node(&self, ipv6_addr: Self::Address) -> bool;
}

pub enum Node<RouterDetails, WalletDetails, StorageDetails> {
    Router(RouterDetails),
    Wallet(WalletDetails),
    Storage(StorageDetails),
}

// ---------------
// REAL NETWORK BACKEND
// ---------------

type RNode = Node<GCoinRouterNode, GCoinWalletNode, GCoinStorageNode>;

// protocols to use https://wiki.bitcoinsv.io/index.php/Application_layer_protocol

pub const GCOIN_AGREEMENT: &[u8] = b"GCOIN-HANDSHAKE";
pub const GCOIN_OK: &[u8] = b"GCOIN-OK";

// uses actual sockets and bytestreams

// create a tcp socket backend
pub struct GCoinRouterNode {
    ipv6_addr: Ipv6Addr,
    linked_nodes: Vec<Ipv6Addr>,
}

pub struct GCoinWalletNode {
    ipv6_addr: Ipv6Addr,
    wallet: Wallet,
}

pub struct GCoinStorageNode {
    ipv6_addr: Ipv6Addr,
    main_blockchain: Blockchain,
}

impl GCoinRouterNode {
    pub fn new(ipv6_addr: Ipv6Addr, linked_nodes: Vec<Ipv6Addr>) -> Self {
        Self {
            ipv6_addr,
            linked_nodes,
        }
    }
}

impl Connection for RNode {
    type Address = Ipv6Addr;

    // * if not already connected, create a socket if possible
    // the attempt to connect. Once done, return true
    fn connect_to_node(&self, ipv6_addr: Self::Address) -> bool {
        let mut res = TcpStream::connect(ipv6_addr.to_string());
        let mut stream = match res {
            Ok(r) => r,
            Err(err) => return false,
        };

        // attempt to agree on connection parameters
        match stream.write(GCOIN_AGREEMENT) {
            Ok(siz) => {
                // if size isnt equal, something went wrong
                assert_eq!(siz, GCOIN_AGREEMENT.len());

                // attempt to read response. Should match GCOIN-OK
                let mut response = [0 as u8; 128];
                let _siz = stream.read(&mut response).unwrap_or(0);
                if _siz == 0 {
                    return false;
                }

                assert_eq!(response, GCOIN_OK);

                return true;
            }
            Err(err) => return false,
        }
    }
}
