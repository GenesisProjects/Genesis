#[macro_use]
extern crate lazy_static;
extern crate rand;

use std::sync::{Arc, Mutex, Condvar};
use std::cell::RefCell;
use std::collections::{LinkedList, HashMap};

const DEFAULT_MSG_QUEUE_SIZE: usize = 1024;

lazy_static! {
    pub static ref MESSAGE_CENTER: Mutex<MessageCenter> = {
        Mutex::new(MessageCenter::new())
    };
}

fn random_string(length: usize) -> String {
    use rand::Rng;
    let result = rand::thread_rng()
        .gen_ascii_chars()
        .take(length)
        .collect::<String>();
    result
}

///
#[derive(Debug, Clone)]
pub struct Message {
    op: u16,
    msg: String
}

///
pub struct MessageQueue {
    queue: LinkedList<Message>,
    limit: usize
}

impl MessageQueue {
    pub fn new(limit: usize) -> Self {
        MessageQueue {
            queue: LinkedList::<Message>::new(),
            limit: limit
        }
    }

    pub fn enqueue_msg(&mut self, msg: Message) -> Result<usize, &'static str> {
        let cur_size = self.queue.len();
        if cur_size >= self.limit {
            Err("The message queue is full.")
        } else {
            self.queue.push_back(msg);
            Ok(cur_size + 1)
        }
    }

    pub fn dequeue_msg(&mut self) -> Option<Message> {
        self.queue.pop_front()
    }

    pub fn flush(&mut self) -> LinkedList<Message> {
        let mut result = LinkedList::<Message>::new();
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

pub struct MessageChannel {
    pub uid: String,
    pub queue: RefCell<MessageQueue>
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
        MessageChannel {
            uid: random_string(32),
            queue: RefCell::new(msg_queue)
        }
    }

    pub fn new_with_size(size: usize) -> Self {
        let msg_queue = MessageQueue::new(size);
        MessageChannel {
            uid: random_string(32),
            queue: RefCell::new(msg_queue)
        }
    }

    pub fn send_msg(&mut self, msg: Message) -> Result<usize, &'static str> {
        let result = self.queue.get_mut().enqueue_msg(msg);
        result
    }

    pub fn send_msg_without_cache(&mut self, msg: Message) -> Result<usize, &'static str> {
        self.flush();
        self.send_msg(msg)
    }

    pub fn accept_msg_async(&mut self) -> Option<Message> {
        self.queue.get_mut().dequeue_msg().to_owned()
    }

    pub fn flush(&mut self) -> LinkedList<Message> {
        let result = self.queue.get_mut().flush();
        result
    }
}

pub struct MessageCenter {
    channel_map: HashMap<String, Vec<Arc<(Mutex<MessageChannel>, Condvar)>>>
}


impl MessageCenter {
    pub fn new() -> Self {
        MessageCenter {
            channel_map: HashMap::<String, Vec<Arc<(Mutex<MessageChannel>, Condvar)>>>::new()
        }
    }

    pub fn subscribe(&mut self, name: &String) -> Arc<(Mutex<MessageChannel>, Condvar)> {
        let existed = self.channels_exist_by_name(name);
        if existed {
            let new_channel = MessageChannel::new();
            let chs = self.channel_map.get_mut(name).unwrap();
            chs.push(Arc::new((Mutex::new(new_channel), Condvar::new() )));
            chs.last_mut().unwrap().clone()
        } else {
            let new_channel = MessageChannel::new();
            self.channel_map.insert(name.to_owned(), vec![Arc::new((Mutex::new(new_channel), Condvar::new()))]);
            let chs = self.channel_map.get_mut(name).unwrap();
            chs.last_mut().unwrap().clone()
        }
    }

    pub fn unsubscribe(&mut self, name: &String, uid: String) {
        let existed = self.channels_exist_by_name(name);
        if existed {
            let chs = self.channel_map.get_mut(name).unwrap();
            let pos = chs.iter().position(|x| (&((*x).0)).lock().unwrap().uid == uid);
            pos.and_then(|r| { chs.remove(r); Some(r) });
        }

        let chs_len = self.channel_map.get_mut(name).unwrap().len();
        if chs_len == 0 {
            self.channel_map.remove(name);
        }
    }

    pub fn send(&mut self, name: &String, msg: Message) {
        let existed = self.channels_exist_by_name(name);
        if !existed {
            return;
        } else {
            let channels = self.channel_map.get_mut(name).unwrap();
            for i in 0 .. channels.len() {
                let mut ch = channels[i].clone();
                ch.0.lock().unwrap().send_msg(msg.to_owned());
            }
        }
    }

    pub fn channels_by_name(&mut self, name: &String) -> Vec<Arc<(Mutex<MessageChannel>, Condvar)>> {
        let existed = self.channels_exist_by_name(name);
        if !existed {
            vec![]
        } else {
            self.channel_map.get_mut(name).unwrap().clone()
        }
    }

    pub fn channels_exist_by_name(&self, name: &String) -> bool {
        let channels = self.channel_map.get(name);
        match channels {
            Some(_) => true,
            None => false
        }
    }
}