use std::collections::HashMap;
use std::io::*;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use observe::*;
use gen_message::Message;

pub const LOOP_PERIOD: u64 = 100u64;

lazy_static! {
    pub static ref THREAD_TABLE: Mutex<ControllerTable> = {
        let table: ControllerTable = ControllerTable::new();
        Mutex::new(table)
    };
}

pub struct ControllerTable {
    table: HashMap<String, Arc<Thread + Send + Sync>>
}

impl ControllerTable {
    pub fn new() -> Self {
        ControllerTable { table: HashMap::new() }
    }

    pub fn insert(
        &mut self,
        key: String,
        value: Arc<Thread + Send + Sync>) -> Option<Arc<Thread + Send + Sync>> {
        match self.table.get(&key) {
            Some(_) => None,
            None => self.table.insert(key, value)
        }
    }

    pub fn remove(&mut self, key: String) {
        self.table.remove(&key);
    }
}

#[derive(Copy, Clone)]
pub enum ThreadStatus {
    Running,
    Stop,
    Pause
}

pub trait Thread {
    fn launch<T>(name: String) where T: Observe + Thread, Self: Sized {
        // TODO: make stack size configuable
        thread::Builder::new().stack_size(4 * 1024 * 1024).name(name.to_owned()).spawn(move || {
            let mut context = if cfg!(test) {
                T::new(name)
            } else {
                T::mock(name)
            };

            match &mut context {
                &mut Ok(ref mut context_ref) => {
                    context_ref.subscribe();
                    context_ref.set_status(ThreadStatus::Running);
                    loop {
                        let ret = context_ref.run();
                        if let Some(msg) = context_ref.receive_async() {
                            let forward_msg = msg.clone();
                            match msg.msg.as_ref() {
                                "start" => {
                                    context_ref.set_status(ThreadStatus::Running);
                                },
                                "pause" => {
                                    context_ref.set_status(ThreadStatus::Pause);
                                },
                                "stop" => {
                                    context_ref.set_status(ThreadStatus::Stop);
                                },
                                _ => {
                                    context_ref.msg_handler(forward_msg);
                                },
                            }
                        }

                        if !ret {
                            break;
                        }
                        thread::sleep(Duration::from_millis(LOOP_PERIOD));
                    }
                },
                &mut Err(ref e) => {
                    println!("Error: {:?}", e);
                }
            }
            let _ = context.and_then(|mut context| {
                context.unsubscribe();
                Ok(context)
            });
        }).unwrap();
    }

    /// run loop
    fn run(&mut self) -> bool;

    /// handle message
    fn msg_handler(&mut self, msg: Message);

    /// set status
    fn set_status(&mut self, status: ThreadStatus);

    /// init instance
    fn new(name: String) -> Result<Self> where Self: Sized;

    /// init mock instance
    fn mock(name: String) -> Result<Self> where Self: Sized {
        unimplemented!()
    }
}
