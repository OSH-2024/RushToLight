//上面是c函数，下面是rust函数，作者舒佳豪
/* 
STATIC INLINE BOOL TryShrinkPool(const VOID *pool, const struct OsMemNodeHead *node)
{
    struct OsMemNodeHead *mySentinel = NULL;
    struct OsMemNodeHead *preSentinel = NULL;
    size_t totalSize = (UINTPTR)node->ptr.prev - (UINTPTR)node;
    size_t nodeSize = OS_MEM_NODE_GET_SIZE(node->sizeAndFlag);

    if (nodeSize != totalSize) {
        return FALSE;
    }

    preSentinel = PreSentinelNodeGet(pool, node);
    if (preSentinel == NULL) {
        return FALSE;
    }

    mySentinel = node->ptr.prev;
    if (OsMemIsLastSentinelNode(mySentinel)) { /* prev node becomes sentinel node */
        preSentinel->ptr.next = NULL;
        OsMemSentinelNodeSet(preSentinel, NULL, 0);
    } else {
        preSentinel->sizeAndFlag = mySentinel->sizeAndFlag;
        preSentinel->ptr.next = mySentinel->ptr.next;
    }

    if (OsMemLargeNodeFree(node) != LOS_OK) {
        PRINT_ERR("TryShrinkPool free 0x%x failed!\n", node);
        return FALSE;
    }

    return TRUE;
} 
*/
fn try_shrink_pool(pool: *const u8, node: *const OsMemNodeHead) -> bool {
    let total_size = (node.ptr.prev as usize) - (node as usize);
    let node_size = os_mem_node_get_size(node.size_and_flag);

    if node_size != total_size {
        return false;
    }

    let pre_sentinel = pre_sentinel_node_get(pool, node);
    if pre_sentinel.is_null() {
        return false;
    }

    let my_sentinel = node.ptr.prev;
    if os_mem_is_last_sentinel_node(my_sentinel) {
        // prev node becomes sentinel node
        unsafe {
            (*pre_sentinel).ptr.next = ptr::null_mut();
            os_mem_sentinel_node_set(pre_sentinel, ptr::null_mut(), 0);
        }
    } else {
        unsafe {
            (*pre_sentinel).size_and_flag = (*my_sentinel).size_and_flag;
            (*pre_sentinel).ptr.next = (*my_sentinel).ptr.next;
        }
    }

    if os_mem_large_node_free(node as *mut OsMemNodeHead) != LOS_OK {
        println!("TryShrinkPool free {:?} failed!", node);
        return false;
    }

    true
}
/*
STATIC INLINE INT32 OsMemPoolExpand(VOID *pool, UINT32 size, UINT32 intSave)
{
    UINT32 tryCount = MAX_SHRINK_PAGECACHE_TRY;
    struct OsMemPoolHead *poolInfo = (struct OsMemPoolHead *)pool;
    struct OsMemNodeHead *newNode = NULL;
    struct OsMemNodeHead *endNode = NULL;

    size = ROUNDUP(size + OS_MEM_NODE_HEAD_SIZE, PAGE_SIZE);
    endNode = OS_MEM_END_NODE(pool, poolInfo->info.totalSize);

RETRY:
    newNode = (struct OsMemNodeHead *)LOS_PhysPagesAllocContiguous(size >> PAGE_SHIFT);
    if (newNode == NULL) {
        if (tryCount > 0) {
            tryCount--;
            MEM_UNLOCK(poolInfo, intSave);
            OsTryShrinkMemory(size >> PAGE_SHIFT);
            MEM_LOCK(poolInfo, intSave);
            goto RETRY;
        }

        PRINT_ERR("OsMemPoolExpand alloc failed size = %u\n", size);
        return -1;
    }
    newNode->sizeAndFlag = (size - OS_MEM_NODE_HEAD_SIZE);
    newNode->ptr.prev = OS_MEM_END_NODE(newNode, size);
    OsMemSentinelNodeSet(endNode, newNode, size);
    OsMemFreeNodeAdd(pool, (struct OsMemFreeNodeHead *)newNode);

    endNode = OS_MEM_END_NODE(newNode, size);
    (VOID)memset_s(endNode, sizeof(*endNode), 0, sizeof(*endNode));
    endNode->ptr.next = NULL;
    OS_MEM_SET_MAGIC(endNode);
    OsMemSentinelNodeSet(endNode, NULL, 0);
    OsMemWaterUsedRecord(poolInfo, OS_MEM_NODE_HEAD_SIZE);

    return 0;
}
*/
fn os_mem_pool_expand(pool: *mut OsMemPoolHead, size: usize) -> i32 {
    let mut try_count = MAX_SHRINK_PAGECACHE_TRY;
    let pool_info = unsafe { &mut (*pool) };
    let mut new_node: *mut OsMemNodeHead = ptr::null_mut();
    let end_node = os_mem_end_node(pool, pool_info.info.total_size);

    let size = (size + os_mem_node_head_size()).round_up_to(PAGE_SIZE);

    'retry: loop {
        new_node = unsafe { los_phys_pages_alloc_contiguous(size / PAGE_SIZE) };
        if !new_node.is_null() {
            break;
        }

        if try_count > 0 {
            try_count -= 1;
            // MEM_UNLOCK(pool_info, int_save);
            os_try_shrink_memory(size / PAGE_SIZE);
            // MEM_LOCK(pool_info, int_save);
            continue 'retry;
        }

        println!("OsMemPoolExpand alloc failed size = {}", size);
        return -1;
    }

    unsafe {
        (*new_node).size_and_flag = size - os_mem_node_head_size();
        (*new_node).ptr.prev = os_mem_end_node(new_node, size);
        os_mem_sentinel_node_set(end_node, new_node, size);
        os_mem_free_node_add(pool, new_node);

        let end_node = os_mem_end_node(new_node, size);
        ptr::write_bytes(end_node, 0, 1);
        (*end_node).ptr.next = ptr::null_mut();
        // OS_MEM_SET_MAGIC(end_node);
        os_mem_sentinel_node_set(end_node, ptr::null_mut(), 0);
        os_mem_water_used_record(pool_info, os_mem_node_head_size());
    }

    0
}
//to be done
VOID LOS_MemExpandEnable(VOID *pool)
{
    if (pool == NULL) {
        return;
    }

    ((struct OsMemPoolHead *)pool)->info.attr |= OS_MEM_POOL_EXPAND_ENABLE;
}
#endif

