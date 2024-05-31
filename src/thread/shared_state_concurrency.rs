use std::{
    error,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};
#[derive(Debug)]
enum Gender {
    male,
    female,
    other,
}
#[derive(Debug)]
pub struct Person {
    name: String,
    age: u8,
    gender: Gender,
}

pub fn shared_state_concurrency() {
    let person = Arc::new(Mutex::new(Person {
        name: "Tom".to_string(),
        age: 20,
        gender: Gender::male,
    }));
    let mut handles = Vec::<JoinHandle<()>>::new();
    for _ in 0..10 {
        let counter = person.clone();
        let handle = thread::spawn(move || {
            let mut p = counter.lock().unwrap();
            p.age += 1;
        });
        handles.push(handle);
    }
    for i in handles {
        i.join();
    }
    println!("{:?}", person.lock().unwrap());
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_shared_state() {
        shared_state_concurrency();
    }
}
