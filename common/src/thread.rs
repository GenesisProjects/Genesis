use std::io::*;
use std::thread;
use std::time;
use std::sync::{Arc, Mutex};
use gen_message::*;


#[derive(Copy, Clone)]
pub enum ThreadStatus {
    Running,
    Stop,
    Pause
}

pub trait Thread {
    fn launch(name: String) {
        // TODO: make stack size configuable
        thread::Builder::new().stack_size(4 * 1024 * 1024).name(name.to_owned()).spawn(move || {
            let mut center = MESSAGE_CENTER.lock().unwrap();
            let ch = center.subscribe(&name);
            loop {
                let status = ThreadStatus::Stop;
                match status {
                    Running => {

                        let msg = ch.accept_msg();
                    },
                    Stop => {

                    },
                    Pause => {
                        break;
                    }
                }
                thread::sleep(time::Duration::from_millis(10));
            }
            center.unsubscribe(&name, ch);
        }).unwrap();
    }

    /// start runloop
    fn start(&mut self);

    /// pause runloop
    fn pause(&mut self);

    /// stop runloop
    fn stop(&mut self);


    fn run(&self);

}
