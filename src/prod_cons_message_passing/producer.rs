use std::thread;
use std::sync::{Arc, Mutex};

pub struct Producer<'a> {
    pub buffer : &'a Arc<Mutex<Vec<i32>>>,
}

impl <'a>Producer<'a> {
    pub fn start(&self) -> thread::JoinHandle<()> {
        let buffer = self.buffer.clone();
        thread::spawn(move|| {
            println!("{:?}", *buffer);
        })
    }
}
