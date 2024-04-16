pub fn linear_search(arr: &[i32], key: i32) -> Option<usize> {
    for (i, &v) in arr.iter().enumerate() {
        if v == key {
            return Some(i);
        }
    }
    
    None
}
