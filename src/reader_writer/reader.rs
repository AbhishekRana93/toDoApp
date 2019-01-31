use std::sync::{Arc, Mutex, Condvar};
use std::time::Duration;
use std::thread;


pub struct Reader<'a> {
    shared_content : &'a Arc<Vec<String>>,
    read_count : &'a Arc<(Mutex<i32>, Condvar)>,
    number : i32,
}

impl <'a>Reader<'a> {
    pub fn start_reading(&self) -> thread::JoinHandle<()> {
        let read_count = self.read_count.clone();
        let shared_content = self.shared_content.clone();
        let no = self.number;
        thread::spawn(move|| {

            let (ref count, ref lock) = *read_count;
            let mut count = count.lock().unwrap();
            *count += 1;
            drop(count);

            for i in 0..10 {
                thread::sleep(Duration::from_millis(1000));
                println!("{:?}, {:?}, {:?}",no, " no. reader is reading", *shared_content);

            }

            let (ref count, ref cvar) = *read_count;
            let mut count = count.lock().unwrap();
            println!("{:?}, {:?}, {:?}", " count after reading ", no, *count);
            *count -= 1;
            if *count == 0 {
                cvar.notify_all();
            }
            drop(count);

        })

    }
}


pub fn new<'a>(vec : &'a Arc<Vec<String>>, count : &'a Arc<(Mutex<i32>, Condvar)>, number: i32) -> Reader<'a> {
    Reader {
        shared_content : vec,
        read_count : count,
        number
    }
}
