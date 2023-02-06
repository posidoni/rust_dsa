/// # Heap
/// Rust stdlib alternative: `std::collections::BinaryHeap`
/// Implements max heap with complete binary tree (every row is full
/// except for lowest row). Tree is filled from left to right.
/// No child has a key less then its parent key.
///
/// ## Indexing rules:
/// Node's parent is floor(i/2)
/// Node's left/right children are 2i / 2i + 1 (starting from 1).
#[derive(Default, Debug)]
pub struct Heap<T> {
    heap: Vec<T>,
}

impl<T> std::ops::IndexMut<usize> for Heap<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.heap[index]
    }
}

impl<T> std::ops::Index<usize> for Heap<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.heap[index]
    }
}

impl<T> Heap<T>
where
    T: Copy,
    T: Default,
    T: Ord,
{
    /// Constructs new empty heap.
    pub fn new() -> Heap<T> {
        Heap::<T> {
            heap: vec![T::default()],
        }
    }

    pub fn heapify(vec: Vec<T>) -> Heap<T> {
        let mut h = Heap::new();
        h.heap = vec;
        for i in (1..=h.heap.len() / 2).rev() {
            h.sink(i);
        }
        return h;
    }

    /// Inserts iter into heap.
    /// O(log(n))
    pub fn insert(&mut self, val: T) {
        self.heap.push(val); // Push to the first free position in tree
        self.swim(self.heap.len() - 1); // Reheapify
    }

    /// Removes greatest element of the heap
    /// O(2 * log(n)), because children also must be compared
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.len() == 1 {
            return None;
        }
        let res = self[1];
        self[1] = self[self.heap.len() - 1];
        self.heap.pop(); // remove the largest element

        self.sink(1);

        return Some(res);
    }

    /// Returns lowerst element of the heap
    /// O(1)
    pub fn peek(&self) -> Option<T> {
        if self.heap.len() > 1 {
            Some(self.heap[1])
        } else {
            None
        }
    }

    pub fn size(&self) -> usize {
        self.heap.len() - 1
    }

    pub fn is_empty(&self) -> bool {
        self.heap.len() == 0
    }

    fn swim(&mut self, mut k: usize) {
        while k > 1 && self.heap[k / 2] < self[k] {
            (self[k / 2], self[k], k) = (self[k], self[k / 2], k / 2)
        }
    }

    fn sink(&mut self, mut k: usize) {
        while 2 * k < self.heap.len() {
            let mut child_to_swap = 2 * k;

            if 2 * k + 1 < self.heap.len() && self.heap[2 * k] < self.heap[2 * k + 1] {
                child_to_swap += 1;
            }

            if self[k] < self[child_to_swap] {
                (self[k], self[child_to_swap]) = (self[child_to_swap], self[k]);
                k = child_to_swap;
            } else {
                break;
            }
        }
    }
}
