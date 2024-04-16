#[derive(Debug)]
pub struct MaxHeap {
    pub heap: Vec<i32>,
}

impl MaxHeap {
    pub fn new() -> MaxHeap {
        MaxHeap { heap: vec![] }
    }

    pub fn parent(&self, index_value: usize) -> Option<usize> {
        if index_value == 0 {
            None
        } else {
            Some((index_value - 1) / 2)
        }
    }

    pub fn left(&self, index_value: usize) -> usize {
        return 2 * index_value + 1;
    }

    pub fn right(&self, index_value: usize) -> usize {
        return 2 * index_value + 2;
    }

    pub fn insert(&mut self, value: i32) {
        self.heap.push(value);

        let mut current = self.heap.len() - 1;

        while let Some(parent) = self.parent(current) {
            if self.heap[parent] < self.heap[current] {
                self.heap.swap(parent, current);
                current = parent;
            } else {
                break;
            }
        }
    }

    pub fn remove_max(&mut self) -> Option<i32> {
        if self.heap.is_empty() {
            return None;
        }
        let max_value = self.heap.swap_remove(0);
        self.heapify_down(0);
        Some(max_value)
    }

    pub fn heapify_down(&mut self, index: usize) {
        let mut current = index;

        loop {
            let left = self.left(current);
            let right = self.right(current);
            let mut largest = current;

            if left < self.heap.len() && self.heap[left] > self.heap[largest] {
                largest = left;
            }

            if right < self.heap.len() && self.heap[right] > self.heap[largest] {
                largest = right;
            }

            if largest != current {
                self.heap.swap(current, largest);
                current = largest;
            } else {
                break;
            }
        }
    }
}
