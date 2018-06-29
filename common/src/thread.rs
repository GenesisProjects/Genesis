use std::io::*;
use std::thread;
use std::time;

use observe::*;

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

            loop {
                let status = ThreadStatus::Stop;
                match status {
                    Running => {

                        //let msg = ch.accept_msg();
                    },
                    Stop => {

                    },
                    Pause => { break; }
                }
                thread::sleep(time::Duration::from_millis(10));
            }
        }).unwrap();
    }

    /// start runloop
    fn start(&mut self);

    /// pause runloop
    fn pause(&mut self);

    /// stop runloop
    fn stop(&mut self);


    fn run(&mut self);
}
