//! Trait allows a thread running with a run loop
//!

use std::io::*;
use std::thread;
use std::time::Duration;

use observe::*;
use gen_message::Message;

pub const LOOP_PERIOD: u64 = 100u64;


/// Thread status
#[derive(Copy, Clone)]
pub enum ThreadStatus {
    // Event loop is running
    Running,
    // Event loop has been killed
    Stop,
    // Event loop is on-hold
    Pause
}

/// Thread trait
pub trait Thread {
    /// Launch a run loop.
    /// Will create a thread context (controller) instance.
    /// The thread will subscribe to the channel `name`.
    fn launch<T>(name: String) where T: Observe + Thread, Self: Sized {
        // TODO: make stack size configuable
        thread::Builder::new().stack_size(64 * 1024 * 1024).name(name.to_owned()).spawn(move || {
            let mut context = T::new(name.to_owned());

            match &mut context {
                &mut Ok(ref mut context_ref) => {
                    context_ref.subscribe(name.to_owned());
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
                context.unsubscribe(name.to_owned());
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

    /// get status
    fn get_status(&self) -> ThreadStatus;

    /// init context instance
    fn new(name: String) -> Result<Self> where Self: Sized;
}
