use std::{mem::swap, ops::Deref};

pub fn select_sort<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: PartialOrd,
{
    for i in 0..vec.len() - 1 {
        let mut min_index = i;
        for j in i..vec.len() {
            if vec[min_index] > vec[j] {
                min_index = j
            }
        }
        if i != min_index {
            vec.swap(i, min_index);
        }
    }
    vec
}

#[test]
pub fn select_sort_test() {
    let vec = vec![1, 23, 4, 2, 77, 12, 888, 45, 6, 343, 33, 09];
    println! {"{:?}",select_sort(vec)};
}
