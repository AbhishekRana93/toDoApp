extern crate rand;
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::sync::{Arc, Mutex, Condvar};

struct Producer<'a>{
    count_pair : &'a Arc<(Mutex<i32>, Condvar)>,
    buffer : &'a Arc<(Mutex<Vec<i32>>)>,
}

impl<'a> Producer<'a> {
    fn produce(&self) -> thread::JoinHandle<()> {
        let count_clone = self.count_pair.clone();
        let buffer_clone = self.buffer.clone();

        let handle = thread::spawn(move|| {
            loop {
                thread::sleep_ms(500);

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
                    let mut count = condvar.wait(count).unwrap();
                }

            }
        });
        return handle;
    }
}

struct Consumer<'a> {
    count_pair : &'a Arc<(Mutex<i32>, Condvar)>,
    buffer : &'a Arc<(Mutex<Vec<i32>>)>,
}

impl<'a> Consumer<'a> {
    fn consume(&self) -> thread::JoinHandle<()> {
        let count_clone = self.count_pair.clone();
        let buffer_clone = self.buffer.clone();

        let handle = thread::spawn(move|| {
            loop {
                println!("{:?} {:?}", "buffer in consumer", "inside");

                thread::sleep_ms(1000);

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
                    println!("{:?} {:?}", "buffer in consumer", "waiting");
                    let mut count = condvar.wait(count).unwrap();
                }
            }
        });
        return handle;
    }
}


fn main() {
    let count_pair = Arc::new((Mutex::new(0), Condvar::new()));
    let buffer = Arc::new(Mutex::new(Vec::new()));

    let prod1 = Producer {
        count_pair : &count_pair,
        buffer : &buffer,
    };

    let cons1 = Consumer {
        count_pair : &count_pair,
        buffer : &buffer,
    };

    let cons2 = Consumer {
        count_pair : &count_pair,
        buffer : &buffer,
    };

    let prod_handle = prod1.produce();
    let cons_handle = cons1.consume();
    let cons_handle2 = cons2.consume();

    prod_handle.join().unwrap();
    cons_handle.join().unwrap();
    cons_handle2.join().unwrap();

}
