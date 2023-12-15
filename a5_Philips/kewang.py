# 利用touch命令来修改文件的创建时间。
# 规则： 第一集的创建时间最老。 最后一集的创建时间最新。

import os
import json
import time


input_dir = "/Users/lx/Movies/2003"

movies = [os.path.join(input_dir, file) for file in os.listdir(
    input_dir) if os.path.isfile(os.path.join(input_dir, file))]


print(movies)
movies.sort() # 按照字符排序。
# print(movies)

pages = []
# 有隐藏文件。  要忽略隐藏文件。 
for movie in movies:
    count = movie.count(".")
    pages.append(movie) 


for page in pages:
    print(page)  # 01 02 03 ... 
    


from datetime import datetime, timedelta

time_str = "201609271330"
format_str = "%Y%m%d%H%M"

# 将时间字符串转换为datetime对象
start_time = datetime.strptime(time_str, format_str)

# 定义累加的时间间隔
interval = timedelta(seconds=86400)

time_str_list = []

# 依次累加时间并输出结果
for i in range(len(pages)):
    current_time = start_time + i * interval
    print(current_time.strftime(format_str))
    time_str_list.append( current_time.strftime(format_str) )

for ff in time_str_list:
    print(ff)


for i in range(len(pages)):
    cmd = "touch -t yyyymmddhhmm your_file.mp4"
    cmd = "touch -t {} {}".format(time_str_list[i], pages[i])
    print(cmd)
    os.system(cmd)
    print(pages[i])
    
