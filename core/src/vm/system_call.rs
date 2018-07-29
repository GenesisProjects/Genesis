use std::collections::HashMap;
use std::sync::Mutex;

use common::address::Address;
use common::hash::Hash;
use storage::StorageCache;

use super::gen_vm::GenVM;
use super::kernel::{Kernel, KernelRef};
use super::runtime::*;
use super::selector::Selector;

use wasmi::*;
use wasmi::ValueType::*;

pub const RETURN_INDEX: usize = 0x01;
pub const CALL_INDEX: usize = 0x02;
pub const CREATE_INDEX: usize = 0x03;
pub const TEST_INDEX: usize = 0x04;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

pub trait Api {
    fn call(&mut self, addr: u32, abi: u32, input_balance: u32) -> RuntimeValue;

    fn create(&mut self, abi: u32, input_balance: u32) -> RuntimeValue;

    fn test(&self);

    fn get_account_balance(&mut self, addr: u32) -> RuntimeValue;

    fn get_current_input(&mut self) -> RuntimeValue;

    fn read_storage(&mut self, key: u32, offset: u32) -> Result<(), Error>;

    fn write_storage(&mut self, key: u32, offset: u32) -> Result<(), Error>;
}

pub struct SystemCall {
    kernel: KernelRef
}

impl SystemCall {
    pub fn new_with_kernel(kernel: KernelRef) -> Self {
        SystemCall {
            kernel: kernel
        }
    }

    fn init_runtime_with_parent(&self, parent_context_ref: RuntimeContextRef, addr: Address, input_balance: u32) -> Result<Runtime, Error> {
        if input_balance > parent_context_ref.borrow().balance {
            return Err(Error::Validation("Insufficient balance".into()));
        }
        let mut code: Vec<u8> = vec![];
        Kernel::load_contract_account(addr).and_then(|account| {
            Kernel::load_code(&account, &mut code).and_then(|_| {
                let child_runtime = Runtime::new(
                    account,
                    parent_context_ref.borrow().depth + 1,
                    &SysCallResolver::new(),
                    &code[..],
                    input_balance,
                );
                Ok(child_runtime)
            })
        })
    }

    // Load data from memory
    fn memory_load(&self, offset: u32, size: usize) -> Result<Vec<u8>, Error> {
        let kernel_ref = self.kernel.borrow();
        let memory_ref = kernel_ref.top_memory();
        match memory_ref {
            Some(memory_ref) => memory_ref.get(offset, size),
            None => Err(Error::Validation("Can not access memory".into()))
        }
    }

    // Set data to memory
    fn memory_set(&self, offset: u32, val: &[u8]) -> Result<(), Error> {
        let kernel_ref = self.kernel.borrow();
        let memory_ref = kernel_ref.top_memory();
        match memory_ref {
            Some(memory_ref) => memory_ref.set(offset, val),
            None => Err(Error::Validation("Can not access memory".into()))
        }
    }

    // excecute code
    fn execute(
        &mut self,
        module_ref: ModuleRef,
        selector: Selector,
        time_limit: usize,
    ) -> Result<RuntimeResult, Error> {
        match module_ref.invoke_export(
            &selector.name()[..],
            &selector.args(),
            self,
        ) {
            Ok(ret) => {
                Ok(RuntimeResult::new_with_ret(ret))
            }
            Err(e) => Err(e)
        }
    }
}

impl Api for SystemCall {
    fn call(&mut self, addr: u32, abi: u32, input_balance: u32) -> RuntimeValue {
        let parent = self.kernel.borrow().top_context();
        let result = self.memory_load(addr, 32).and_then(|vec| {
            match Address::try_from(vec) {
                Ok(addr) => {
                    self.init_runtime_with_parent
                    (
                        parent,
                        addr,
                        input_balance,
                    ).and_then(|new_runtime| {
                        // sub balance from top context
                        let top_context = self.kernel.borrow().top_context();
                        top_context.borrow_mut().balance -= input_balance;

                        // push stack
                        if self.kernel.borrow_mut().push_runtime(
                            new_runtime.context(),
                            new_runtime.memory_ref(),
                            new_runtime.module_ref().unwrap(),
                            StorageCache::new(),
                        ) {
                            //TODO: decode from memory
                            let selector = Selector::new(
                                "test_1".into(),
                                vec![],
                                vec![],
                            );
                            //TODO: end decode from memory

                            // begin excution
                            let result = self.execute(new_runtime.module_ref().unwrap(), selector, 1000);

                            // add remain balance back to top context
                            let new_top_context = self.kernel.borrow().top_context();
                            let remain_balance = new_top_context.borrow_mut().balance;
                            top_context.borrow_mut().balance += remain_balance;

                            // pop stack
                            self.kernel.borrow_mut().pop_runtime();

                            // merge result
                            self.kernel.borrow_mut().merge_result(&result);

                            result
                        } else {
                            Err(Error::Validation("Stack overflow".into()))
                        }
                    })
                }
                Err(_) => Err(Error::Validation("Invalid Address".into()))
            }
        });

        match result {
            Ok(r) => r.return_val().unwrap(),
            Err(_) => RuntimeValue::I32(-1i32)
        }
    }

    fn create(&mut self, abi: u32, input_balance: u32) -> RuntimeValue {
        unimplemented!()
    }

