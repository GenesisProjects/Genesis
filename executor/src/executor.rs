use common::address::Address as Account;
use common::hash::Hash;

use wasm::WASM;

/// # Executor
/// **Usage**
/// - executor for vm
/// **Member**
/// - 1. ***wasm***:        WASM
/// ## Examples
/// ```
/// ```
pub struct Executor {
    wasm: WASM

}

impl Executor {

    /// # exec_vm(&mut self, 0)
    /// **Usage**
    /// - execute vm instance
    ///
    /// **Return**
    /// - 1. ***Result<()>***
    /// ## Examples
    /// ```
    /// ```
    fn exec_vm(
        &mut self,
        ...
    ) -> vm::Result<> {

    }

    /// # call(&mut self, 0)
    /// **Usage**
    /// **call smart contract**
    /// -
    /// **Return**
    /// - 1. ***Result<()>***
    /// ## Examples
    /// ```
    /// ```
    pub fn call(
        &mut self,
        ...
    ) -> vm::Result<>{

    }
}