#ifdef LOSCFG_KERNEL_LMS
STATIC INLINE VOID OsLmsFirstNodeMark(VOID *pool, struct OsMemNodeHead *node)
{
    if (g_lms == NULL) {
        return;
    }

    g_lms->simpleMark((UINTPTR)pool, (UINTPTR)node, LMS_SHADOW_PAINT_U8);
    g_lms->simpleMark((UINTPTR)node, (UINTPTR)node + OS_MEM_NODE_HEAD_SIZE, LMS_SHADOW_REDZONE_U8);
    g_lms->simpleMark((UINTPTR)OS_MEM_NEXT_NODE(node), (UINTPTR)OS_MEM_NEXT_NODE(node) + OS_MEM_NODE_HEAD_SIZE,
        LMS_SHADOW_REDZONE_U8);
    g_lms->simpleMark((UINTPTR)node + OS_MEM_NODE_HEAD_SIZE, (UINTPTR)OS_MEM_NEXT_NODE(node),
        LMS_SHADOW_AFTERFREE_U8);
}

STATIC INLINE VOID OsLmsAllocAlignMark(VOID *ptr, VOID *alignedPtr, UINT32 size)
{
    struct OsMemNodeHead *allocNode = NULL;

    if ((g_lms == NULL) || (ptr == NULL)) {
        return;
    }
    allocNode = (struct OsMemNodeHead *)((struct OsMemUsedNodeHead *)ptr - 1);
    if (ptr != alignedPtr) {
        g_lms->simpleMark((UINTPTR)ptr, (UINTPTR)ptr + sizeof(UINT32), LMS_SHADOW_PAINT_U8);
        g_lms->simpleMark((UINTPTR)ptr + sizeof(UINT32), (UINTPTR)alignedPtr, LMS_SHADOW_REDZONE_U8);
    }

    /* mark remining as redzone */
    g_lms->simpleMark(LMS_ADDR_ALIGN((UINTPTR)alignedPtr + size), (UINTPTR)OS_MEM_NEXT_NODE(allocNode),
        LMS_SHADOW_REDZONE_U8);
}

