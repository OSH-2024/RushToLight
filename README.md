# RushToLight：Rust改写Harmony LiteOS
![alt_text](./src/our_logo.jpg)

项目介绍：OpenHarmony LiteOS-M内核是面向IoT领域构建的轻量级嵌入式物联网操作系统内核。我们打算用RUST改写Harmony LiteOS-M以提高其安全性。

**小组名称**：觉悟OS (RushToLight)

**小组成员**：姬子琢 区家彬 吴宇翀 舒佳豪

**指导老师**：邢凯

**关键词**：Rust，嵌入式操作系统，liteOS，鸿蒙，~~遥遥领先~~

**语言版本: [中文](README.md), [English](README_english.md).**

**OpenHarmony LiteOS-M源码：**[https://gitee.com/openharmony/kernel_liteos_m](https://gitee.com/openharmony/kernel_liteos_m)
## 项目进展

| 项目阶段 | 时间 | 项目进展 | 工作安排 |
|:----: | :----: | :----: | :----: |
| 选题阶段 | 3.3 - 3.9 | 研究往届选题，对可选题方向进行大致讨论后确定每个人细细调研方向 | 小组分工：1.	区家彬：分布式内存管理与进程管理。2.	吴宇翀：机器人操作系统（ROS） 3.	姬子琢：虚拟化4.舒佳豪：AI powered OS|
|    |      3.10  -   3.16          |     各组员分享了自己调研方向的进展，并提出了相关的选题，主要有：  1.制作简单的RosOS 2.  AI操作系统：能实现查找文件，人机交互等功能 3.进程管理 ：使用较新算法对linux任务调度进行优化，实现某一场景的效率提升         |      在和邢凯老师交流后，我们打算尝试AI操作系统方向，并展开进一步的调研
|          |   3.17 - 3.24       |  研讨：如果做系统优化，选择LLM还是 ML好? 用 LLM 是否需要数据集进行优化? If 使用机器学习能否本地制造数据集？ 有那些系统方面的数据集比较容易获取？ 有哪些优化的价值较大的应用场景？ | 咨询邢老师的建议，并在各个开源数据集网站和相关论文中寻找可用数据集，同时保持对Rust等方向的调研（防止找不到数据集）
|   确定选题    |   3.25 - 3.31       |  经过一周查找数据集无果后，我们转战Rust改写方向，确定选题为：用RUST改写Harmony LiteOS-M   |    学习Rust相关特性，对LiteOS的函数调用关系进行研究
|    可行性报告      |   4.1  - 4.7        |  可行性报告内容及分工讨论，报告范围为：Rust特性及改写优缺点，LiteOS内核分析，liteOS编译、 Rust改写LiteOS尝试  | 分工：舒佳豪：Rust特性及改写优缺点  吴宇翀：LiteOS内核分析  	姬子琢：liteOS编译  区家彬：Rust改写LiteOS尝试
|    概要设计报告      |   4.8  - 4.14       |    	汇报交流项目进展，主要有：LiteOS各模块函数功能	LiteOS编译的方法，rust/c混合编译的方法      |    继续推进、完善相关工作
|     期中汇报     |   4.15  - 4.21       |   会议主要任务：规划PPT内容和分工、交流项目进展和困难。我们将PPT分成了liteOS介绍、mem.c,task.c函数介绍、Rust改写原因、LiteOS编译和运行方法、C,Rust编译依赖工具链、未来计划等部分，并进行了分工。   |   分工： 演讲：区家彬     PPT制作： 舒佳豪：liteOS介绍、Rust优点及特性、未来计划    姬子琢： LiteOS编译和运行方法、C,Rust编译依赖工具链     吴宇翀：mem.c,task.c函数介绍，函数潜在漏洞和改写原因分析
| | |