    fn test(&self) {
        println!("test12311");
    }

    // get account balance with provided addr
    fn get_account_balance(&mut self, addr: u32) -> RuntimeValue {
        let result = self.memory_load(addr, 32).and_then(|vec| {
            match Address::try_from(vec) {
                Ok(addr) => {
                    Kernel::load_contract_account(addr).and_then(|account| {
                        Ok(RuntimeValue::I32(account.balance() as i32))
                    })
                }
                Err(_) => Err(Error::Validation("Invalid Address".into()))
            }
        });

        match result {
            Ok(r) => r,
            Err(_) => RuntimeValue::I32(0)
        }
    }

    // get current input of executed contract
    fn get_current_input(&mut self) -> RuntimeValue {
        let kernel_ref = self.kernel.borrow();
        let context_ref = kernel_ref.top_context();
        let current_input = context_ref.borrow().balance as i32;
        RuntimeValue::I32(current_input)
    }

    // Read from the storage
    fn read_storage(&mut self, key: u32, offset: u32) -> Result<(), Error>
    {
        Kernel::load_contract_account(self.kernel.borrow().address()).and_then(|account| {
            self.memory_load(key, 32).and_then(|vec| {
                let mut key: Hash = [0u8; 32];
                let vec = &vec[..key.len()];
                key.copy_from_slice(vec);
                let storage = account.storage();
                match self.kernel
                    .borrow_mut()
                    .top_cache_mut()
                    .read(&key, &storage) {
                    Ok(chunk) => {
                        self.memory_set(offset, &chunk[..])
                    },
                    Err(e) => {
                        Err(Error::Validation("Can not read storage".into()))
                    }
                }
            })
        })
    }

    // Write to storage
    fn write_storage(&mut self, key: u32, offset: u32) -> Result<(), Error>
    {
        self.memory_load(offset, 32).and_then(|val_vec| {
            self.memory_load(key, 32).and_then(|key_vec| {
                Kernel::load_contract_account(self.kernel.borrow().address()).and_then(|account| {
                    let mut key: Hash = [0u8; 32];
                    let key_vec = &key_vec[..key.len()];
                    key.copy_from_slice(key_vec);
                    account.set_storage(key, &val_vec[..]);
                    Ok(())
                })
            })
        })
    }
}

pub trait SysCallRegister {
    fn register(&self, sys_resolver: &SysCallResolver) -> ModuleRef;
}

impl SysCallRegister for Module {
    fn register(&self, sys_resolver: &SysCallResolver) -> ModuleRef {
        let mut imports = ImportsBuilder::new();
        imports.push_resolver("env", sys_resolver);

        ModuleInstance::new(
            self,
            &imports,
        ).expect("Failed to instantiate module")
            .assert_no_start()
    }
}

impl Externals for SystemCall {
    fn invoke_index(&mut self, index: usize, args: RuntimeArgs) -> Result<Option<RuntimeValue>, Trap> {
        match index {
            CALL_INDEX => {
                Ok(Some(self.call(args.nth(0), args.nth(1), args.nth(2))))
            }
            _ => panic!("unknown function index {}", index)
        }
    }
}

/// # SysCallResolver
/// **Usage**
/// - WASMI Import resolver
/// **Member**
/// - 1. ***max_memory***:      instance of [u32]
#[derive(Default)]
pub struct SysCallResolver {
    system_call_table: HashMap<usize, Signature>
}

impl SysCallResolver {
    /// # new(2)
    /// **Usage**
    /// - create new SysCallResolver
    /// **Return**
    /// - 1. ***Self***
    /// ## Examples
    /// ```
    /// ```
    pub fn new() -> SysCallResolver {
        SysCallResolver {
            system_call_table: hashmap![
                CALL_INDEX => Signature::new(&[I32, I32, I32][..], Some(I32)),
                TEST_INDEX => Signature::new(&[][..], None)
            ]
        }
    }

    /// # func_ref
    /// **Usage**
    /// - Generate FuncRef for WASMI based on external functions
    /// **Parameters**
    /// - 1. ***usize(index)***: external function index
    /// **Return**
    /// - 1. ***Option<FuncRef>***
    /// ## Examples
    /// ```
    /// ```
    pub fn func_ref(&self, index: usize) -> Option<FuncRef> {
        self.system_call_table.get(&index).and_then(|sign| {
            Some(FuncInstance::alloc_host(sign.to_owned(), index))
        })
    }
}

impl ModuleImportResolver for SysCallResolver {
    fn resolve_func(
        &self,
        field_name: &str,
        _signature: &Signature,
    ) -> Result<FuncRef, Error> {
        match field_name {
            "call" => {
                match self.func_ref(CALL_INDEX) {
                    Some(f) => Ok(f),
                    None => Err(Error::Function(
                        format!("function index: {} is not register in the kernel", CALL_INDEX)
                    ))
                }
            }
            "test" => {
                match self.func_ref(TEST_INDEX) {
                    Some(f) => Ok(f),
                    None => Err(Error::Function(
                        format!("function index: {} is not register in the kernel", TEST_INDEX)
                    ))
                }
            }
            _ =>
                Err(Error::Function(
                    format!("kernel module doesn't export function with name {}", field_name)
                ))
        }
    }
}
