/*UINT32 LOS_MemFreeByTaskID(VOID *pool, UINT32 taskID)
{
    UINT32 args[2] = { taskID, (UINT32)(UINTPTR)pool };
    if (pool == NULL) {
        return LOS_NOK;
    }

    if (taskID >= LOSCFG_BASE_CORE_TSK_LIMIT) {
        return LOS_NOK;
    }

    OsAllMemNodeDoHandle(pool, MemNodeFreeByTaskIDHandle, (VOID *)args);

    return LOS_OK;
}
#endif*/
pub unsafe fn LOS_MemFreeByTaskID ->UINT32(pool:&mut VOID,taskID:UINT32){
    let args:[UINT32;2] =[taskID,(pool as UINTPTR) as UINT32];
    //pub type UINTPTR = ::std::os::raw::c_uint;
    if pool==std::ptr::null_mut() {
        return LOS_NOK;
    }
    if taskID >=LOSCFG_BASE_CORE_TSK_LIMIT {
        return LOS_NOK;
    }

    //pub fn OsAllMemNodeDohandle(pool: &mut OsMemPoolHead,handle: fn(*const OsMemNodeHead,*const VOID)->*const VOID,arg:*const VOID){ //可以不用加上结构体前缀"struct"
    OsAllMemNodeDoHandle((pool as &mut OsMemPoolHead),MemNodeFreeByTaskIDHandle,args as *const VOID);//////////////
    ////????????????????????/pool
    LOS_OK;
}

}//这是一个#endif   是#[cfg(...)]{ }的一部分



/*UINT32 LOS_MemPoolSizeGet(const VOID *pool)
{
    UINT32 count = 0;

    if (pool == NULL) {
        return LOS_NOK;
    }

    count += ((struct OsMemPoolHead *)pool)->info.totalSize;
#if (LOSCFG_MEM_MUL_REGIONS == 1)
    count -= ((struct OsMemPoolHead *)pool)->info.totalGapSize;
#endif

#if OS_MEM_EXPAND_ENABLE
    UINT32 size;
    struct OsMemNodeHead *node = NULL;
    struct OsMemNodeHead *sentinel = OS_MEM_END_NODE(pool, count);

    while (OsMemIsLastSentinelNode(sentinel) == FALSE) {
        size = OS_MEM_NODE_GET_SIZE(sentinel->sizeAndFlag);
        node = OsMemSentinelNodeGet(sentinel);
        sentinel = OS_MEM_END_NODE(node, size);
        count += size;
    }
#endif
    return count;
}*/
pub unsafe fn LOS_MemPoolSizeGet ->UINT32(pool:*const VOID){
    let mut count:UINT32 =32;
    if pool ==std::ptr::null_mut() {
        LOS_NOK;
    }

    count =count+ (pool as *mut OsMemPoolHead)->info.totalSize;
    #[cfg(LOSCFG_MEM_MUL_REGIONS==1)]{
        count =count-(pool as *mut OsMemPoolHead)->info.totalGapSize;
    }
    #[cfg(OS_MEM_EXPAND_ENABLE)]{
        let mut size:UINT32;
        let mut node:*mut OsMemNodeHead=std::ptr::null_mut();
        let mut sentinel:*mut OsMemNodeHead=os_mem_end_node(pool,(count as usize));
        //fn os_mem_end_node(pool: *const VOID, size: usize) -> *const OsMemNodeHead 
        while os_mem_is_last_sentinel_node(sentinel)==false {
            //fn os_mem_is_last_sentinel_node(sentinel_node: *mut OsMemNodeHead) -> bool
            size=os_mem_node_get_size(sentinel->sizeAndFlag);
            //fn os_mem_node_get_size(size_and_flag: UINT32) -> UINT32
            ////fn os_mem_sentinel_node_get(node: &OsMemNodeHead) -> *mut u8 {
            node=os_mem_sentinel_node_get(&(*sentinel));
            sentinel=os_mem_end_node((node as *const VOID),(size as usize));
            count =count+size;
        }       
    }
    count;
}

/*STATIC VOID MemUsedGetHandle(struct OsMemNodeHead *curNode, VOID *arg)
{
    UINT32 *memUsed = (UINT32 *)arg;
    if (OS_MEM_IS_GAP_NODE(curNode)) {
        *memUsed += OS_MEM_NODE_HEAD_SIZE;
    } else if (OS_MEM_NODE_GET_USED_FLAG(curNode->sizeAndFlag)) {
        *memUsed += OS_MEM_NODE_GET_SIZE(curNode->sizeAndFlag);
    }
    return;
}*/
//????static??
pub unsafe fn MemUsedGetHandle(curNode:&OsMemNodeHead,arg:*mut VOID){//什么时候用引用，什么时候用指针
    let mut memUsed:*mut UINT32=arg as *mut UINT32;
    //fn os_mem_is_gap_node(node: &OsMemNodeHead) -> bool
    if os_mem_is_gap_node(&curNode) {
        *memUsed += OS_MEM_NODE_HEAD_SIZE;
    } else if OS_MEM_NODE_GET_USED_FLAG(curNode.sizeAndFlag) {
        *memUsed += OS_MEM_NODE_GET_SIZE(curNode.sizeAndFlag);
    }
}

