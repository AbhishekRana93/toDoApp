use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use std::thread;

pub struct Writer<'a> {
    shared_content : &'a Arc<Vec<String>>,
    read_count : &'a Arc<(Mutex<i32>, Condvar)>,
}

impl <'a> Writer<'a> {
    pub fn start_writing(&self) -> thread::JoinHandle<()> {
        let shared_content = self.shared_content.clone();
        let read_count = self.read_count.clone();

        thread::spawn(move|| {

            let (ref count, ref cvar) = *read_count;
            let mut count = count.lock().unwrap();

            while *count != 0 {
                count = cvar.wait(count).unwrap();
            }

            println!("{:?}", "writer is writing");

            //update content here

        })
    }
}

pub fn new<'a>(vec : &'a Arc<Vec<String>>, count : &'a Arc<(Mutex<i32>, Condvar)>) -> Writer<'a> {
    Writer {
        shared_content : vec,
        read_count : count,
    }
}
