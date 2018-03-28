pub enum RLPError {
    RLPErrorUnknown
}

#[derive(Clone, Debug)]
pub enum RLP {
    RLPList { list: Vec<RLP> },
    RLPItem { value: String },
}

pub type EncodedRLP = Vec<u8>;