/*UINT32 LOS_MemTotalUsedGet(VOID *pool)
{
    UINT32 memUsed = 0;

    if (pool == NULL) {
        return LOS_NOK;
    }

    OsAllMemNodeDoHandle(pool, MemUsedGetHandle, (VOID *)&memUsed);

    return memUsed;
}*/

//pub重载可见性，表示该项是公开的，不只在函数所在的模块（mod x)中可用，可以从模块外的作用域访问。
//    pub(crate)表示项对于整个crate是可见的，但对crate的外部代码是不可见的
//crate可以是一个库，可以是一个编译单元，有唯一的名称标识   模块化，封装
//C语言中函数前的static表示为内部函数，只能在当前文件中访问
////！！！！！！！！！调用unsafe的函数的时候 要unsafe {  f();  }
unsafe fn LOS_MemTotalUsedGet ->UINT32(pool:*mut VOID){
    let mut memUsed:UINT32=0;
    if pool==std::ptr::null_mut() {
        LOS_NOK;
    }
    //pub fn OsAllMemNodeDohandle(pool: &mut OsMemPoolHead,handle: fn(*const OsMemNodeHead,*const VOID)->*const VOID,arg:*const VOID){ //可以不用加上结构体前缀"struct"
    OsAllMemNodeDohandle((pool as &mut OsMemPoolHead),MemUsedGetHandle,&memUsed as *const VOID);
    memUsed;
}


/*STATIC INLINE VOID OsMemMagicCheckPrint(struct OsMemNodeHead **tmpNode)
{
#if (LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)
    PRINT_ERR("[%s], %d, memory check error!\n"
              "memory used but magic num wrong, magic num = 0x%x\n",
              __FUNCTION__, __LINE__, (*tmpNode)->magic);
#else
    (VOID)tmpNode;
#endif
}*/
#[inline]
pub unsafe fn OsMemMagicCheckPrint ->VOID(tmpNode:*const *mut OsMemNodeHead){
    if cfg!(feature = "LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK"){
        println!("[%s], %d, memory check error!\n"
        "memory used but magic num wrong, magic num = 0x%x\n",
        file!(), line!(), (*tmpNode)->magic);
    }
    /*#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK==0)]{
        tmpNode as VOID;//return
    }*/
}

/*STATIC UINT32 OsMemAddrValidCheckPrint(const VOID *pool, struct OsMemFreeNodeHead **tmpNode)
{
    if (((*tmpNode)->prev != NULL) && !OsMemAddrValidCheck(pool, (*tmpNode)->prev)) {
        PRINT_ERR("[%s], %d, memory check error!\n"
                  " freeNode.prev: %p is out of legal mem range\n",
                  __FUNCTION__, __LINE__, (*tmpNode)->prev);
        return LOS_NOK;
    }
    if (((*tmpNode)->next != NULL) && !OsMemAddrValidCheck(pool, (*tmpNode)->next)) {
        PRINT_ERR("[%s], %d, memory check error!\n"
                  " freeNode.next: %p is out of legal mem range\n",
                  __FUNCTION__, __LINE__, (*tmpNode)->next);
        return LOS_NOK;
    }
    return LOS_OK;
}*/
//static??????
pub unsafe fn OsMemAddrValidCheckPrint->UINT32(pool:*const VOID,tmpNode:*const *mut OsMemFreeNodeHead){
    /// //pub(crate) fn os_mem_addr_valid_check(pool: *const OsMemPoolHead, addr: *const c_void) -> bool {
    if ((*tmpNode)->prev != std::ptr::null_mut()) && !os_mem_addr_valid_check(pool as *const OsMemPoolHead, (*tmpNode)->prev as *const c_void) {
        ////////直接写pool会自动强制类型转换吗    需要手动转换类型
        println!("[%s], %d, memory check error!\n"
        " freeNode.prev: %p is out of legal mem range\n",
        file!(), line!(), (*tmpNode)->prev);
        return LOS_NOK;
    }
    if ((*tmpNode)->next != std::ptr::null_mut()) && !os_mem_addr_valid_check(pool as *const OsMemPoolHead, (*tmpNode)->next as *const c_void) {
        println!("[%s], %d, memory check error!\n"
        " freeNode.next: %p is out of legal mem range\n",
        file!(), line!(), (*tmpNode)->next);
        return LOS_NOK;
    }
    LOS_OK;
}

