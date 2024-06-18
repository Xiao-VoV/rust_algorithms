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

//左闭右开区间写法
//最后左右指针都是指向答案，返回任意一个均可
pub fn bineary_search_01(vec: &Vec<i32>, target: i64) -> Result<usize, usize> {
    let mut left: i32 = 0;
    let mut right: i32 = vec.len() as i32; //左闭右开区间
    while left < right
    //左闭右开区间
    {
        let mid = left + (right - left) / 2;
        if (vec[mid as usize] as i64) < target {
            left = mid + 1; //[left,right)
        } else {
            right = mid; //[left ,mid)
        }
    }
    if left <= vec.len() as i32 - 1 {
        Ok(left as usize) // return left 或者 right 都可以
    } else {
        Err(0)
    }
}

//开区间写法
//最后返回 right 是答案
pub fn bineary_search_02(vec: &Vec<i32>, target: i64) -> Result<usize, usize> {
    let mut left: i32 = 0;
    let mut right: i32 = vec.len() as i32; //开区间
    while left + 1 < right
    //开区间
    {
        let mid = left + (right - left) / 2;
        if (vec[mid as usize] as i64) < target {
            left = mid; //(mid,right)
        } else {
            right = mid; //(left ,mid)
        }
    }
    if left <= vec.len() as i32 - 1 {
        Ok(right as usize) // return right
    } else {
        Err(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    pub fn binary_search_test() {
        let pos = binary_search(&mut vec![30, 11, 23, 4, 20], &8);
        assert_eq!(pos.unwrap(), 7usize);
    }
}
