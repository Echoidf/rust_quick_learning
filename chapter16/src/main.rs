use std::{sync::{mpsc, Arc, Mutex}, thread};

fn main() {
    let v = vec![1,2,3];
    let handler = thread::spawn(move || {
        println!("{:?}",v);
    });
    // 等待子线程结束,阻塞当前线程直到子线程完成
    handler.join().unwrap();
}

#[test]
fn test_channel() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello").unwrap();
    });

    let msg = rx.recv().unwrap();
    println!("Got: {msg}");
}

#[test]
fn test_mutex() {
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];
    //启动十个线程，并在各个线程中对同一个计数器值加一，这样计数器将从 0 变为 10
    for _i in 0..10 {
        let counter = Arc::clone(&counter);
        let handler = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;

        });
        handlers.push(handler);
    }

    for handler in handlers {
        handler.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}