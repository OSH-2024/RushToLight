先输入```cargo build --release```构建target文件
随后即可输入 ```gcc -o test test.c -I. -Ltarget/release -llos_memory_lib -ldl -lpthread -Wl,-gc-sectio```编译出可执行文件
