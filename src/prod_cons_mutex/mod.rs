use std::sync::{Arc, Mutex, Condvar};

mod producer;
mod consumer;

pub fn start() {
    let count_pair = Arc::new((Mutex::new(0), Condvar::new()));
    let buffer = Arc::new(Mutex::new(Vec::new()));

    let prod1 = producer::Producer {
        count_pair : &count_pair,
        buffer : &buffer,
    };

    let prod2 = producer::Producer {
        count_pair : &count_pair,
        buffer : &buffer,
    };

    let cons1 = consumer::Consumer {
        count_pair : &count_pair,
        buffer : &buffer,
    };

    let cons2 = consumer::Consumer {
        count_pair : &count_pair,
        buffer : &buffer,
    };

    let prod_handle1 = prod1.produce();
    let prod_handle2 = prod2.produce();

    let cons_handle = cons1.consume();
    let cons_handle2 = cons2.consume();

    prod_handle1.join().unwrap();
    prod_handle2.join().unwrap();

    cons_handle.join().unwrap();
    cons_handle2.join().unwrap();

}