/*STATIC UINT32 OsMemIntegrityCheckSub(struct OsMemNodeHead **tmpNode, const VOID *pool)
{
    if (!OS_MEM_MAGIC_VALID(*tmpNode)) {
        OsMemMagicCheckPrint(tmpNode);
        return LOS_NOK;
    }

    if (!OsMemAddrValidCheck(pool, (*tmpNode)->ptr.prev)) {
        PRINT_ERR("[%s], %d, memory check error!\n"
                  " node prev: %p is out of legal mem range\n",
                  __FUNCTION__, __LINE__, (*tmpNode)->ptr.next);
        return LOS_NOK;
    }

    if (!OS_MEM_NODE_GET_USED_FLAG((*tmpNode)->sizeAndFlag)) { /* is free node, check free node range */
        if (OsMemAddrValidCheckPrint(pool, (struct OsMemFreeNodeHead **)tmpNode)) {
            return LOS_NOK;
        }
    }

    return LOS_OK;
}*/

//在一个unsafe函数中调用另一个unsafe函数不用再加unsafe{}
pub unsafe fn OsMemIntegrityCheckSub ->UINT32(tmpNode:&mut *mut OsMemNodeHead,pool:*const VOID){/////
    if !os_mem_magic_valid(*tmpNode) {//////?????//fn os_mem_magic_valid(node: &OsMemNodeHead) -> bool {
        OsMemMagicCheckPrint(tmpNode as *const *mut OsMemNodeHead);
        return LOS_NOK;
    }
    if !os_mem_addr_valid_check(pool as *const OsMemPoolHead,(*tmpNode)->ptr.prev as *const c_void) {
        println!("[%s], %d, memory check error!\n"
        " node prev: %p is out of legal mem range\n",file!(), line!(), (*tmpNode)->ptr.next);
        return LOS_NOK;
    }
    //fn os_mem_node_get_used_flag(size_and_flag: UINT32) -> UINT32 
    if !os_mem_node_get_used_flag((*tmpNode)->sizeAndFlag) {
        if OsMemAddrValidCheckPrint(pool,tmpNode as *const *mut OsMemNodeHead) {
            return LOS_NOK;
        }
    }
    LOS_OK;
}

/*STATIC UINT32 OsMemFreeListNodeCheck(const struct OsMemPoolHead *pool,
                const struct OsMemFreeNodeHead *node)
{
    if (!OsMemAddrValidCheck(pool, node) ||
        ((node->prev != NULL) && !OsMemAddrValidCheck(pool, node->prev)) ||
        ((node->next != NULL) && !OsMemAddrValidCheck(pool, node->next)) ||
        !OsMemAddrValidCheck(pool, node->header.ptr.prev)) {
        return LOS_NOK;
    }

    if (!OS_MEM_IS_ALIGNED(node, sizeof(VOID *)) ||
        !OS_MEM_IS_ALIGNED(node->prev, sizeof(VOID *)) ||
        !OS_MEM_IS_ALIGNED(node->next, sizeof(VOID *)) ||
        !OS_MEM_IS_ALIGNED(node->header.ptr.prev, sizeof(VOID *))) {
        return LOS_NOK;
    }

    return LOS_OK;
}*/
pub unsafe fn OsMemFreeListNodeCheck ->UINT32(pool:*const OsMemPoolHead,node:*const OsMemFreeNodeHead){
    //pub(crate) fn os_mem_addr_valid_check(pool: *const OsMemPoolHead, addr: *const c_void) -> bool {
    //手动转换类型
    if os_mem_addr_valid_check(pool,node as *const c_void) ||
        ((node->prev != std::ptr::null_mut()) && !os_mem_addr_valid_check(pool, node->prev as *const c_void)) ||
        ((node->next != std::ptr::null_mut()) && !os_mem_addr_valid_check(pool, node->next as *const c_void))||
        !os_mem_addr_valid_check(pool,node->header.ptr.prev as *const c_void) {
            return LOS_NOK;
        }
        //fn os_mem_is_aligned(a: UINT32, b: usize)
    if !os_mem_is_aligned(node as UINT32,std::mem::size_of::<*mut VOID>()) ||
    !os_mem_is_aligned(node->prev as UINT32,std::mem::size_of::<*mut VOID>()) ||
    !os_mem_is_aligned(node->next as UINT32,std::mem::size_of::<*mut VOID>()) ||
    !os_mem_is_aligned(node->header.ptr.prev as UINT32,std::mem::size_of::<*mut VOID>()) {
        return LOS_NOK;
    }
    LOS_OK;
}

