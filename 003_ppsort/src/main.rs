use ppsort::quick_sort;
use rand::Rng;

fn main() {
    // 创建一个随机数生成器
    let mut rng = rand::thread_rng();

    let mut numbers = Vec::with_capacity(1000000);

    // 生成随机数并将其添加到向量中
    for _ in 0..1000000 {
        let random_number = rng.gen_range(0..=100000);
        numbers.push(random_number);
    }

    let len = numbers.len();

    quick_sort(&mut numbers, 0, len-1);
    println!("{:?}", numbers);
}
