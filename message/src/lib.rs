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

    pub fn send_msg(&mut self, msg: Message) -> Result<usize, &'static str> {
        let (ref mutex_lock, ref cond_var) = *self.cond_var_pair;
        let queue_ref = mutex_lock.lock().unwrap();
        let result = queue_ref.borrow_mut().enqueue_msg(msg);
        cond_var.notify_all();
        result
    }

    pub fn send_msg_without_cache(&mut self, msg: Message) -> Result<usize, &'static str> {
        self.flush();
        self.send_msg(msg)
    }

    pub fn accept_msg(&mut self) -> Message {
        let (ref mutex_lock, ref cond_var) = *(self.cond_var_pair);
        let mut queue = mutex_lock.lock().unwrap();
        // if the queue is not empty
        if !queue.borrow().is_empty() {
            return queue.borrow_mut().dequeue_msg().unwrap();
        }
        // else wait for new msg
        let result: Message;
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

    pub fn accept_msg_async(&mut self) -> Option<Message> {
        let (ref mutex_lock, _) = *(self.cond_var_pair);
        let queue = mutex_lock.lock().unwrap();
        let mut queue_ref = queue.borrow_mut();
        queue_ref.dequeue_msg().to_owned()
    }

    pub fn flush(&mut self) -> LinkedList<Message> {
        let (ref mutex_lock, _) = *(self.cond_var_pair);
        let queue_lock = mutex_lock.lock().unwrap();
        let mut queue_ref = queue_lock.borrow_mut();
        let result = queue_ref.flush();
        result
    }
}

pub struct MessageCenter {
    channel_map: HashMap<String, Vec<Arc<Mutex<MessageChannel>>>>
}


impl MessageCenter {
    pub fn new() -> Self {
        MessageCenter {
            channel_map: HashMap::<String, Vec<Arc<Mutex<MessageChannel>>>>::new()
        }
    }

    pub fn subscribe(&mut self, name: &String) -> Arc<Mutex<MessageChannel>> {
        let existed = self.channels_exist_by_name(name);
        if existed {
            let new_channel = MessageChannel::new();
            let chs = self.channel_map.get_mut(name).unwrap();
            chs.push(Arc::new(Mutex::new(new_channel)));
            chs.last_mut().unwrap().clone()
        } else {
            let new_channel = MessageChannel::new();
            self.channel_map.insert(name.to_owned(), vec![Arc::new(Mutex::new(new_channel))]);
            let chs = self.channel_map.get_mut(name).unwrap();
            chs.last_mut().unwrap().clone()
        }
    }

    pub fn unsubscribe(&mut self, name: &String, ch: Arc<Mutex<MessageChannel>>) {
        let existed = self.channels_exist_by_name(name);
        if existed {
            let chs = self.channel_map.get_mut(name).unwrap();
            let uid = ch.lock().unwrap().uid.clone();
            let pos = chs.iter().position(|x| x.lock().unwrap().uid == uid);
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
                ch.lock().unwrap().send_msg(msg.to_owned());
            }
        }
    }

    pub fn channels_by_name(&mut self, name: &String) -> Vec<Arc<Mutex<MessageChannel>>> {
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
            Some(_) => { true },
            None => { false }
        }
    }
}