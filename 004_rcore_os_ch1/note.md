http://rcore-os.cn/rCore-Tutorial-Book-v3/chapter1/2remove-std.html

leonhxx commented on 2021年4月27日
使用清华网盘镜像里的Ubuntu时，加上#![no_std] 来告诉 Rust 编译器不使用 Rust 标准库 std后，报的错误和书里的有点不一样，我这儿的异常信息如下，请问还需要安装软件么？
error[E0463]: can't find crate for core
|
= note: the riscv64gc-unknown-none-elf target may not be installed

error: aborting due to previous error

For more information about this error, try rustc --explain E0463.
error: could not compile os

@wyfcyx
wyfcyx commented on 2021年4月27日
Collaborator
@leonhxx 请先输入以下命令安装一下相关软件：

rustup target add riscv64gc-unknown-none-elf
cargo install cargo-binutils --vers ~0.2
rustup component add llvm-tools-preview
rustup component add rust-src



# 大端序 小端序
## GPT 
大端序和小端序， 谁看上去像是反了？

大端序和小端序是指在计算机中存储多字节数据时，字节的顺序排列方式。

在大端序（Big Endian）中，最高有效字节（Most Significant Byte，MSB）存储在最低的内存地址，而最低有效字节（Least Significant Byte，LSB）存储在最高的内存地址。这类似于数字的书写方式，最高位在左边，最低位在右边。

在小端序（Little Endian）中，最低有效字节（LSB）存储在最低的内存地址，而最高有效字节（MSB）存储在最高的内存地址。这种方式看起来好像反了，因为我们习惯从左到右读取数字，而不是从右到左。

因此，相对于我们习惯的数字书写方式，小端序看起来像是反了。大部分个人电脑和服务器使用的是小端序，而一些嵌入式系统和网络协议则使用大端序。

## CSDN
数据存储：小端模式和大端模式——终于搞明白了！！！_大端模式和小端模式_风兔子喵的博客-CSDN博客 
https://blog.csdn.net/weixin_45633061/article/details/117421452 

## 我怎么记忆：

给我从左到右的书写方向建立坐标轴， 以我上学的时候常见的x正方向（从左到右）为正方向， 那么当我写出100这个数字的时候， 我采用的是大端序。 



# 2023年08月27日 晚上  我的结果：
Breakpoint 1, 0x0000000080200000 in stext ()
(gdb) x/5i $pc
=> 0x80200000 <stext>:  li      ra,100
   0x80200004:  unimp
   0x80200006:  unimp
   0x80200008:  unimp
   0x8020000a:  unimp
(gdb) p/d $x1
$7 = 2147495338
(gdb) p/x $sp
$8 = 0x0
(gdb) si
0x0000000080200004 in ?? ()
(gdb) p/d $x1
$9 = 100
(gdb) p/x $sp
$10 = 0x0
(gdb) 

# 剪裁
rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin


# 2023年08月27日
今天报错， 教程问答区有说把rust换成nightly版本，  但是因为nightly是开发板我想要稳定版所以我不敢做（开发板的新特性能有机会让我困扰吗）。  后来问chatGPT， 说rustup的rust的版本管理工具， 也就是说， 换版本不会像换python版本那样， 把电脑搞脏。 所以我换了， 然后安装riscv64gc-unknown-none-elf之类的， 然后就成功打印出hello world！

