use wasmi::{self, ValueType, Signature};
use wasmi::ValueType::*;

pub mod index {
    pub const RETURN_FUNC: usize = 10;
    pub const CALL_FUNC: usize = 20;
    pub const CREATE_FUNC: usize = 30;
}

pub mod signatures {
    use wasmi::{self, ValueType};
    use wasmi::ValueType::*;

    pub const RETURN: wasmi::Signature = wasmi::Signature::new(&[I32, I32], None);

    pub const CALL: wasmi::Signature = wasmi::Signature::new(&[I32, I32], Some(I32));

    pub const CREATE: wasmi::Signature = wasmi::Signature::new(&[I32, I32], Some(I32));
}

pub struct SystemCall {

}

impl SystemCall {
    pub fn create(code: &[u8]) {
        unimplemented!()
    }

    pub fn call() {
        unimplemented!()
    }

    pub fn ret() {
        unimplemented!()
    }
}