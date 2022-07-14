// ---------------
// SIMULATION
// ---------------

// simulation actually implements Network. The main module can then use the sim backend rather than a real network module
// like std::network

// uses IPC and virtual nodes. Everything in memory

// Virtual structs

// all keys must be unique

/*
SMP

a thread is spawned for each request
1. sleep 1 to simulate the propagation time
2. once arrived at its destination, sleep 1 to simulate processing of the data
3. if paying someone, dont bother waiting on the sender node. Just keep track of a certain storage node. The sender node should have sent the packet to storage nodes and wallet nodes
4. if trying to discover other nodes esp wallet and storage nodes, wait 10 sec max for a reply from the server node.  The print it out
5. if trying to get the latest blockchain, similar to 4 except we wait 15 sec max. Print out new blockchain

*/

use std::future::Future;

use crate::{
    core::{Blockchain, Pay, Wallet},
    network::Node,
};

type VNode = Node<VRouter, VWallet, VStorage>;
type VKey = u64;

// Ping and get something back later
pub trait Pingable {
    fn ping<F>(data: &[u8]) -> F::Output
    where
        F: Future;
}

// Assume that everyone has out of date info and must constantly listen and update their info
// When a new node connects, it should request for the latest data and tell everyone the latest block that it has. If it matches the main blockchain, it will send the rest of the blocks. Else it will send Error "blockchain not canon" and the sender can request for the entire blockchain instead

/// Stores as many ips (keys) of other nodes as possible
pub struct VRouter {
    key: VKey,
    other_nodes: Vec<VKey>,
    connected_nodes: Vec<VNode>,
}

pub struct VWallet {
    key: VKey,
    wallet: Wallet,
    connected_nodes: Vec<VNode>,
}

impl VWallet {
    pub fn new(key: VKey, wallet: Wallet, connected_nodes: Vec<VNode>) -> Self {
        Self {
            key,
            wallet,
            connected_nodes,
        }
    }
}

pub const WAIT_RESPONSE_TIMEOUT_SEC: usize = 5;
pub const MINIMUM_CONFIRMATION_RATIO: f32 = 0.51;

impl Pay for VWallet {
    fn pay(&mut self, amount: u64, payee: crate::core::GCoinAddress) -> bool {
        if amount == 0 {
            return false;
        }

        // ping network node about payee's address
        self.connected_nodes.iter().for_each(|node| {
            // ping the node and wait for it to return on another thread
            // if it satisfies the future and provides an OK, address exists, take that as confirmation
            // ? maybe make a threshold of X confirmations at least out of the ones that responded in Y seconds
        });

        // if true, then confident that payee indeed exists

        true
    }
}

/// Stores a copy of the entire blockchain
/// And other 'potential blockchains' in its lower list
pub struct VStorage {
    key: VKey,
    main_blockchain: Blockchain,
    connected_nodes: Vec<VNode>,
}

/// We start with an entry node which holds the keys of other nodes
/// It then calls vnet.connect() to attempt a connection
pub struct EntryNode {}

/// Entire view of the network instead of just one node's view
/// Contains a list of every node but doesnt know their relationship
/// If a node wants to connect, it will search if that node exists
pub struct VNetwork {
    nodes: Vec<VNode>,
}

// TESTS

#[test]
fn test_async() {}
