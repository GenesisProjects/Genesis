use wasmi::*;
use wasmi::ValueType::*;
use std::sync::Mutex;

use std::collections::HashMap;

pub const RETURN_INDEX: usize       = 0x01;
pub const CALL_INDEX:   usize       = 0x02;
pub const CREATE_INDEX: usize       = 0x03;

lazy_static! {
    pub static ref SYSTEM_CALL: Mutex<SystemCall> = {
        Mutex::new(SystemCall::new())
    };
}

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub trait Api {
    fn create() {
        unimplemented!()
    }

    fn call() {
        unimplemented!()
    }

    fn ret() {
        unimplemented!()
    }
}

pub struct SystemCall {
    system_call_table: HashMap<usize, Signature>
}

impl SystemCall {
    pub fn new() -> Self {
        SystemCall {
            system_call_table: hashmap![
                CALL_INDEX => Signature::new(&[I32, I32][..], Some(I32))
            ]
        }
    }

    pub fn func_ref(&self, index: usize) -> Option<FuncRef> {
       self.system_call_table.get(&index).and_then(|sign| {
           Some(FuncInstance::alloc_host(sign.to_owned(), index))
       })
    }
}

impl Api for SystemCall {

}