use std::io::*;
use std::thread;
use std::time;

use observe::*;
use gen_message::Message;

#[derive(Copy, Clone)]
pub enum ThreadStatus {
    Running,
    Stop,
    Pause
}

pub trait Thread {
    fn launch<T>(name: String) where T: Observe + Thread {
        // TODO: make stack size configuable
        thread::Builder::new().stack_size(4 * 1024 * 1024).name(name.to_owned()).spawn(move || {
            let mut context = T::new();
            match &mut context {
                &mut Ok(ref mut context_ref) => {
                    context_ref.subscribe();
                    context_ref.set_status(ThreadStatus::Running);
                    loop {
                        let ret = context_ref.run();
                        context_ref.update();
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
                    }
                },
                &mut Err(ref e) => {

                }
            }
            context.unwrap().unsubscribe();
        }).unwrap();
    }


    /// run loop
    fn run(&mut self) -> bool;

    /// handle message
    fn msg_handler(&mut self, msg: Message);

    /// set status
    fn set_status(&mut self, status: ThreadStatus);

    /// update
    fn update(&mut self);

    ///
    fn new() -> Result<Self> where Self: Sized;
}
