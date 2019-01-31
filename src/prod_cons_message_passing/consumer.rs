use std::thread;
use std::sync::{Arc, Mutex};

pub struct Consumer<'a> {
    pub buffer : &'a Arc<Mutex<Vec<i32>>>,
}

impl <'a>Consumer<'a> {
    pub fn start(&self) -> thread::JoinHandle<()> {
        let buffer = self.buffer.clone();
        thread::spawn(|| {
            println!("{:?}", "Consumer");
        })
    }
}
