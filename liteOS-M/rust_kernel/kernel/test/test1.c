#include "los_config.h"
#include "los_compiler.h"
#include "los_memory.h"
#include <stdlib.h>
#include <stdio.h>

#define Normal_Pool_Size 10000
#define Small_Pool_Size 100
#define Large_Pool_Size 10000000
#define Small_Alloc_Size 1
#define Large_Alloc_Size 10000 //more than NormalPoolSize

void Print_Pool_Info(void* pool);

void* Init_Pool();


int main(){
    //内存池初始化悬空指针测试---------------------------
    void* pool = NULL;
    printf("悬空指针测试\n");
    int ret = LOS_MemInit(pool,10000);
    if(ret == LOS_NOK){
        printf("ptr is null\n");
    }
    //初始化内存池过大测试----------------------
    printf("初始化内存过大测试\n");
    ret = LOS_MemInit(pool,Large_Pool_Size);
    if(ret == LOS_NOK){
        printf("memory pool is too large\n");
    }
    //初始化内存池过小测试---------------------
    printf("初始化内存过小测试\n");
    ret = LOS_MemInit(pool,Small_Pool_Size);
    if(ret == LOS_NOK){
        printf("memory pool is too small\n");
    }
    //分配内存块过大测试
    printf("分配内存块过大测试\n");
    pool = (void*)malloc(Normal_Pool_Size);//正常初始化内存池
    LOS_MemInit(pool,Normal_Pool_Size);
    void* block = LOS_MemAlloc(pool,Large_Alloc_Size);
    if(block == NULL){
        printf("alloc memory is too large\n");
    }
}



