use std::io::*;
use std::thread;
use std::sync::Arc;

use gen_message::*;

#[derive(Copy, Clone)]
pub enum ThreadStatus {
    Running,
    Stop,
    Pause
}

pub trait Thread {
    fn start(&mut self, name: String) -> thread::JoinHandle<()> {
        // TODO: make staack size configuable
        thread::Builder::new().stack_size(4 * 1024 * 1024).name(name.to_owned()).spawn(move || {
            let mut center = MESSAGE_CENTER.lock().unwrap();
            let ch = center.subscribe(&name);
            loop {
                //shared_self.status();

            }
            center.unsubscribe(&name, ch);
        }).unwrap()
    }

    fn stop(&mut self) {

    }

    fn status(&self) -> ThreadStatus;

    fn update_status(&mut self, status: ThreadStatus);

    fn process(&mut self) -> Result<()>;
}
