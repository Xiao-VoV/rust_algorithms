use std::{iter::Rev, mem::swap};

// 堆排序下标从 1 开始如果从 0 开始则不满足左子树为 2i 的规律
pub fn heap_sort_start_at_1<T: Ord>(vec: &mut [T]) {
    build_heap(vec);
    for i in (1..vec.len()).rev() {
        vec.swap(i, 1);
        heapify(vec, 1, i);
    }
}

fn build_heap<T: Ord>(vec: &mut [T]) {
    let helf = (vec.len()) / 2;
    for i in (1..=helf).rev() {
        heapify(vec, i, vec.len());
    }
}

fn heapify<T: Ord>(vec: &mut [T], i: usize, max_index: usize) {
    let left = i * 2;
    let right = left + 1;
    let mut max;
    if left < max_index && vec[left] > vec[i] {
        max = left;
    } else {
        max = i
    }

    if right < max_index && vec[right] > vec[max] {
        max = right;
    }

    if max != i {
        vec.swap(i, max);
        heapify(vec, max, max_index);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn test_heap_sort() {
        let mut vec = vec![0, 1, 23, 4, 2, 77, 12, 888, 45, 6, 343, 33, 09];
        heap_sort_start_at_1(&mut vec);
        println!("{:?}", vec);
    }
}
