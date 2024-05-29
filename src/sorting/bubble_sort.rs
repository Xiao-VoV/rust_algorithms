pub fn bubble_sort<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: PartialOrd,
{
    for i in 0..vec.len() - 1 {
        for j in i..vec.len() {
            if vec[i] > vec[j] {
                vec.swap(i, j);
            }
        }
    }
    vec
}

#[test]
pub fn bubble_sort_test() {
    let vec = vec![1, 23, 4, 2, 77, 12, 888, 45, 6, 343, 33, 09];
    println! {"{:?}",bubble_sort(vec)};
}
