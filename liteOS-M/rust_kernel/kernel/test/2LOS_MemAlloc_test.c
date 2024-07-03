#include "los_memory.h"
void printret(VOID* ret){
    if(ret==NULL){
        printf("fail\n");
    }
    else{
        printf("success\n");//成功时的返回值是分配好的内存块的开始地址
    }
}

int main(){//LOS_MemAlloc申请动态内存，p不能为空,必须是LOS_MemDeInit过的，size不能为0，且要比OS_MEM_MIN_POOL_SIZE大
//(要求4字节对齐，起始地址是4的倍数)？
//pool是指向要分配的内存块的指针，size是内存块的大小，以B为单位
    VOID* ret;
    UINT32 size=20;
    VOID *p = NULL;//内存起始地址
    ret=LOS_MemAlloc(p,size);//p为NULL,f
    printret(ret);
    p=malloc(size*4);
    if(LOS_MemDeInit(p,size)==OS_ERROR){//如果size<=OS_MEM_MIN_POOL_SIZE,这里出错
        printf("LOS_MemDeInit error\n");
    }

    ret=LOS_MemAlloc(p,0);//size==0,f
    printret(ret);
    ret=LOS_MemAlloc(p,size);//size==0,f
    printret(ret);
    
    free(p);
    return 0;
    //结果应该是 fail fail success

}