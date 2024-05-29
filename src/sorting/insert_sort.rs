use std::fmt::Display;

pub fn insert_sort<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    for i in 1..vec.len() {
        let target = vec[i];
        for j in (0..i).rev() {
            if vec[j] > target {
                vec.swap(j, j + 1);
                vec[j] = target;
            }
        }
    }
    vec
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn insert_sort_test() {
        let vec = vec![1, 23, 4, 2, 77, 12, 888, 45, 6, 343, 33, 09];
        println! {"{:?}",insert_sort(vec)};
    }
}
