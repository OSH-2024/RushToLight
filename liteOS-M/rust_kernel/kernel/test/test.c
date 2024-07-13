
#include "los_config.h"
#include "los_compiler.h"
#include "los_memory.h"
#include <stdlib.h>
#include <stdio.h>

#define Pool_Size 10000
#define Region_Size 400
#define Boundary 32

void Print_Pool_Info(void* pool);

void* Init_Pool();


void AllocAlign_Test(void* pool); //测试对齐内存的申请
int main(){
    int ret;
    int i;
    void* pool = Init_Pool();
    Print_Pool_Info(pool);
    LOS_MemAlloc(pool,Region_Size);
    Print_Pool_Info(pool);
    void* block = LOS_MemAllocAlign(pool,Region_Size,Boundary);
    Print_Pool_Info(pool);
    block = LOS_MemRealloc(pool,block,800);
    Print_Pool_Info(pool);
    LOS_MemFree(pool,block);
    Print_Pool_Info(pool);
}

void* Init_Pool(){
    void* pool = (void*)malloc(Pool_Size);
    int ret = LOS_MemInit(pool,Pool_Size); //初始化内存池
    if(ret == LOS_NOK){
        printf("LOS_MemInit UNSUCCESS\n");
        exit(-1);
    }
    else{
        printf("LOS_MemInit SUCCESS\n");
        return pool;
    }
}


void Print_Pool_Info(void* pool){
    LOS_MEM_POOL_STATUS *poolStatus = (LOS_MEM_POOL_STATUS*)malloc(sizeof(LOS_MEM_POOL_STATUS));
    LOS_MemInfoGet(pool,poolStatus);
    printf
    ("----------------------------------------\npoolsize = %d\ntotalUsedSize = %d\ntotalFreeSize = %d\nmaxFreeNodeSize = %d\nusedNodeNum = %d\nfreeNodeNum = %d\n-----------------------------------------\n",
    LOS_MemPoolSizeGet(pool),poolStatus->totalUsedSize,poolStatus->totalFreeSize,poolStatus->maxFreeNodeSize,poolStatus->usedNodeNum,poolStatus->freeNodeNum);
    free(poolStatus);
    return;
}
