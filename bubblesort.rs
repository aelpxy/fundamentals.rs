fn main() {
    let mut arr = [9, 5, 4, 1, 6, 3, 8, 7, 10, 2];
    bubble_sort(&mut arr);
    println!("{:?}", arr);
}

fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n - 1 {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}