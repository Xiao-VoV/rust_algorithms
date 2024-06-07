
enum Gender{
    Male,
    Female,
    Other,
}
struct Single{
    name:String,
    age:usize,
    gender:Gender,
}

mod GLOBAL_INSTANCE{
    use std::sync::{Arc,RwLock};
    use once_cell::sync::Lazy;
    static GLOBAL_SINGLE: Lazy<RwLock<super::Single>> =Lazy::new(|| RwLock::new(super::Single{
        name:"Tom".to_string(),
        age:10,
        gender:super::Gender::Male
    }));
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
mod test{
    use super::*;
    #[test]
    pub fn test_singleton()
    {
        
    }
}