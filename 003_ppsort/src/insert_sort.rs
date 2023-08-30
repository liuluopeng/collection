

pub fn insert_sort<T: Ord + Clone + std::fmt::Display + std::fmt::Debug >(arr: &mut [T]) {
    let length = arr.len();
    for i in 1..length {
        let mut j = i;
        while j > 0 && arr[j-1] > arr[j] {
            arr.swap(j-1, j);
            j -= 1;
        }
    }
}