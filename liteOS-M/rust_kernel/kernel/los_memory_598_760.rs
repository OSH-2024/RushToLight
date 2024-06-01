//598-760行，以后需要修改引用类型如何检测NULL类型


#[inline]
pub fn OsMemUsedNodePrint(node: &mut OsMemNodeHead){
    let mut count: UINT32;
    if (OS_MEM_NODE_GET_USED_FLAG(node->sizeAndFlag) && !OS_MEM_IS_GAP_NODE(node)) {
        PRINTK("0x%x: 0x%x ", (UINTPTR)node, OS_MEM_NODE_GET_SIZE(unsafe{node->sizeAndFlag}));
        for count in 0..LOSCFG_MEM_RECORD_LR_CNT{
            PRINTK(" 0x%x ", node->linkReg[count]);
        }
        PRINTK("\n");
        OsMemLeakCheckInfoRecord(node);
    }
}

pub fn OsMemUsedNodePrintHandle(node: &mut OsMemNodeHead,arg: *const VOID){
    UNUSED(arg);
    OsMemUsedNodePrint(node);
    return;
}

pub fn LOS_MemUsedNodeShow(pool: &mut OsMemPoolHead){
    let mut count: UINT32;
    PRINTK("\n\rnode          size    ");
    for count in 0..LOSCFG_MEM_RECORD_LR_CNT{
        PRINTK("    LR[%u]   ", count);
    }
    OsMemLeakCheckInit();
    OsAllMemNodeDoHandle(pool, OsMemUsedNodePrintHandle, std::ptr::null_mut());
    return;
}

#[cfg(LOSCFG_KERNEL_PRINTF != 0)]
pub fn OsMemNodeBacktraceInfo(tmpNode: &mut OsMemNodeHead,preNode: &mut OsMemNodeHead){
    let mut i: UINT32;
    for i in 0..LOSCFG_MEM_RECORD_LR_CNT{
        PRINTK(" LR[%d]:0x%x\n", i, tmpNode->linkReg[i]);
    }
    PRINTK("\n pre node head LR info: \n");
    for i in 0..LOSCFG_MEM_RECORD_LR_CNT{
        PRINTK(" LR[%d]:0x%x\n", i, preNode->linkReg[i]);
    }
}

#[inline]
pub fn OsMemFreeListIndexGet(size: UINT32)->UINT32{
    let fl: UINT32 = OsMemFlGet(size);
    if (fl < OS_MEM_SMALL_BUCKET_COUNT) {
        return fl;
    }
    let sl: UINT32 = OsMemSlGet(size, fl);
    return (OS_MEM_SMALL_BUCKET_COUNT + ((fl - OS_MEM_SMALL_BUCKET_COUNT) << OS_MEM_SLI) + sl);
}

#[inline]
pub fn OsMemFindCurSuitableBlock(poolHead: &mut OsMemPoolHead,index: UINT32,size: UINT32)->(*const OsMemFreeNodeHead){
    let mut node: &OsMemPoolHead = poolHead->freeList[index];
    while(!node.isnull()){
        if (node->header.sizeAndFlag >= size) {
            return (node as *const OsMemFreeNodeHead);
        }
        node = node->next;
    }
    return std::ptr::null_mut();
}

//rust的位操作应该与c一致
#[inline]
pub fn OsMemNotEmptyIndexGet(poolHead: &mut OsMemPoolHead,index: UINT32)->UINT32{
    /* 5: Divide by 32 to calculate the index of the bitmap array. */
    let mask: UINT32 = poolHead->freeListBitmap[index >> 5];
    mask &= ~((1 << (index & OS_MEM_BITMAP_MASK)) - 1);
    if (mask != 0) {
        index = OsMemFFS(mask) + (index & ~OS_MEM_BITMAP_MASK);
        return index;
    }

    return OS_MEM_FREE_LIST_COUNT;
}

#[inline]
pub fn OsMemFindNextSuitableBlock(pool: &mut OsMemPoolHead,size: UINT32,outIndex: &mut UINT32)->(*const c_void){
    let poolHead: &OsMemPoolHead  = pool;
    let mut fl: UINT32 = OsMemFlGet(size);
    let mut sl: UINT32;
    let mut index: UINT32;
    let mut tmp : UINT32;
    let mut mask : UINT32;
    let mut curIndex = OS_MEM_FREE_LIST_COUNT;
    do{
        if (fl < OS_MEM_SMALL_BUCKET_COUNT) {
            index = fl;
        } 
        else {
            sl = OsMemSlGet(size, fl);
            curIndex = ((fl - OS_MEM_SMALL_BUCKET_COUNT) << OS_MEM_SLI) + sl + OS_MEM_SMALL_BUCKET_COUNT;
            index = curIndex + 1;
        }

        tmp = OsMemNotEmptyIndexGet(poolHead, index);
        if (tmp != OS_MEM_FREE_LIST_COUNT) {
            index = tmp;
            *outIndex = index;//change GOTO
            return poolHead->freeList[index] as *const c_void;
        }

        for (index = LOS_Align(index + 1, 32); index < OS_MEM_FREE_LIST_COUNT; index += 32) {
            /* 5: Divide by 32 to calculate the index of the bitmap array. */
            mask = poolHead->freeListBitmap[index >> 5];
            if (mask != 0) {
                index = OsMemFFS(mask) + index;
                *outIndex = index;
                return poolHead->freeList[index] as *const c_void;
            }
        }
    } while(0);
    if (curIndex == OS_MEM_FREE_LIST_COUNT) {
        return ptr::null_mut();
    }
    *outIndex = curIndex;
    return OsMemFindCurSuitableBlock(poolHead, curIndex, size);
}

#[inline]
pub fn OsMemSetFreeListBit(head: &mut OsMemPoolHead,index: UINT32){
    head->freeListBitmap[index >> 5] |= 1U << (index & 0x1f);
}

#[inline]
pub fn OsMemClearFreeListBit(head: &mut OsMemPoolHead,index: UINT32){
    head->freeListBitmap[index >> 5] &= ~(1U << (index & 0x1f));
}

#[inline]
pub fn OsMemListAdd(pool: &mut OsMemPoolHead,listIndex: UINT32,node: &mut OsMemFreeNodeHead){
    let mut firstNode: &OsMemPoolHead = pool->freeList[listIndex];
    if (!firstNode.isnull()) { //引用不方便检测isnull
        firstNode->prev = node;
    }
    node->prev = ptr::null_mut();
    node->next = firstNode;
    pool->freeList[listIndex] = node;
    OsMemSetFreeListBit(pool, listIndex);
    OS_MEM_SET_MAGIC(&node->header);
}