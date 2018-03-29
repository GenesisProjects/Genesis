pub enum RLPError {
    RLPErrorUnknown,

    RLPErrorUTF8,

    RLPEncodingErrorUnencodable,

    RLPDecodingErrorMalformed,
}

#[derive(Clone, Debug)]
pub enum RLP {
    RLPList { list: Vec<RLP> },
    RLPItem { value: String },
}

pub type EncodedRLP = Vec<u8>;