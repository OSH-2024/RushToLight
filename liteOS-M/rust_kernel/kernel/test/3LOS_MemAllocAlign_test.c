#include "los_memory.h"
void printret(VOID* ret){
    if(ret==NULL){
        printf("fail\n");
    }
    else{
        printf("success");//成功时的返回值是分配好的内存块的开始地址
    }
}
//VOID *LOS_MemAllocAlign(VOID *pool, UINT32 size, UINT32 boundary)，
//从指定动态内存池中申请长度为size且地址按boundary字节对齐的内存
//pool必须经过LOS_MemInit,size比OS_MEM_MIN_POOL_SIZE大（是LOS_MemInit)要求的，也要大于OS_MEM_MIN_ALLOC_SIZE=8大
//boundary必须是2^N次方，且最小为4
//成功的话返回分配的内存的起始地址

int main(){
    VOID* ret;
    UINT32 size1=20;
    UINT32 size2=4;//4不行
    UINT32 boundary1=4;
    UINT32 boundary2=5;//5不行
    VOID *p = NULL;//内存起始地址
    ret=LOS_MemAllocAlign(p,size1,boundary1);//p为NULL,f
    printret(ret);
    p=malloc(size*4);
    if(LOS_MemDeInit(p,size)==OS_ERROR){//如果size<=OS_MEM_MIN_POOL_SIZE,这里出错
        printf("LOS_MemDeInit error\n");
    }
    ret=LOS_MemAllocAlign(p,size2,boundary1);//size,f
    printret(ret);
    ret=LOS_MemAllocAlign(p,size1,boundary2);//boundary,f
    printret(ret);
    ret=LOS_MemAllocAlign(p,size1,boundary1);//s
    printret(ret);
    
    free(p);
    return 0;
    //结果应该是 fail fail fail success

}