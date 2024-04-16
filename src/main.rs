mod heap;
mod sorts;

use heap::MaxHeap;
use sorts::{bubble_sort, quick_sort};
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let mut arr1 = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
    let mut arr2 = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];

    bubble_sort::bubble_sort(&mut arr1);
    println!("{:?}", arr1);

    quick_sort::quick_sort(&mut arr2);
    println!("{:?}", arr2);

    let mut max_heap = MaxHeap::new();

    for _ in 0..5 {
        let random_number: i32 = rand();
        max_heap.insert(random_number);
    }

    max_heap.remove_max();

    println!("{:?}", max_heap);
}

fn rand() -> i32 {
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_micros() as u64;
    let mut x = current_time
        .wrapping_mul(0x5DEECE66D)
        .wrapping_add(0xB)
        .wrapping_rem(1 << 48);

    x >>= 16;
    x as i32
}
