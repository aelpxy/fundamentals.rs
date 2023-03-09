fn quicksort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    let pivot = partition(arr);
    quicksort(&mut arr[..pivot]);
    quicksort(&mut arr[pivot + 1..]);
}

fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    arr.swap(pivot_index, len - 1);
    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] < arr[len - 1] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, len - 1);
    i
}

fn main() {
    let mut arr = [3, 2, 1, 5, 4, 10, 6, 8, 9, 7, 0];
    quicksort(&mut arr);
    println!("{:?}", arr);
}
