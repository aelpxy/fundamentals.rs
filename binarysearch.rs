fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let key = 2;
    match binary_search(&arr, key) {
        Some(result) => println!("Key {} is present at index {}", key, result),
        None => println!("Key {} is not present in array", key),
    }
}

fn binary_search(arr: &[i32], key: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high {
        let mid = (low + high) / 2;
        match arr[mid].cmp(&key) {
            std::cmp::Ordering::Less => low = mid + 1,
            std::cmp::Ordering::Greater => high = mid - 1,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}
