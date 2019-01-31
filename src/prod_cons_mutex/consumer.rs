extern crate rand;
use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex, Condvar};

pub struct Consumer<'a> {
    pub count_pair : &'a Arc<(Mutex<i32>, Condvar)>,
    pub buffer : &'a Arc<(Mutex<Vec<i32>>)>,
}

impl<'a> Consumer<'a> {
    pub fn consume(&self) -> thread::JoinHandle<()> {
        let count_clone = self.count_pair.clone();
        let buffer_clone = self.buffer.clone();

        let handle = thread::spawn(move|| {
            loop {
                thread::sleep(Duration::from_millis(800));

                let (ref count, ref condvar) = *count_clone;
                let mut count = count.lock().unwrap();

                if *count != 0 {
                    *count -= 1;
                    let ref buffer = &*buffer_clone;
                    let mut buffer = buffer.lock().unwrap();
                    buffer.pop();
                    println!("{:?} {:?}", "buffer in consumer", *buffer);
                    condvar.notify_one();
                } else {
                    while *count == 0 {
                        count = condvar.wait(count).unwrap();
                    }
                }
            }
        });
        return handle;
    }
}
