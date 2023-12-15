import os
import re


input_dir = "/Users/lx/Movies/2003"
movies = [os.path.join(input_dir, file) for file in os.listdir(
    input_dir) if os.path.isfile(os.path.join(input_dir, file))]

print(movies)


for i in movies:
   # 原始文件的完整路径
    old_filepath = i

    # 提取集数
    pattern = r"E(\d+)"
    match = re.search(pattern, old_filepath)
    if match:
        episode_number = match.group(1)
    else:
        print("未找到集数")
        # exit()

    # 提取文件名和目录路径
    old_directory = os.path.dirname(old_filepath)
    old_filename = os.path.basename(old_filepath)

    # 构建新文件名
    new_filename = episode_number + ".mp4"

    # 构建新文件的完整路径
    new_filepath = os.path.join(old_directory, new_filename)

    # 重命名文件
    os.rename(old_filepath, new_filepath)
    print(old_filepath, new_filepath)
    print("文件已重命名为:", new_filename)