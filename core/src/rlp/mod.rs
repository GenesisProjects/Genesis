pub mod decoder;
pub mod encoder;
pub mod types;

pub trait RLPAttribute {
    fn encode(&self) -> Result<types::RLP, RLPError>;
    fn decode() -> Result<Self, RLPError>;
}