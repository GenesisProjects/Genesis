use wasmi::*;

pub struct Runtime {
    module_instance: ModuleInstance
}

impl Runtime {
    fn new(buff: &[u8]) -> Self {
        unimplemented!()
    }

    fn new_with_contract(name: &'static str) -> Self {
        unimplemented!()
    }
}
