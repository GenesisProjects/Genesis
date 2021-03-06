use std::cell::RefCell;

pub trait Poolable {
    fn empty_obj() -> Self;
    fn unique_id(&self) -> &String;
}

#[derive(Debug)]
pub struct Pool<T: Poolable> {
    max_size: usize,
    num_usage: usize,
    working_pool: Vec<RefCell<T>>,
    recycle_pool: Vec<RefCell<T>>,
}

impl<T: Poolable> Pool<T> {
    pub fn new(max: usize) -> Self {
        let working_pool: Vec<RefCell<T>> = vec![];
        let mut recycle_pool: Vec<RefCell<T>> = vec![];
        for _ in 0usize .. max {
            recycle_pool.push(RefCell::new(T::empty_obj()));
        }
        let new_pool = Pool {
            max_size: max,
            num_usage: 0,
            working_pool: working_pool,
            recycle_pool: recycle_pool
        };
        new_pool
    }

    pub fn obtain(&mut self) -> Option<&RefCell<T>> {
        let new_obj = self.recycle_pool.pop();
        match new_obj {
            None => None,
            Some(r) => {
                self.working_pool.push(r);
                self.num_usage += 1;
                self.working_pool.last()
            }
        }
    }

    pub fn recycle(&mut self, obj: &T) {
        let index = self.working_pool.iter().position(|x: &RefCell<T>| x.borrow().unique_id() == obj.unique_id());
        match index {
            None => (),
            Some(index) => {
                let obj = self.working_pool.remove(index);
                self.working_pool.push(obj);
                self.num_usage -= 1;
            }
        }
    }
}