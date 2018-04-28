use std::sync::{Arc, Mutex, Condvar};
use std::cell::RefCell;
use std::collections::LinkedList;

const DEFAULT_MSG_QUEUE_SIZE: usize = 1024;

#[derive(Copy, Clone)]
enum SignalType {
    Normal
}

struct MessageQueue {
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
/// Message center is thread safe.
pub struct MessageCenter {
    cond_var_pair: Arc<(Mutex<SignalType>, Condvar)>,
    msg_queue_pointer: RefCell<MessageQueue>
}

impl MessageCenter {
    pub fn new() -> Self {
        let msg_queue = MessageQueue::new(DEFAULT_MSG_QUEUE_SIZE);
        let cond_var_pair = (Mutex::<SignalType>::new(SignalType::Normal), Condvar::new());
        let msg_queue_arc_pointer = RefCell::new(msg_queue);
        MessageCenter {
            cond_var_pair: Arc::new(cond_var_pair),
            msg_queue_pointer: msg_queue_arc_pointer
        }
    }

    pub fn new_with_size(size: usize) -> Self {
        let msg_queue = MessageQueue::new(size);
        let cond_var_pair = (Mutex::<SignalType>::new(SignalType::Normal), Condvar::new());
        let msg_queue_arc_pointer = RefCell::new(msg_queue);
        MessageCenter {
            cond_var_pair: Arc::new(cond_var_pair),
            msg_queue_pointer: msg_queue_arc_pointer
        }
    }

    pub fn send_msg(&mut self, msg: u16) {
        let (ref mutex_lock, ref cond_var) = *self.cond_var_pair;
        mutex_lock.lock().unwrap();
        self.msg_queue_pointer.borrow_mut().enqueue_msg(msg);
        cond_var.notify_one();
    }

    pub fn fetch_msg(&mut self) -> u16 {
        let (ref mutex_lock, ref cond_var) = *(self.cond_var_pair);
        let mut sync_start = mutex_lock.lock().unwrap();
        let mut result: u16;
        // loop to avoid spurious wakeup
        loop {
            sync_start = cond_var.wait(sync_start).unwrap();
            if !self.msg_queue_pointer.borrow().is_empty() {
                result = self.msg_queue_pointer.borrow_mut().dequeue_msg().unwrap();
                break;
            }
        }
        result
    }
}