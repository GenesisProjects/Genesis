#[macro_use]
extern crate lazy_static;
extern crate rand;

use std::sync::{Arc, Mutex, Condvar};
use std::cell::RefCell;
use std::rc::Rc;
use std::collections::{LinkedList, HashMap};

const DEFAULT_MSG_QUEUE_SIZE: usize = 1024;

lazy_static! {
    pub static ref MESSAGE_CENTER: Mutex<MessageCenter> = {
        Mutex::new(MessageCenter::new())
    };
}

fn random_string(length: usize) -> String {
    use rand::Rng;
    let s = rand::thread_rng()
        .gen_ascii_chars()
        .take(length)
        .collect::<String>();
    s
}

pub struct MessageQueue {
    queue: LinkedList<u16>,
    limit: usize
}

impl MessageQueue {
    pub fn new(limit: usize) -> Self {
        MessageQueue {
            queue: LinkedList::<u16>::new(),
            limit: limit
        }
    }

    pub fn enqueue_msg(&mut self, msg: u16) -> Result<usize, &'static str> {
        let cur_size = self.queue.len();
        if cur_size >= self.limit {
            Err("The message queue is full.")
        } else {
            self.queue.push_back(msg);
            Ok(cur_size + 1)
        }
    }

    pub fn dequeue_msg(&mut self) -> Option<u16> {
        self.queue.pop_front()
    }

    pub fn flush(&mut self) -> LinkedList<u16> {
        let mut result = LinkedList::<u16>::new();
        loop {
            if self.queue.is_empty() { break; }
            result.push_front(self.queue.pop_back().unwrap());
        }
        result
    }

    pub fn get_size(&self) -> usize {
        self.queue.len()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}
/// Message channel is thread safe.
pub struct MessageChannel {
    pub uid: String,
    pub cond_var_pair: Arc<(Mutex<RefCell<MessageQueue>>, Condvar)>
}

impl PartialEq for MessageChannel {
    fn eq(&self, other: &MessageChannel) -> bool {
        self.uid == other.uid
    }
}

unsafe impl std::marker::Send for MessageChannel {}
unsafe impl std::marker::Sync for MessageChannel {}

impl MessageChannel {
    pub fn new() -> Self {
        let msg_queue = MessageQueue::new(DEFAULT_MSG_QUEUE_SIZE);
        let cond_var_pair = Arc::new((Mutex::new(RefCell::new(msg_queue)), Condvar::new()));
        MessageChannel {
            uid: random_string(32),
            cond_var_pair: cond_var_pair
        }
    }

    pub fn new_with_pair(name: &String, cond_var_pair: Arc<(Mutex<RefCell<MessageQueue>>, Condvar)>) -> Self {
        MessageChannel {
            uid: name.to_owned(),
            cond_var_pair: cond_var_pair.clone()
        }
    }

    pub fn new_with_size(size: usize) -> Self {
        let msg_queue = MessageQueue::new(size);
        let cond_var_pair = Arc::new((Mutex::new(RefCell::new(msg_queue)), Condvar::new()));
        MessageChannel {
            uid: random_string(32),
            cond_var_pair: cond_var_pair
        }
    }

    pub fn send_msg(&mut self, msg: u16) -> Result<usize, &'static str> {
        let (ref mutex_lock, ref cond_var) = *self.cond_var_pair;
        let queue_ref = mutex_lock.lock().unwrap();
        let result = queue_ref.borrow_mut().enqueue_msg(msg);
        cond_var.notify_one();
        result
    }

    pub fn send_msg_without_cache(&mut self, msg: u16) -> Result<usize, &'static str> {
        self.flush();
        self.send_msg(msg)
    }

    pub fn accept_msg(&mut self) -> u16 {
        let (ref mutex_lock, ref cond_var) = *(self.cond_var_pair);
        let mut queue = mutex_lock.lock().unwrap();
        // if the queue is not empty
        if !queue.borrow().is_empty() {
            return queue.borrow_mut().dequeue_msg().unwrap();
        }
        // else wait for new msg
        let result: u16;
        // loop to avoid spurious wakeup
        loop {
            queue = cond_var.wait(queue).unwrap();
            if !queue.borrow().is_empty() {
                result = queue.borrow_mut().dequeue_msg().unwrap();
                break;
            }
        }
        result
    }

    pub fn flush(&mut self) -> LinkedList<u16> {
        let (ref mutex_lock, ref cond_var) = *(self.cond_var_pair);
        let queue_lock = mutex_lock.lock().unwrap();
        let mut queue_ref = queue_lock.borrow_mut();
        queue_ref.flush()
    }
}

pub struct MessageCenter {
    channel_map: HashMap<String, Vec<MessageChannel>>
}


impl MessageCenter {
    pub fn new() -> Self {
        MessageCenter {
            channel_map: HashMap::<String, Vec<MessageChannel>>::new()
        }
    }

    pub fn subscribe(&mut self, name: &String) -> &mut MessageChannel {
        let existed = self.channels_exist_by_name(name);
        if existed {
            let new_channel = MessageChannel::new();
            let chs = self.channel_map.get_mut(name).unwrap();
            chs.push(new_channel);
            chs.last_mut().unwrap()
        } else {
            let new_channel = MessageChannel::new();
            self.channel_map.insert(name.to_owned(), vec![new_channel]);
            let chs = self.channel_map.get_mut(name).unwrap();
            chs.last_mut().unwrap()
        }
    }

    pub fn unsubscribe(&mut self, name: &String, ch: &MessageChannel) {
        let existed = self.channels_exist_by_name(name);
        if existed {
            let chs = self.channel_map.get_mut(name).unwrap();
            let pos = chs.iter().position(|x| *x == *ch);
            pos.and_then(|r| { chs.remove(r); Some(r) });
        }

        let chs = self.channel_map.get_mut(name).unwrap();
        if chs.len() == 0 {
            //self.channel_map.remove(name);
        }
    }

    pub fn send(&mut self, name: &String, msg: u16) {
        let existed = self.channels_exist_by_name(name);
        if !existed {
            return;
        } else {
            let channels = self.channel_map.get_mut(name).unwrap();
            for i in 0 .. channels.len() {
                let mut ch = &mut channels[i];
                ch.send_msg(msg);
            }
        }
    }

    pub fn channels_by_name(&mut self, name: &String) -> Vec<&mut MessageChannel> {
        let existed = self.channels_exist_by_name(name);
        if !existed {
            vec![]
        } else {
            let channels = self.channel_map.get_mut(name).unwrap();
            let mut result = vec![];
            for e in channels {
                result.push(e)
            }
            result
        }
    }

    pub fn channels_exist_by_name(&self, name: &String) -> bool {
        let channels = self.channel_map.get(name);
        match channels {
            Some(_) => { true },
            None => { false }
        }
    }
}