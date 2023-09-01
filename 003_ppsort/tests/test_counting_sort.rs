use ppsort::counting_sort;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_counting_sort() {
        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();

        let mut arr = Vec::with_capacity(100);

        // 生成随机数并将其添加到向量中
        for _ in 0..100 {
            let random_number = rng.gen_range(0..=100);
            arr.push(random_number);
        }

        println!("{:?}", arr);
        counting_sort(&mut arr);
        println!("{:?}", arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }

}