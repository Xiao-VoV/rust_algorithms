enum Gender {
    Male,
    Female,
    Other,
}
struct Single {
    name: String,
    age: usize,
    gender: Gender,
}

mod GLOBAL_INSTANCE {
    use once_cell::sync::Lazy;
    use std::sync::{Arc, RwLock};
    static GLOBAL_SINGLE: Lazy<RwLock<super::Single>> = Lazy::new(|| {
        RwLock::new(super::Single {
            name: "Tom".to_string(),
            age: 10,
            gender: super::Gender::Male,
        })
    });
}
//使用 lazy_static 社区库
//lazy_static 可以把全局静态变量的初始化推迟到运行时。
//从而是想类似函数初始化的功能。
use lazy_static::lazy_static;
use std::sync::Mutex;
lazy_static! {
    static ref GLOBAL_SINGLE_INSTANCE: Mutex<Single> = Mutex::new(Single {
        name: "Tom".to_string(),
        age: 10,
        gender: Gender::Male,
    });
}

// impl Single{
//     pub fn get_instance()->Arc<RwLock<Single>>
//     {
//         //如果在单线程使用还可以使用Rc+RefCell
//         static mut INSTANCE: Option<Arc<RwLock<Single>>> = None;
//         unsafe {
//            INSTANCE.get_or_insert_with(|| {
//                Arc::new(RwLock::new(Single {
//                    name:"Tom".to_string(),
//                    age:10,
//                    gender:Gender::Male,
//                }))
//            }).clone()
//        }
//     }
// }

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_singleton() {}
}
