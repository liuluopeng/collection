import os
import subprocess

def convert_mkv_to_mp4(folder_path, output_path):
    # 获取文件夹中的所有文件
    files = os.listdir(folder_path)

    for file in files:
        if file.endswith(".mkv"):
            # 构建输入和输出文件路径
            input_file = os.path.join(folder_path, file)
            output_file = os.path.join(output_path, os.path.splitext(file)[0] + ".mp4")

            # 使用ffmpeg进行转换
            subprocess.run(["ffmpeg", "-i", input_file, output_file])

            # 如果需要，可以在转换完成后删除原始的.mkv文件
            # os.remove(input_file)

    print("转换完成！")

# 指定文件夹路径
folder_path = "/Users/lx/Movies/还珠格格3.2003.40集全.国语.繁体中字￡CMCT小鱼"
output_path = "/Users/lx/Movies/2003"
# 调用函数进行转换
convert_mkv_to_mp4(folder_path, output_path)
