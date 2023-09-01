
在归并排序中警惕：

```rust
    let left = merge_sort(&arr[..mid]);
    let right = merge_sort(&arr[mid..]);

    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
```

复制的时候，我不想深拷贝，这里面有一个写法是深拷贝了， 结果：在新创建的半个数组里自娱自乐，排序结果影响不到分出俩来自的原数组。



# 计数排序
被困扰，先不用泛型了。
```
    let min = *arr.iter().min().unwrap();
    let max = *arr.iter().max().unwrap();
    
    let mut counter_arr = vec![0, (max - min + 1 ) as usize ];
```