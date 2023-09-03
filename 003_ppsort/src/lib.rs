mod helper;
mod insert_sort;
mod merge_sort;
mod quick_sort;
mod counting_sort;
mod selection_sort;
mod bubble_sort;
mod heap_sort;
pub use insert_sort::insert_sort;
pub use merge_sort::merge_sort;
pub use quick_sort::quick_sort;
pub use counting_sort::counting_sort;
pub use selection_sort::selection_sort;
pub use bubble_sort::bubble_sort;
pub use heap_sort::heap_sort;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;
  
    #[test]
    fn test_merge_sort() {
        // 创建一个随机数生成器
        let mut rng = rand::thread_rng();

        let mut arr = Vec::with_capacity(1000000);

        // 生成随机数并将其添加到向量中
        for _ in 0..1000000 {
            let random_number = rng.gen_range(0..=100000);
            arr.push(random_number);
        }

        merge_sort(&mut arr);

        for i in 0..arr.len() - 1 {
            assert!(arr[i] <= arr[i + 1]);
        }
    }
}
