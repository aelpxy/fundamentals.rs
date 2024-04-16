pub fn bubble_sort(arr: &mut [i32]) {
    let mut n = arr.len();
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..n - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        
        n -= 1;
    }
}
