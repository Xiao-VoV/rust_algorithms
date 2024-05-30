use std::ops::DerefMut;

//归并排序
pub fn merge_sort<T: Ord + Copy + Sized>(vec: &mut [T]) {
    if vec.len() <= 1 {
        return;
    }
    let mid: usize = vec.len() / 2;
    merge_sort(&mut vec[..mid]);
    merge_sort(&mut vec[mid..]);
    merge(vec, mid);
}

fn merge<T: Ord + Copy>(vec: &mut [T], mid: usize) {
    let left_half = vec[..mid].to_vec();
    let right_half = vec[mid..].to_vec();

    let mut l = 0;
    let mut r = 0;

    for v in vec {
        if r == right_half.len() || (l < left_half.len() && left_half[l] < right_half[r]) {
            *v = left_half[l];
            l += 1;
        } else {
            *v = right_half[r];
            r += 1;
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn merge_sort_test() {
        let mut vec = vec![1, 23, 4, 2, 77, 12, 888, 45, 6, 343, 33, 09];
        merge_sort(&mut vec);
        println!("{:?}", vec);
    }
}
