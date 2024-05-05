pub fn OsAllMemNodeDohandle(pool: &mut OsMemPoolHead,handle: fn(*const OsMemNodeHead,*const VOID)->*const VOID,arg:*const VOID){ //可以不用加上结构体前缀"struct"
    let poolInfo: &OsMemPoolHead = pool ;
    let intsave: UINT32 = 0;
    if(pool.isnull()){
        PRINTK("input param is NULL\n"); //los_debug的宏
        return;
    }
    if (LOS_MemIntegrityCheck(pool)) {
        PRINTK("LOS_MemIntegrityCheck error\n");
        return;
    }
    MEM_LOCK(poolInfo, intSave);
    let mut endNode: &OsMemPoolHead = OS_MEM_END_NODE(pool, poolInfo->info.totalSize);
    let mut tmpNode: &OsMemPoolHead = OS_MEM_FIRST_NODE(pool);
    while(tmpNode <= endNode){
        if (tmpNode == endNode) {
#[cfg(OS_MEM_EXPAND_ENABLE == 1)]
{   
            let size:UINT32 =  OS_MEM_NODE_GET_SIZE(endNode->sizeAndFlag);
            tmp_node = OsMemSentinelNodeGet(endNode) ;
            end_node = OS_MEM_END_NODE(tmpNode, size);
            continue;
}
        break;
        }
        handle(tmpNode, arg);
        tmpNode = OS_MEM_NEXT_NODE(tmpNode);
    }
    MEM_UNLOCK(poolInfo, intSave);
}


#[cfg(LOSCFG_TASK_MEM_USED == 1)]{
pub fn GetTaskMemUsedHandle(curNode: &mut OsMemNodeHead,arg: &UINT32){
    let mut args: &UINT32 = arg;
    let tskMemInfoBuf: &UINT32 = (args as UINTPTR)as &UINT32; //双重转换
    let tskMemInfoCnt: UINT32 = *args.offset(1) as UINT32;
#[cfg(not(feature = "LOSCFG_MEM_MUL_REGIONS"))]{ //暂时没有ifndef的对应语句
    if (OS_MEM_NODE_GET_USED_FLAG(curNode->sizeAndFlag)) {
}
#[cfg(feature = "LOSCFG_MEM_MUL_REGIONS")]{
    if (OS_MEM_NODE_GET_USED_FLAG(curNode->sizeAndFlag) && !OS_MEM_IS_GAP_NODE(curNode)) {
}   
        if (curNode->taskID < tskMemInfoCnt) {
            tskMemInfoBuf[curNode->taskID] += OS_MEM_NODE_GET_SIZE(unsafe{curNode->sizeAndFlag});
        }
    }
    return;
}