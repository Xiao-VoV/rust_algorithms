use std::{
    sync::mpsc::{self, RecvError},
    thread,
};

pub fn message_send_rev() {
    //multiple producer, single consumer
    let (tx, tr) = mpsc::channel();
    let tx2 = tx.clone();
    let handle1 = thread::spawn(move || {
        for i in 0..=100 {
            let message: String = format!(" {i}_1");
            println!("send-->{message}");
            tx2.send(message).unwrap();
        }
    });
    // loop {
    // match tr.recv() {
    //     Ok(message) => println!("receive<-- {} \n ", message),
    //     _ => break,
    // }
    //try_recv 不会挂起当前线程,sen会转移所有权

    // match tr.try_recv() {
    //     Ok(message) => println!("receive<-- {} \n ", message),
    //     Err(err) => println!("{:?}", err),
    // }
    // }
    let handle2 = thread::spawn(move || {
        for i in 0..=100 {
            let message: String = format!(" {i}_2");
            println!("send-->{message}");
            tx.send(message).unwrap();
        }
    });
    for received in tr {
        println!("Got: {}", received);
    }
    handle1.join().unwrap();
    handle2.join().unwrap();
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn thread_message_test() {
        message_send_rev();
    }
}
