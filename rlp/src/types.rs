pub enum RLPError {
    RLPErrorUnknown
}

pub enum RLP {
    RLPList { list: Vec<RLP> },
    RLPItem { value: String },
    RLPNone
}

pub type EncodedRLP = Vec<u8>;