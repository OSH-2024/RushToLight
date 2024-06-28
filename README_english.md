# RushToLight: Rewriting Huawei LiteOS with Rust
![alt_text](./src/our_logo.jpg)

Project Introduction: OpenHarmony LiteOS-M kernel is a lightweight embedded IoT operating system kernel built for the IoT field. We plan to rewrite Harmony LiteOS-M using RUST to improve its security.

**Team Name:** RushToLight

**Team Members:**  Ji Zizhuo, Qu Jiabin, Wu Yuchong, Shu Jiahao

**Supervisor:** Xing Kai

**Keywords:** Rust, Embedded Operating System, LiteOS, Harmony, ~~Leading the Way~~

**OpenHarmony LiteOS-M Source Code:**[https://gitee.com/openharmony/kernel_liteos_m](https://gitee.com/openharmony/kernel_liteos_m)

**Other Language Versions:  [中文](README.md), [English](README_english.md).**

## Project Progress

| Project Stage | Time | Project Progress | Work Arrangement |
|:----: | :----: | :----: | :----: |
| Topic Selection | 3.3 - 3.9 |Researched previous topics, discussed possible directions, and determined individual research directions | Team division: 1. Qu Jiabin: Distributed memory management and process management. 2. Wu Yuchong: Robot Operating System (ROS) 3. Ji Zizhuo: Virtualization 4. Shu Jiahao: AI powered OS|
|    |      3.10  -   3.16          |   Each team member shared their research progress and proposed related topics, mainly including: 1. Create a simple RosOS 2. AI operating system: can realize functions such as file search and human-computer interaction 3. Process management: use newer algorithms to optimize Linux task scheduling and achieve efficiency improvement in a certain scenario        |      After communicating with Professor Xing Kai, we plan to try the AI operating system direction and conduct further research
|          |   3.17 - 3.24       |  Discussion: Is LLM or ML better for system optimization? Does using LLM require a dataset for optimization? If using machine learning, can datasets be created locally? What system-related datasets are relatively easy to obtain? What application scenarios have greater value for optimization? | Consult Professor Xing for advice, search for available datasets on various open-source dataset websites and related papers, while continuing to research Rust and other directions (in case datasets cannot be found)
|   Determine Topic    |   3.25 - 3.31       |  After a week of unsuccessful dataset search, we switched to the Rust rewrite direction and determined the topic: Rewrite Harmony LiteOS-M using RUST  |    Learn Rust features and study the function call relationships in LiteOS
|    Feasibility Report      |   4.1  - 4.7        |  Discussed the content and division of the feasibility report, covering: Rust features and pros and cons of rewriting, LiteOS kernel analysis, LiteOS compilation, and attempts to rewrite LiteOS with Rust  | Division: Shu Jiahao: Rust features and pros and cons of rewriting, Wu Yuchong: LiteOS kernel analysis, Ji Zizhuo: LiteOS compilation, Qu Jiabin: Attempts to rewrite LiteOS with Rust
|    Outline Design Report      |   4.8  - 4.14       |    Reported and discussed project progress, mainly including: LiteOS module function features, LiteOS compilation methods, Rust/C mixed compilation methods      |    Continue to advance and improve related work
|     Mid-term Report    |   4.15  - 4.21       |  The main tasks of the meeting were to plan PPT content and division of labor, and discuss project progress and difficulties. We divided the PPT into sections: LiteOS introduction, mem.c and task.c function introduction, reasons for Rust rewriting, LiteOS compilation and running methods, C and Rust compilation toolchains, future plans, etc., and assigned tasks.   |   Division: Presentation: Qu Jiabin. PPT creation: Shu Jiahao: LiteOS introduction, Rust advantages and features, future plans. Ji Zizhuo: LiteOS compilation and running methods, C and Rust compilation toolchains. Wu Yuchong: mem.c and task.c function introduction, analysis of potential vulnerabilities in functions and reasons for rewriting. Translate the above into English.
| Rust Rewriting Attempt | 4.22 - 4.28 | Plan to start rewriting from memory.c, with each person assigned to rewrite specific functions, and finally attempt to combine them. | The rewriting task assignments are annotated in the uploaded liteOS-M/kernel/mm/memory.c file.
| | 4.29 - 5.12 | Discuss rewriting experiences, focusing on how to handle null pointer rewriting issues, and continue with new rewriting tasks. | Accelerate the rewriting pace and increase the rewriting workload.
| Lab4 | 5.13 - 6.2 | Discuss the tasks for Lab4 | Decide to choose the ray topic and carry out related tasks.
| Lab4 | 6.3 - 6.25 | Complete Lab4 related tasks | After selecting the test program, perform single-machine deployment of ray, and conduct distributed deployment and performance testing on the vlab virtual machine.
| Final Sprint | 6.26 - Present | Integrate the rust rewriting results of memory.c and start the rewriting task for task.c | Main person in charge of memory.c rewriting: Qu Jiabin; Main Person in charge of task.c rewriting: Shu Jiahao, Ji Zizhuo, Wu Yuchong