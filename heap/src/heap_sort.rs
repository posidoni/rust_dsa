fn sink<T>(arr: &mut [T], mut k: usize, n: usize)
where
    T: Ord,
    T: Copy,
{
    while 2 * k <= n {
        let mut j = 2 * k;
        if j < n && arr[j] < arr[j + 1] {
            j += 1;
        }
        if arr[k] > arr[j] {
            break;
        }
        (arr[k], arr[j]) = (arr[j], arr[k]);
        k = j;
    }
}

pub fn heapsort<T>(arr: &mut [T])
where
    T: Ord,
    T: Copy,
    T: std::fmt::Debug,
{
    for i in (1..=arr.len() / 2).rev() {
        sink(arr, i, arr.len() - 1);
    }
    let mut n = arr.len() - 1;
    while n > 1 {
        (arr[n], arr[1]) = (arr[1], arr[n]);
        n -= 1;
        sink(arr, 1, n); // heapify smaller part
    }
}
