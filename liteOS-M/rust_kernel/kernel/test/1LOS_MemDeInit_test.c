#include "los_memory.h"
void printret(UINT32 ret){
    if(ret==OS_ERROR){
        printf("OS_ERROR\n");
    }
    else{
        printf("LOS_OK");
    }
}

int main(){//size是内存块的大小（B为单位）不是节点的个数 
    UINT32 ret;
    UINT32 size1,size2;
    VOID *p1 = NULL;//内存起始地址
    VOID *p2=NULL;
    size1=OS_MEM_MIN_POOL_SIZE-4;//不合格
    size2=OS_MEM_MIN_POOL_SIZE+4;//合格
    ret=LOS_MemDeInit_test(p2,size2);//p为null-->OS_ERROR
    printret(ret);
    //p1=(struct OsMemPoolHead *)malloc(size1*sizeof(struct OsMemPoolHead));
    p1=malloc(size1*4);
    ret=LOS_MemDeInit_test(p1,size1);//size1<=OS_MEM_MIN_POOL_SIZE-->OS_ERROR
    printret(ret);
    //p2=(struct OsMemPoolHead *)malloc(size2*sizeof(struct OsMemPoolHead));
    p2=malloc(size2*4);
    ret=LOS_MemDeInit_test(p2,size2);//-->LOS_OK
    printret(ret);

    free(p1);
    free(p2);
    return 0;
    //结果应该是 OS_ERROR OS_ERROR LOS_OK

}