/*STATIC VOID OsMemPoolHeadCheck(const struct OsMemPoolHead *pool)
{
    struct OsMemFreeNodeHead *tmpNode = NULL;
    UINT32 index;
    UINT32 flag = 0;

    if ((pool->info.pool != pool) || !OS_MEM_IS_ALIGNED(pool, sizeof(VOID *))) {
        PRINT_ERR("wrong mem pool addr: %p, func: %s, line: %d\n", pool, __FUNCTION__, __LINE__);
        return;
    }

    for (index = 0; index < OS_MEM_FREE_LIST_COUNT; index++) {
        for (tmpNode = pool->freeList[index]; tmpNode != NULL; tmpNode = tmpNode->next) {
            if (OsMemFreeListNodeCheck(pool, tmpNode)) {
                flag = 1;
                PRINT_ERR("FreeListIndex: %u, node: %p, bNode: %p, prev:%p, next: %p\n",
                          index, tmpNode, tmpNode->header.ptr.prev, tmpNode->prev, tmpNode->next);
            }
        }
    }
    /*
    改为 
    index=0
    while(index <OS_MEM_FREE_LIST_COUNT) {
        ...
        index++;
    }
         */

    if (flag) {
        PRINTK("mem pool info: poolAddr: %p, poolSize: 0x%x\n", pool, pool->info.totalSize);
#if (LOSCFG_MEM_WATERLINE == 1)
        PRINTK("mem pool info: poolWaterLine: 0x%x, poolCurUsedSize: 0x%x\n", pool->info.waterLine,
               pool->info.curUsedSize);
#endif
#if OS_MEM_EXPAND_ENABLE
        UINT32 size;
        struct OsMemNodeHead *node = NULL;
        struct OsMemNodeHead *sentinel = OS_MEM_END_NODE(pool, pool->info.totalSize);
        while (OsMemIsLastSentinelNode(sentinel) == FALSE) {
            size = OS_MEM_NODE_GET_SIZE(sentinel->sizeAndFlag);
            node = OsMemSentinelNodeGet(sentinel);
            sentinel = OS_MEM_END_NODE(node, size);
            PRINTK("expand node info: nodeAddr: 0x%x, nodeSize: 0x%x\n", node, size);
        }
#endif
    }
}*/
pub unsafe fn OsMemPoolHeadCheck(pool:*const OsMemPoolHead){
    let mut tmpNode:*mut OsMemFreeNodeHead=std::ptr::null_mut();
    let mut index:UINT32;
    let mut flag:UINT32=0;
    if (pool->info.pool != pool) || !os_mem_is_aligned(pool as UINT32, std::mem::size_of::<*mut VOID>()) {
        println!("wrong mem pool addr: %p, func: %s, line: %d\n", pool, file!(), line!());//??_FUNCTION_
        return;
    }
    indes=0;
    while index<OS_MEM_FREE_LIST_COUNT {
        tmpNode = pool->freeList[index];
        while tmpNode !=std::ptr::null_mut() {
            //pub unsafe fn OsMemFreeListNodeCheck ->UINT32(pool:*const OsMemPoolHead,node:*const OsMemFreeNodeHead){
            if OsMemFreeListNodeCheck(pool,tmpNode as *const OsMemFreeNodeHead) {
                flag = 1;
                println!("FreeListIndex: %u, node: %p, bNode: %p, prev:%p, next: %p\n",
                          index, tmpNode, tmpNode->header.ptr.prev, tmpNode->prev, tmpNode->next);
            }
            tmpNode = tmpNode->next;
        }

        index=index+1;
    }

    if flag {///////????PRINTK,PRINTERR要换成println!吗？？
        println!("mem pool info: poolAddr: %p, poolSize: 0x%x\n", pool, pool->info.totalSize);
        #[cfg(LOSCFG_MEM_WATERLINE == 1)] {
            println!("mem pool info: poolWaterLine: 0x%x, poolCurUsedSize: 0x%x\n", pool->info.waterLine,
               pool->info.curUsedSize);
        }
        #[cfg(OS_MEM_EXPAND_ENABLE)] {
            let mut size:UINT32;
            let mut node:*mut OsMemNodeHead=std::ptr::null_mut();
            let mut sentinel:*mut OsMemNodeHead=os_mem_end_node(pool as *const VOID, pool->info.totalSize as usize);
            while os_mem_is_last_sentinel_node(sentinel)==false {
                size = os_mem_node_get_size(sentinel->sizeAndFlag);
                //fn os_mem_sentinel_node_get(node: &OsMemNodeHead) -> *mut u8 {
                node = os_mem_sentinel_node_get(&(*sentinel));
                sentinel = os_mem_end_node(node as *const VOID, size as usize);
                println!("expand node info: nodeAddr: 0x%x, nodeSize: 0x%x\n", node, size);
            }
        }
    }
}

