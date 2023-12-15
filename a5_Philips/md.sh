#!/bin/bash

# 设置文件夹路径
# folder_path = "/Volumes/Ventoy/还珠格格第1部"
# folder_path = "/Volumes/Untitled/还珠格格第1部"
folder_path = "/Users/lx/Movies/2003"
# 进入文件夹
cd "$folder_path"

# 循环遍历文件夹内的文件
for file in *; do
    # 获取文件名（不包括扩展名）
    filename="${file%.*}"

    # 截取最后两个字符
    new_name="${filename: -2}"

    # 构建新的文件名（保留原始扩展名）
    new_file_name="$new_name.${file##*.}"

    # 重命名文件
    mv "$file" "$new_file_name"
done
