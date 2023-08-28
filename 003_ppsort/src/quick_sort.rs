


pub fn quick_sort<T: Ord + Clone + std::fmt::Display + std::fmt::Debug>(arr: &mut [T], l: usize, r: usize) {
    if l >= r {
        return;
    }

    let pivot = arr[r].clone();
    let mut reserved_index = l;

    for i in l..r { //[l..r） 左闭右开 NXP 星火讯飞机器人骗老子是闭区间
        if arr[i] < pivot {
            arr.swap(i, reserved_index);
            reserved_index += 1;
        }
    }

    arr.swap(reserved_index, r);

    if reserved_index != l { // reserved_index - 1 >= 0 
        quick_sort(arr, l, reserved_index - 1);
        quick_sort(arr, reserved_index + 1 , r);
    } else {  // 此时reserved_index的左边为空，只需排序右边
        quick_sort(arr, reserved_index + 1 , r);
    }


}