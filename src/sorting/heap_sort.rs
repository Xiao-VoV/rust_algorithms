use std::{iter::Rev, mem::swap};

// 堆排序下标从 1 开始如果从 0 开始则不满足左子树为 2i 的规律
pub fn heap_sort<T: Ord>(vec: &mut [T]) {
    build_heap(vec);
    for i in (1..vec.len()).rev() {
        vec.swap(i, 1);
        heapify(vec, 1);
    }
}

fn build_heap<T: Ord>(vec: &mut [T]) {
    let helf = vec.len() / 2;
    for i in (1..=helf).rev() {
        heapify(vec, i);
    }
}

fn heapify<T: Ord>(vec: &mut [T], i: usize) {
    let left = i * 2;
    let right = left + 1;
    let mut min;
    if left < vec.len() && vec[left] < vec[i] {
        min = left;
    } else {
        min = i
    }

    if right < vec.len() && vec[right] < vec[i] {
        min = right;
    }

    if min != i {
        vec.swap(i, min);
        heapify(vec, min)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_heap_sort() {
        let mut vec = vec![0, 1, 23, 4, 2, 77, 12, 888, 45, 6, 343, 33, 09];
        heap_sort(&mut vec);
        println!("{:?}", vec);
    }
}
