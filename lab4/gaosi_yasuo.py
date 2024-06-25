import os
import time
import ray
import numpy as np
from PIL import Image, ImageFilter
from io import BytesIO

# 初始化Ray
ray.init()

# 定义一个远程函数,用于对单个图像应用高斯模糊
@ray.remote
def apply_gaussian_blur(image_data, radius):
    img = Image.open(BytesIO(image_data))
    blurred_img = img.filter(ImageFilter.GaussianBlur(radius=radius))
    return np.array(blurred_img)

# 定义一个函数,用于对单个图像应用高斯模糊(不使用Ray)
def apply_gaussian_blur_serial(image_path, radius):
    with Image.open(image_path) as img:
        blurred_img = img.filter(ImageFilter.GaussianBlur(radius=radius))
        return np.array(blurred_img)

# 定义一个函数,用于对一批图像应用高斯模糊
def process_images(image_dir, radius):
    image_files = [os.path.join(image_dir, f) for f in os.listdir(image_dir) if f.endswith(".jpg")]
    
    # 读取图像文件内容并使用ray.put()发送给工作进程
    image_data = [ray.put(open(file, 'rb').read()) for file in image_files]
    
    # 使用Ray并行处理图像
    start_time = time.time()
    blurred_images = ray.get([apply_gaussian_blur.remote(data, radius) for data in image_data])
    end_time = time.time()
    
    print(f"使用Ray处理{len(image_files)}张图像耗时: {end_time - start_time:.2f}秒")
    
    # 不使用Ray串行处理图像
    start_time = time.time()
    blurred_images_serial = [apply_gaussian_blur_serial(file, radius) for file in image_files]
    end_time = time.time()
    
    print(f"不使用Ray处理{len(image_files)}张图像耗时: {end_time - start_time:.2f}秒")
    
    return blurred_images

# 指定图像目录和高斯模糊半径
image_dir = "/home/ubuntu/Desktop/image"
radius = 5

# 调用图像处理函数
blurred_images = process_images(image_dir, radius)