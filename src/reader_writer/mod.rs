use std::sync::{Arc, Mutex, Condvar};

mod reader;
mod writer;

pub fn start() {
    let contents = vec!["db".to_string(), "contents".to_string(), "to".to_string(),
        "read".to_string()];
    let shared_content = Arc::new(contents);
    let read_count = Arc::new((Mutex::new(0), Condvar::new()));

    let writer1 = writer::new(&shared_content, &read_count);
    let writer1_handle = writer1.start_writing();
    writer1_handle.join().unwrap();
    let reader1 = reader::new(&shared_content, &read_count, 1);
    let reader2 = reader::new(&shared_content, &read_count, 2);
    let reader3 = reader::new(&shared_content, &read_count, 3);
    let reader4 = reader::new(&shared_content, &read_count, 4);

    let reader1_handle = reader1.start_reading();
    let reader2_handle = reader2.start_reading();
    let reader3_handle = reader3.start_reading();
    let reader4_handle = reader4.start_reading();

    reader1_handle.join().unwrap();
    reader2_handle.join().unwrap();
    reader3_handle.join().unwrap();
    reader4_handle.join().unwrap();

    // let buffer1 = buffer.clone();
    // let reader1 = thread::spawn(move|| {
    //     loop {
    //         println!("{:?}", "Reader 1 entered");
    //
    //         thread::sleep(Duration::from_millis(1000));
    //         let buffer = buffer1.lock().unwrap();
    //         println!("{:?}, {:?}", "Reading in 1: ", *buffer);
    //
    //         println!("{:?}", "Reader 1 left");
    //     }
    // });
    //
    // let buffer2 = buffer.clone();
    // let reader2 = thread::spawn(move|| {
    //     loop {
    //         println!("{:?}", "Reader 2 entered");
    //
    //         thread::sleep(Duration::from_millis(1000));
    //         let buffer = buffer2.lock().unwrap();
    //         println!("{:?}, {:?}", "Reading in 2: ", *buffer);
    //
    //         println!("{:?}", "Reader 2 left");
    //
    //     }
    // });
    //
    // let buffer3 = buffer.clone();
    // let reader3 = thread::spawn(move|| {
    //     loop {
    //         println!("{:?}", "Reader 3 entered");
    //
    //         thread::sleep(Duration::from_millis(1000));
    //         let buffer = buffer3.lock().unwrap();
    //         println!("{:?}, {:?}", "Reading in 3: ", *buffer);
    //
    //         println!("{:?}", "Reader 3 left");
    //
    //     }
    // });
    //
    // let buffer4 = buffer.clone();
    // let writer = thread::spawn(move|| {
    //     loop {
    //         println!("{:?}", "Writer 1 entered");
    //
    //         thread::sleep(Duration::from_millis(1000));
    //         let mut buffer = buffer4.lock().unwrap();
    //         *buffer = "bla bla".to_string();
    //         println!("{:?}, {:?}", "Updated var to : ", *buffer);
    //
    //         println!("{:?}", "Writer 1 left");
    //
    //     }
    // });
    //
    // reader1.join().unwrap();
    // reader2.join().unwrap();
    // reader3.join().unwrap();
    //
    // writer.join().unwrap();

}
