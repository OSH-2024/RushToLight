import os
import time
import ray
from PIL import Image
from glob import glob
from io import BytesIO  
import psutil
import numpy as np
from torchvision import datasets
import torchvision.transforms as transforms
import matplotlib.pyplot as plt

# 初始化Ray
ray.init(num_cpus=8,num_gpus=0,object_store_memory=3e9)

# 将图像保存到本地目录
def save_images(dataset, save_dir, num_images):
    for i in range(num_images):
        img, label = dataset[i]
        img = transforms.ToPILImage()(img)
        img.save(os.path.join(save_dir, f'cifar10_image_{i}.jpg'))

# 定义图像压缩函数
def compress_image(image_data, output_path):
    img = Image.open(BytesIO(image_data))
    img = img.resize((256, 256), Image.BICUBIC)
    img.save(output_path, "JPEG", quality=85)

# 定义远程函数，用于图像压缩
@ray.remote
def ray_compress_image(image_data, output_path):
    compress_image(image_data, output_path)

# 性能测量函数
def measure_performance(image_paths, output_pattern, use_ray=False):
    start_time = time.time()
    processed_count = 0

    if use_ray:
        tasks = []
        for i, image_path in enumerate(image_paths):
            output_path = output_pattern.format(i)
            image_data = open(image_path, 'rb').read()
            tasks.append(ray_compress_image.remote(image_data, output_path))
        
        # 等待所有任务完成
        ray.get(tasks)
        processed_count = len(tasks)
    else:
        for i, image_path in enumerate(image_paths):
            output_path = output_pattern.format(i)
            image_data = open(image_path, 'rb').read()
            compress_image(image_data, output_path)
            processed_count += 1

    end_time = time.time()
    elapsed_time = end_time - start_time

    # 计算吞吐量
    throughput = processed_count / elapsed_time if elapsed_time > 0 else 0
    print(f"{'Ray' if use_ray else '单线程'}处理时间: {elapsed_time:.2f} 秒, 吞吐量: {throughput:.2f} images/s")

    # 输出系统资源使用情况
    cpu_usage = psutil.cpu_percent()
    memory_usage = psutil.virtual_memory().percent
    print(f"系统资源使用情况：CPU {cpu_usage}%，内存 {memory_usage}%")

    return elapsed_time, throughput, cpu_usage, memory_usage

def plot_performance(num_images_list, single_thread_throughputs, ray_throughputs, single_thread_cpu_usages, ray_cpu_usages):
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(10, 8))

    ax1.plot(num_images_list, single_thread_throughputs, marker='o', linestyle='-', color='b', label='Single-thread Throughput')
    ax1.plot(num_images_list, ray_throughputs, marker='o', linestyle='-', color='g', label='Ray Throughput')
    ax1.set_xscale('log')
    ax1.set_xlabel('Number of Images (Log Scale)')
    ax1.set_ylabel('Throughput (images/s)')
    ax1.set_title('Single-thread vs Ray Throughput with Number of Images')
    ax1.legend()

    ax2.plot(num_images_list, single_thread_cpu_usages, marker='o', linestyle='-', color='r', label='Single-thread CPU Usage')
    ax2.plot(num_images_list, ray_cpu_usages, marker='o', linestyle='-', color='m', label='Ray CPU Usage')
    ax2.set_xscale('log')
    ax2.set_xlabel('Number of Images (Log Scale)')
    ax2.set_ylabel('CPU Usage (%)')
    ax2.set_title('CPU Usage with Number of Images')
    ax2.legend()

    plt.tight_layout()

    # 保存图表到当前目录
    plt.savefig(os.path.join(os.getcwd(), 'performance_plot.png'))

    # 清除绘图缓存
    plt.clf()
    plt.close()

if __name__ == "__main__":
    num_images_list = [10, 50, 100, 200, 500, 1000, 2000, 5000, 10000, 20000]

    single_thread_throughputs = []
    ray_throughputs = []
    single_thread_cpu_usages = []
    ray_cpu_usages = []

    # 下载 CIFAR-10 数据集
    transform = transforms.Compose([transforms.ToTensor()])
    cifar10 = datasets.CIFAR10(root='.', train=True, download=True, transform=transform)

    for num_images in num_images_list:
        # 设置保存图像的目录为当前工作目录下的 cifar10_images 文件夹
        save_dir = os.path.join(os.getcwd(), 'cifar10_images')

        # 如果目录存在，则清空其中的所有文件
        if os.path.exists(save_dir):
            file_list = glob(os.path.join(save_dir, '*.jpg'))
            for file_path in file_list:
                os.remove(file_path)
        else:
            os.makedirs(save_dir)  # 如果目录不存在，则创建目录

        # 保存图像
        save_images(cifar10, save_dir, num_images)
        print(f"保存了 {num_images} 张图像")

        # 获取图像路径
        image_paths = glob(os.path.join(save_dir, '*.jpg'))

        # 测量单线程性能
        _, single_thread_throughput, single_thread_cpu_usage, _ = measure_performance(image_paths, os.path.join(save_dir, 'single_thread_image_{}.jpg'), use_ray=False)
        single_thread_throughputs.append(single_thread_throughput)
        single_thread_cpu_usages.append(single_thread_cpu_usage)

        # 测量 Ray 并行性能
        _, ray_throughput, ray_cpu_usage, _ = measure_performance(image_paths, os.path.join(save_dir, 'ray_image_{}.jpg'), use_ray=True)
        ray_throughputs.append(ray_throughput)
        ray_cpu_usages.append(ray_cpu_usage)

        # 清理测试生成的压缩图像文件
        for i in range(num_images):
            single_file = os.path.join(save_dir, f"single_thread_image_{i}.jpg")
            ray_file = os.path.join(save_dir, f"ray_image_{i}.jpg")
            if os.path.exists(single_file):
                os.remove(single_file)
            if os.path.exists(ray_file):
                os.remove(ray_file)

    # 绘制图表
    plot_performance(num_images_list, single_thread_throughputs, ray_throughputs, single_thread_cpu_usages, ray_cpu_usages)

    # 关闭 Ray
    ray.shutdown()
