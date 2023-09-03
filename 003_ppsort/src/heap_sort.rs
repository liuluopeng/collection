// 最大堆用的多。
// 0  1     2     3
//  2i+1  2i+2  2i+1

pub fn heap_sort<T: Ord + std::fmt::Debug + std::fmt::Display>(arr: &mut [T]) {
    // 把数组弄成堆
    let length = arr.len();
    for i in (0..=(length / 2)).rev() {
        _heapify(arr, length, i);
    }
    // 到这里  arr是一个最大堆了， 下一步把第一个元素放到最后一个
    println!("max heap:{:?}", arr);

    for i in (1..length).rev() {
        arr.swap(0, i);
        // arr[-i] 不用管，处于正确的位置了。 然后把arr[:i]维护成一个最大堆
        _heapify(arr, i, 0);
    }
}

// n: 原地排序，n用来控制堆能访问的范围。
// i: 原地排序，i表示当前节点的索引。
pub fn _heapify<T: Ord + std::fmt::Display>(arr: &mut [T], n: usize, i: usize) {
    let left_index = 2 * i + 1;
    let right_index = 2 * i + 2;

    let mut bigger_index = i;

    if left_index < n && arr[bigger_index] < arr[left_index] {
        bigger_index = left_index;
    }
    if right_index < n && arr[bigger_index] < arr[right_index] {
        bigger_index = right_index;
    }

    if i != bigger_index {
        println!("i: {} big: {}", arr[i], arr[bigger_index]);
        arr.swap(i, bigger_index);
        _heapify(arr, n, bigger_index);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_heap_sort() {
        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();

        let mut arr = Vec::with_capacity(1000000);

        // 生成随机数并将其添加到向量中
        for _ in 0..1000000 {
            let random_number = rng.gen_range(0..=1000000);
            arr.push(random_number);
        }

        let mut arr = vec![10, 60, 34, 24, 2, 77, 10, 73, 26, 50];

        println!("{:?}", arr);

        heap_sort(&mut arr);

        println!("{:?}", arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}

// use std::cmp::Ordering;

// fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
//     let mut largest = i;
//     let left = 2 * i + 1;
//     let right = 2 * i + 2;

//     if left < n && arr[left] > arr[largest] {
//         largest = left;
//     }

//     if right < n && arr[right] > arr[largest] {
//         largest = right;
//     }

//     if largest != i {
//         arr.swap(i, largest);
//         heapify(arr, n, largest);
//     }
// }

// fn heap_sort<T: Ord>(arr: &mut [T]) {
//     let n = arr.len();

//     for i in (0..=(n / 2)).rev() {
//         heapify(arr, n, i);
//     }

//     for i in (1..n).rev() {
//         arr.swap(0, i);
//         heapify(arr, i, 0);
//     }
// }

// fn main() {
//     let mut arr = [4, 10, 3, 5, 1];
//     heap_sort(&mut arr);
//     println!("Sorted array: {:?}", arr);

//     let mut vec = vec![9, 2, 7, 6, 8];
//     heap_sort(&mut vec);
//     println!("Sorted vector: {:?}", vec);
// }
