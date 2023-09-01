

pub fn counting_sort(arr: &mut [usize]) 

{


    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    println!("min: {} max:{}", min, max);
    let mut counter_arr = vec![0 ; (max - min + 1) ];
    println!("{:?}", counter_arr);
    for &i in arr.iter() {
        println!("&i: {}  min:{}", &i, min);
        counter_arr[ i - min  ] += 1;
    }
    println!("{:?}", counter_arr);

    let mut index = 0 as usize;
    
    for i in 0..counter_arr.len() {
        for _ in 0..counter_arr[i] {
            arr[index] = i + min;
            index += 1;
        }
    }
    
}







// fn counting_sort<T: Ord + Copy>(data: &mut [T]) {
//     // 找到数组中的最大值和最小值
//     let min = *data.iter().min().unwrap();
//     let max = *data.iter().max().unwrap();

//     // 创建一个计数数组，用于记录每个元素出现的次数
//     let mut counts = vec![0; (max - min + 1) as usize];

//     // 统计每个元素出现的次数
//     for &item in data.iter() {
//         counts[(item - min) as usize] += 1;
//     }

//     // 根据计数数组重新排列原始数组
//     let mut index = 0;
//     for i in 0..counts.len() {
//         for _ in 0..counts[i] {
//             data[index] = (i as T) + min;
//             index += 1;
//         }
//     }
// }


