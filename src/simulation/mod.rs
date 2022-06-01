// ---------------
// SIMULATION
// ---------------

// simulation actually implements Network. The main module can then use the sim backend rather than a real network module
// like std::network

// uses IPC and virtual nodes. Everything in memory

// Virtual structs

// all keys must be unique

use crate::{
    core::{Blockchain, Pay, Wallet},
    network::Node,
};

type VNode = Node<VRouter, VWallet, VStorage>;
type VKey = u64;

// Assume that everyone has out of date info and must constantly listen and update their info
// When a new node connects, it should request for the latest data and tell everyone the latest block that it has. If it matches the main blockchain, it will send the rest of the blocks. Else it will send Error "blockchain not canon" and the sender can request for the entire blockchain instead

/// Stores as many ips (keys) of other nodes as possible
pub struct VRouter {
    key: VKey,
    other_nodes: Vec<VKey>,
}

pub struct VWallet {
    key: VKey,
    wallet: Wallet,
}

impl VWallet {
    pub fn new(key: VKey, wallet: Wallet) -> Self {
        Self { key, wallet }
    }
}

impl Pay for VWallet {
    fn pay(&mut self, amount: u64, payee: crate::core::GCoinAddress) -> bool {
        
        if amount == 0 {
            return false;
        }

        // ping network node about payee's address
        // if true, then confident that payee indeed exists

        true
    }
}

/// Stores a copy of the entire blockchain
/// And other 'potential blockchains' in its lower list
pub struct VStorage {
    key: VKey,
    main_blockchain: Blockchain,
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
