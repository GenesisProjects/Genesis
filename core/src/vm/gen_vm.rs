use super::kernel::Kernel;
use super::runtime::Runtime;

use std::collections::LinkedList;
use wasmi::RuntimeValue;

pub struct GenVM {
    kernel: Kernel,
    runtimes: LinkedList<Runtime>,
    return_val: Option<RuntimeValue>
}

impl GenVM {

}