pub fn selection_sort<T: Ord + Clone + std::fmt::Display + std::fmt::Debug>(arr: &mut [T]) {
    if arr.len() == 0 {
        return ;
    }

    for i in 0..arr.len() {
        let mut min_index = i;
        for j in i + 1..arr.len() {
            if arr[min_index] > arr[j] {
                min_index = j;
            }
        }
        arr.swap(min_index, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_selection_sort() {
        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();

        let mut arr: Vec<i32> = Vec::with_capacity(1000000);

        // 生成随机数并将其添加到向量中
        for _ in 0..120 {
            let random_number = rng.gen_range(0..=100000);
            arr.push(random_number);
        }

        selection_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
