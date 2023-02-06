#[cfg(test)]
mod tests {
    use std::collections::binary_heap::BinaryHeap;

    #[test]
    fn simulate_heap_client() {
        let mut heap = heap::heap::Heap::new();
        let mut stdheap = BinaryHeap::<i32>::new();

        for (i, j) in vec![(1, 1), (2, 2), (3, 3), (4, 4), (-1, 4)] {
            heap.insert(i);
            stdheap.push(i);
            assert_eq!(heap.peek().unwrap(), j);
        }

        for i in vec![4, 3, 2, 1, -1] {
            assert_eq!(heap.pop().unwrap(), i);
            assert_eq!(stdheap.pop(), Some(i));
        }
        assert_eq!(heap.size(), 0)
    }

    #[test]
    fn heapify_test() {
        let mut unsorted = vec![5, 2, 3, 1, 4];
        let sorted = vec![1, 2, 3, 4, 5];
        heap::heap_sort::heapsort(&mut unsorted);
        assert_eq!(unsorted, sorted);

        let mut unsorted = vec![1, 2, 3, 4, 5];
        let sorted = vec![1, 2, 3, 4, 5];
        heap::heap_sort::heapsort(&mut unsorted);
        assert_eq!(unsorted, sorted);

        let mut unsorted = vec![1, 5, 8, 9, 3, 4, 5, 5, 0];
        let sorted = vec![0, 1, 3, 4, 5, 5, 5, 8, 9];
        heap::heap_sort::heapsort(&mut unsorted);
        assert_eq!(unsorted, sorted);
    }
}
