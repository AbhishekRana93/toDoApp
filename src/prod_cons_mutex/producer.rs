extern crate rand;

use std::thread;
use std::time::Duration;
use rand::Rng;
use std::sync::{Arc, Mutex, Condvar};

pub struct Producer<'a>{
    pub count_pair : &'a Arc<(Mutex<i32>, Condvar)>,
    pub buffer : &'a Arc<(Mutex<Vec<i32>>)>,
}

impl<'a> Producer<'a> {
    pub fn produce(&self) -> thread::JoinHandle<()> {
        let count_clone = self.count_pair.clone();
        let buffer_clone = self.buffer.clone();

        let handle = thread::spawn(move|| {
            loop {
                thread::sleep(Duration::from_millis(800));

                let (ref count, ref condvar) = *count_clone;
                let mut count = count.lock().unwrap();

                if *count != 10 {
                    let x = rand::thread_rng().gen_range(1,10);
                    let ref buffer = &*buffer_clone;
                    let mut buffer = buffer.lock().unwrap();
                    buffer.push(x);
                    println!("{:?} {:?}", "buffer in producer", *buffer);
                    *count += 1;
                    condvar.notify_one();
                } else {
                    while *count == 10 {
                        count = condvar.wait(count).unwrap();
                    }
                }

            }
        });
        return handle;
    }
}
