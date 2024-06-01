pub fn quick_sort<T: Ord + Copy>(vec: &mut [T], left: usize, right: usize) {
    if left < right {
        let pivot = partition(vec, left, right);
        quick_sort(vec, left, pivot);
        quick_sort(vec, pivot + 1, right);
    }
}

fn partition<T: Ord + Copy>(vec: &mut [T], mut left: usize, mut right: usize) -> usize {
    let pivot = vec[left];
    while left < right {
        while left < right && vec[right] > pivot {
            right -= 1;
        }
        while left < right && vec[left] < pivot {
            left += 1;
        }
        vec.swap(left, right);
    }
    vec[left] = pivot;
    left
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn Quick_sort_test() {
        let mut vec = vec![1, 23, 4, 2, 77, 12, 888, 45, 6, 343, 33, 09];
        let right: usize = vec.len();
        quick_sort(&mut vec, 0, right - 1);
        println!("{:?}", vec);
    }
}
