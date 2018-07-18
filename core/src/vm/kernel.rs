use std::sync::Mutex;

use wasmi::*;

lazy_static! {
    pub static ref GEN_KERNEL: Mutex<Kernel> = {
        Mutex::new(Kernel::new())
    };
}

pub struct Kernel {

}

impl Kernel {
    pub fn new() -> Self {
        Kernel {}
    }
}

impl Externals for Kernel {
    fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        unimplemented!()
    }
}

impl ModuleImportResolver for Kernel {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, Error> {
        unimplemented!()
    }
}