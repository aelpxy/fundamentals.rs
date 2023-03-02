fn main() {
    let array = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let target = 10; // array starts at index 0, so it should print 9
    let index = binary_search(&array, target);
    println!("{}", index);
}

fn binary_search(array: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = array.len() - 1;
    while left <= right {
        let middle = (left + right) / 2;
        if array[middle] == target {
            return middle as i32;
        } else if array[middle] < target {
            left = middle + 1;
        } else {
            right = middle - 1;
        }
    }
    -1
}