STATIC INLINE VOID OsLmsReallocMergeNodeMark(struct OsMemNodeHead *node)
{
    if (g_lms == NULL) {
        return;
    }

    g_lms->simpleMark((UINTPTR)node + OS_MEM_NODE_HEAD_SIZE, (UINTPTR)OS_MEM_NEXT_NODE(node),
        LMS_SHADOW_ACCESSIBLE_U8);
}

STATIC INLINE VOID OsLmsReallocSplitNodeMark(struct OsMemNodeHead *node)
{
    if (g_lms == NULL) {
        return;
    }
    /* mark next node */
    g_lms->simpleMark((UINTPTR)OS_MEM_NEXT_NODE(node),
        (UINTPTR)OS_MEM_NEXT_NODE(node) + OS_MEM_NODE_HEAD_SIZE, LMS_SHADOW_REDZONE_U8);
    g_lms->simpleMark((UINTPTR)OS_MEM_NEXT_NODE(node) + OS_MEM_NODE_HEAD_SIZE,
        (UINTPTR)OS_MEM_NEXT_NODE(OS_MEM_NEXT_NODE(node)), LMS_SHADOW_AFTERFREE_U8);
}

STATIC INLINE VOID OsLmsReallocResizeMark(struct OsMemNodeHead *node, UINT32 resize)
{
    if (g_lms == NULL) {
        return;
    }
    /* mark remaining as redzone */
    g_lms->simpleMark((UINTPTR)node + resize, (UINTPTR)OS_MEM_NEXT_NODE(node), LMS_SHADOW_REDZONE_U8);
}
#endif

#if (LOSCFG_MEM_LEAKCHECK == 1)
struct OsMemLeakCheckInfo {
    struct OsMemNodeHead *node;
    UINTPTR linkReg[LOSCFG_MEM_RECORD_LR_CNT];
};

struct OsMemLeakCheckInfo g_leakCheckRecord[LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM] = {0};
STATIC UINT32 g_leakCheckRecordCnt = 0;

STATIC INLINE VOID OsMemLeakCheckInfoRecord(struct OsMemNodeHead *node)
{
    struct OsMemLeakCheckInfo *info = &g_leakCheckRecord[g_leakCheckRecordCnt];

    if (!OS_MEM_NODE_GET_LEAK_FLAG(node->sizeAndFlag)) {
        info->node = node;
        (VOID)memcpy(info->linkReg, node->linkReg, sizeof(node->linkReg));
        OS_MEM_NODE_SET_LEAK_FLAG(node->sizeAndFlag);
        g_leakCheckRecordCnt++;
        if (g_leakCheckRecordCnt >= LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM) {
            g_leakCheckRecordCnt = 0;
        }
    }
}

STATIC INLINE VOID OsMemLeakCheckInit(VOID)
{
    (VOID)memset_s(g_leakCheckRecord, sizeof(struct OsMemLeakCheckInfo) * LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM,
                   0, sizeof(struct OsMemLeakCheckInfo) * LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM);
    g_leakCheckRecordCnt = 0;
}

STATIC INLINE VOID OsMemLinkRegisterRecord(struct OsMemNodeHead *node)
{
    (VOID)memset(node->linkReg, 0, sizeof(node->linkReg));
    OsBackTraceHookCall(node->linkReg, LOSCFG_MEM_RECORD_LR_CNT, LOSCFG_MEM_OMIT_LR_CNT, 0);
}