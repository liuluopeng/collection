pub fn bubble_sort<T: Ord + Clone + std::fmt::Display + std::fmt::Debug>(arr: &mut [T]) {
    let length = arr.len();
    for _ in 0..length {
        for j in 0..length-1 {
            if arr[j] > arr[j+1] {
                arr.swap(j, j+1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn test_bubble_sort() {
        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();

        let mut arr = Vec::with_capacity(1000000);

        // 生成随机数并将其添加到向量中
        for _ in 0..100 {
            let random_number = rng.gen_range(0..=100000);
            arr.push(random_number);
        }
        bubble_sort(&mut arr);
        println!("{:?}", arr);
        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
