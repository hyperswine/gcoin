// good interface to define certain pkg wide things and annotations
// maybe allow lib.rei to not exist? but what about pkg wide stuff?
// same with prelude?
use std::crypto::Sha512
use std::net::NetworkStatus

PublicAddress: Sha512
PublicKey: Sha512
PrivateKey: Sha512

// mixing behavior with data, ehh
// I think its just more ergonomic that way

@derive(Serialize, Deserialize)
Block: {
    transactions: Vec[Transaction]
    timestamp: Timestamp
    creator: PublicAddress

    // default empty
    new: (creator: _) -> Self {
        // if no positional args given, then always assume the same order of non fn fields
        Self {[], Timestamp(), creator}
    }

    // chain keyword allows self to be returned automatically at the end
    add_timestamp: chain (mut self, timestamp: Timestamp) {
        // note using self here doesnt consume
        self.timestamp = timestamp
    }

    // all chain does is change the return type to -> <modifier> Self and return self
    add_transaction: chain (mut self, transaction: Transaction) {
        // push takes and gives back
        self.transactions.push(transaction)
    }

    // get raw bytes of the entire block (e.g. for hashing)
    get_raw_data: (&self) -> Bytes => self.serialize()

    // parse a block from raw bytes, e.g. from another node
    parse_block: (bytes: Bytes) -> Self => self.deserialize(bytes)
}

@derive(Serialize, Deserialize)
Blockchain: {
    hash: Sha512
    merkle_root: Merkle
    blocks: Vec[Block]
}

Net: {
    const MESSAGE = "GCoin, It Just Works"
    const ACK = MESSAGE + ". ACK"
    connections: Vec[Connection]

    # connect to another node in the default Dual Mode (broadcast + listen?)
    connect: (addr: Ipv4) -> NetworkStatus {
        // use tcp
        let connection = std::net::tcp::connect(public_addr).await?
        // once connected, send a node message
        connection.send(MESSAGE).await?
        // add node to list of live connections
        connections.push(connection)
    }

    # handler for a request targed at your node
    listen: (from: Ipv4, message: String) {
        connections.push(std::net::connect(from, ACK))
    }
}
