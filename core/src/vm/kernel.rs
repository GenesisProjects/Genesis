use std::sync::Mutex;

use wasmi::*;

use super::system_call::*;

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

        match index {
            index::RETURN_FUNC => unimplemented!(),
            index::CALL_FUNC => unimplemented!(),
            index::CREATE_FUNC => unimplemented!(),
            _ => panic!("unknown function index {}", index)
        }
    }
}

impl ModuleImportResolver for Kernel {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, Error> {
        let func_ref = match field_name {
            "ret" => {
                FuncInstance::alloc_host(signatures::RETURN, index::RETURN_FUNC)
            },
            "call" => FuncInstance::alloc_host(signatures::CALL, index::CALL_FUNC),
            "create" => FuncInstance::alloc_host(signatures::CREATE, index::CREATE_FUNC),
            _ => return Err(
                InterpreterError::Function(
                    format!("host module doesn't export function with name {}", field_name)
                )
            )
        };
        Ok(func_ref)
    }
}