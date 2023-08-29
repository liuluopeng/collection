pub fn merge_sort<T: Ord + Clone + std::fmt::Display + std::fmt::Debug>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let len = arr.len();
    let mid = len / 2;

    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);

    __merge(arr, mid);
}

fn __merge<T: Ord + Clone + std::fmt::Display + std::fmt::Debug>(arr: &mut [T], mid: usize) {
    let mut aux_vec = Vec::with_capacity(arr.len());

    let (mut i, mut j) = (0 as usize, mid);

    while i < mid && j < arr.len() {
        if arr[i] < arr[j] {
            aux_vec.push(arr[i].clone());
            i += 1;
        } else {
            aux_vec.push(arr[j].clone());
            j += 1;
        }
    }

    aux_vec.extend_from_slice(&arr[i..mid]);
    aux_vec.extend_from_slice(&arr[j..]);

    for i in 0..arr.len() {
        arr[i] = aux_vec[i].clone();
    }
}
