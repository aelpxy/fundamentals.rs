fn main() {
    let arr = [10, 20, 80, 30, 60, 50, 110, 100, 130, 170];
    let key = 110;
    match linear_search(&arr, key) {
        Some(result) => println!("Key {} is present at index {}", key, result),
        None => println!("Key {} is not present in array". key),
    }
}

fn linear_search(arr: &[i32], key: i32) -> Option<usize> {
    for (i, &v) in arr.iter().enumerate() {
        if v == key {
            return Some(i);
        }
    }
    None
}
