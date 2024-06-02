pub fn linear_search<T: PartialEq>(value: &T, arr: &[T]) -> Option<usize> {
    for i in 0..arr.len() {
        if arr[i] == *value {
            return Some(i);
        }
    }
    None
}

mod tests {
    use super::*;

    #[test]
    fn search_strings() {
        let index = linear_search(&"a", &["a", "b", "c", "d", "google", "zoo"]);
        assert_eq!(index, Some(0));
    }
}
