use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

mod producer;
mod consumer;

pub fn start() {

    let buffer = Arc::new(Mutex::new(Vec::new()));
    // let producer1 = producer::Producer {
    //     buffer : &buffer
    // };
    //
    // let prod_handle = producer1.start();
    //
    // prod_handle.join().unwrap();
    let (ptx, prx) = mpsc::channel();
    let (ctx, crx) = mpsc::channel();

    println!("{:?}", "Yay");
    let buffer1 = buffer.clone();
    let prod = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000));
            let mut buffer = buffer1.lock().unwrap();
            buffer.push(1);
            println!("{:?}, {:?}", "Buffer in producer", *buffer);
            ptx.send("send".to_string()).unwrap();
            drop(buffer);
            crx.recv().unwrap();
        }
    });

    let buffer2 = buffer.clone();
    let cons = thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(1000));

            let recv = prx.recv().unwrap();
            let mut buffer = buffer2.lock().unwrap();
            buffer.pop();
            println!("{:?}, {:?}", "Buffer in consumer 1", *buffer);
            ctx.send("Push this".to_string()).unwrap();
        }
    });

    prod.join().unwrap();
    cons.join().unwrap();

}
