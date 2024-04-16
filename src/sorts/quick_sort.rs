pub fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();

    if len < 2 {
        return;
    }

    if len < 10 {
        insertion_sort(arr);
        return;
    }

    let pivot = median_of_three(arr);
    let (mut left, mut right) = (0, len - 1);

    loop {
        while arr[left] < pivot {
            left += 1;
        }

        while arr[right] > pivot {
            right -= 1;
        }

        if left >= right {
            break;
        }

        arr.swap(left, right);

        left += 1;
        right -= 1;
    }

    quick_sort(&mut arr[..right + 1]);
    quick_sort(&mut arr[right + 1..]);
}

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let mut j = i;

        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn median_of_three(arr: &mut [i32]) -> i32 {
    let len = arr.len();
    let mid = len / 2;
    let last = len - 1;

    if arr[0] > arr[mid] {
        arr.swap(0, mid);
    }

    if arr[mid] > arr[last] {
        arr.swap(mid, last);
        if arr[0] > arr[mid] {
            arr.swap(0, mid);
        }
    }

    arr[mid]
}
