mod shared_state_concurrency;
mod thread_message;
mod tiny_threadpool;
use std::{
    thread::{self, sleep},
    time::Duration,
};

pub fn thread_new() {
    let handal = thread::spawn(|| {
        for i in 0..=100 {
            println!("spawn thread -->{i}");
            thread::sleep(Duration::from_millis(20));
        }
    });
    for i in 0..=100 {
        println!("main thread -->{i}");
        thread::sleep(Duration::from_micros(20));
    }
    handal.join();
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_thread() {
        thread_new();
    }
}
