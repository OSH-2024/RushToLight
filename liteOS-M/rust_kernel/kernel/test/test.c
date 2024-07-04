#include "los_memory.h" 
#include "los_config.h"
#include <stdlib.h>
#include <stdio.h>

#define Pool_Size = 400


//可能无法得到OS_MEM_NODE_HEAD_SIZE的大小

void Print_Pool_Info(void* pool;

void Print_Pool_Block(void* pool);

void* Init_Pool();

void Alloc_Test(void* pool); //测试内存申请和释放

void AllocAlign_Test(void* pool); //测试对齐内存的申请

int main(){
    int ret;
    int i;
    void* pool = Init_Pool();
    Print_Pool_Info(pool);
    //---------------------------------------------------
    Alloc_Test(pool);
    //---------------------------------------------------
    LOS_MemDeInit(pool);
}

void Init_Pool(){
    void* pool = (void*)malloc(Pool_Size);
    ret = LOS_MemInit(pool,Pool_Size); //初始化内存池
    if(ret == LOS_NOK){
        printf("LOS_MemInit UNSUCCESS\n");
        exit(-1);
    }
    else{
        return pool;
    }
}

void Alloc_Test(void* pool){
    printf("现在申请5个分别为为22,44的内存块:\n");
    //注意,返回的地址是实际可用内存的地址,去掉了头部的信息域
    void* Alloc_Block_1 = LOS_MemAlloc(pool,22);
    void* Alloc_Block_2 = LOS_MemAlloc(pool,44);
    Print_Pool_Info(pool);
    LOS_MemUsedNodeShow(pool);//打印使用节点
    LOS_MemFreeNodeShow(pool);//打印空闲节点

    LOS_MemFree(Pool,Alloc_Block1);
    LOS_MemFree(Pool,Alloc_Block2);
    Print_Pool_Info(pool);
    LOS_MemUsedNodeShow(pool);//打印使用节点
    LOS_MemFreeNodeShow(pool);//打印空闲节点
}

void AllocAlign_Test(void* pool){
    printf("内存池起始位置:%p\n",pool);
    printf("内存池头大小:%d\n",sizeof(struct OsMemPoolHead));
    printf("对齐边界:32\n");
    void* Alloc_Block = LOS_MemAllocAlign(pool,32);
    printf("未偏移内存开头为:%p\n",pool-sizeof(struct OsMemPoolHead)-OS_MEM_NODE_HEAD_SIZE);
    printf("偏移后内存开头为:%p\n",Alloc_Block);
}

void Print_Pool_Info(void* pool){
    LOS_MEM_POOL_STATUS *poolStatus = (*LOS_MEM_POOL_STATUS)malloc(sizeof(LOS_MEM_POOL_STATUS));
    LOS_MemInfoGet(pool,poolStatus);
    printf
    ("----------------------------------------
    poolsize = %d
    totalUsedSize = %d
    totalFreeSize = %d
    maxFreeNodeSize = %d
    usedNodeNum = %d
    freeNodeNum = %d
    -----------------------------------------",
    LOS_MemPoolSizeGet(pool),poolStatus->totalUsedSize,poolStatus->totalFreeSize,poolStatus->maxFreeNodeSize,poolStatus->usedNodeNum,poolStatus->freeNodeNum);
    free(poolStatus);
    return;
}

void Print_Pool_Block(void* pool){ //打印整个内存池各个节点信息
    int node_count;
    (struct OsMemNodeHead *) current_node = (struct OsMemNodeHead *)((UINT8 *)(pool) + sizeof(struct OsMemPoolHead));
    (struct OsMemNodeHead *) end_node = (struct OsMemNodeHead *)((UINT8 *)(pool) + (size) - OS_MEM_NODE_HEAD_SIZE);
    while(current_node < end_node){
        printf("%d号节点,")
    }
}