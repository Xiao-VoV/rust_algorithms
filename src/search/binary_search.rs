pub fn binary_search<T: PartialOrd>(vec: &mut [T], target: &T) -> Option<usize> {
    let mut left: usize = 0;
    let mut right: usize = vec.len() - 1;
    while left <= right {
        let mid = (left + right) / 2;
        if vec[mid] == *target {
            return Some(mid);
        }
        if vec[mid] > *target {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    None
}

pub fn bineary_search(vec: &Vec<i32>, target: i64) -> Result<usize, usize> {
    let mut left: i32 = 0;
    let mut right: i32 = vec.len() as i32 - 1;
    while left <= right {
        let mid = left + (right - left) / 2;
        if (vec[mid as usize] as i64) < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }
    if left <= vec.len() as i32 - 1 {
        Ok(left as usize)
    } else {
        Err(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn binary_search_test() {
        let pos = binary_search(&mut vec![1, 2, 3, 4, 5, 6, 7, 8, 9], &8);
        assert_eq!(pos.unwrap(), 7usize);
    }
}
