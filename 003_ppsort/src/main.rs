fn main() {
    let mut arr = vec![10, 60, 34, 24, 2, 77, 10, 73, 26, 50];
    heap_sort(&mut arr);
    println!("{:?}", arr); // 输出: [2, 10, 10, 24, 26, 34, 50, 60, 73, 77]
}

fn heap_sort<T: Ord + std::fmt::Display + std::fmt::Debug>(arr: &mut [T]) {
    let len = arr.len();
    println!("arr:{:?}", arr);

    // 构建最大堆
    for i in (0..=len / 2).rev() {
        max_heapify(arr, i, len);
    }

    println!("max heap:{:?}", arr);
    // max heap:[77, 73, 34, 60, 50, 10, 10, 24, 26, 2]
    // 从最后一个元素开始，依次将最大元素交换到数组末尾
    for i in (1..len).rev() {
        arr.swap(0, i); // 将最大元素交换到数组末尾
        max_heapify(arr, 0, i);
    }
}

fn max_heapify<T: Ord+ std::fmt::Display + std::fmt::Debug>(arr: &mut [T], i: usize, heap_size: usize) {
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut largest = i;

    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }

    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        println!("i:{}  larg:{}", arr[i], arr[largest]);
        arr.swap(i, largest);
        max_heapify(arr, largest, heap_size);
    }
}
