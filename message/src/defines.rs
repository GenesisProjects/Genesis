// Chain controller message
pub static BLOCK_GEN: &'static str = "BLOCK_GEN";

// P2P controller message

// Consensus controller message

// Pool controller message
pub mod pool {
    pub const CLEAN_NONCE_CACHE: &'static str = "CLEAN_NONCE_CACHE";
}

// P2P controller message
pub mod p2p {
    pub const SEND_MESSAGE: &'static str = "SEND_MESSAGE";
}