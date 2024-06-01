//向内存池中的空闲节点链表中添加新的空闲节点
#[inline]
pub fn OsMemFreeNodeAdd(pool:&mut VOID,node:&mut OsMemFreeNodeHead){
    let mut index:UINT32=OsMemFreeListIndexGet(node.header.sizeAndFlag);
    if(index>=OS_MEM_FREE_LIST_COUNT){
        LOS_Panic("The index of free lists is error, index = %u\n", index);
    }
    OsMemListAdd(&pool, index, &node);
}

//从内存池中的空闲节点链表中删除指定的空闲节点
#[inline]
pub fn OsMemFreeNodeDelete(pool:&mut VOID,mode:&mut OsMemFreeNodeHead){
    let mut index:UINT32=OsMemFreeListIndexGet(node.header.sizeAndFlag);
    OsMemListDelete(&pool,index,&node);
}

//从给定的内存池中获取一个足够大的空闲节点，并将其从空闲节点链表中删除  ,返回这个空闲节点的指针
#[inline]////////////////////////////////////////////////////////
pub unsafe fn OsMemFreeNodeGet ->*mut OsMemNodeHead(pool:*mut VOID,size:UINT32){//返回值为结构体指针？？
    let mut pool:*mut VOID;//c_void就是VOID    新定义的pool与参数不是一个pool
    let mut poolHead:*mut OsMemPoolHead=pool as *mut OsMemPoolHead;//pool强制类型转换为OsMemPoolHead 
    let mut index:UINT32;
    //let mut firstNode:OsMemFreeNodeHead =OsMemFindNextSuitableBlock(&pool,size,&index);
    let mut firstNode:*mut OsMemPoolHead = match OsMemFindNextSuitableBlock(&pool, size,&index) {
        //C中返回struct OsMemFreeNodeHead *，rust中假设返回OsMemFreeNodeHead
        Some(node) => node,
        None => return ptr::null_mut(),  //option中的some(T)表示一个值，None表示没有值///////////////////////
    };

    
    OsMemListDelete(&mut *poolHead, index, &mut *firstNode);///要看rs代码
    
    return firstNode->header;//这是个结构体指针，怎么变为结构体   或是让函数返回值还为结构体指针？？？
}

//合并相邻的内存块
#[inline]
pub unsafe fn OsMemMergeNode(node:&mut OsMemNodeHead){//用了裸指针，加unsafe
    let mut nextNode:*mut OsMemNodeHead=std::ptr::null_mut();
    node.ptr.prev->sizeAndFlag +=node.sizeAndFlag;
    //let nextNode:&mut OsMemNodeHead=None;//////////////////
    //sizeAndFlag是UINT32  los_memory_h.rs中pub type UINTPTR = ::std::os::raw::c_uint;
    let mut temp:UINT32=(node as UINTPTR)+node.sizeAndFlag;
    nextNode=temp as *mut OsMemNodeHead;
    if !OS_MEM_NODE_GET_LAST_FLAG(nextNode->sizeAndFlag)&&!OS_MEM_IS_GAP_NODE(nextNode)
    {
        nextNode->ptr.prev=node.ptr.prev;
    }

}

//将一个已分配的内存块分割成两个部分：一个部分是已分配的内存块，另一个部分是空闲的内存块
#[inline]
pub unsafe fn OsMemSplitNode(pool:&mut VOID,allocNode:&mut OsMemNodeHead,allocSize:UINT32){
    let mut newFreeNode:*mut OsMemFreeNodeHead=std::ptr::null_mut();
    let mut nextNode:*mut OsMemNodeHead=std::ptr::null_mut();
    newFreeNode=(((allocNode as *mut UINT8)+allocSize) as *mut VOID) as *mut OsMemFreeNodeHead;
    newFreeNode->header.ptr.prev = allocNode;
    newFreeNode->header.sizeAndFlag = allocNode.sizeAndFlag - allocSize;
    allocNode.sizeAndFlag = allocSize;
    if !OS_MEM_NODE_GET_LAST_FLAG(nextNode->sizeAndFlag) && !OS_MEM_IS_GAP_NODE(nextNode)
    {
        OsMemFreeNodeDelete(&pool,&mut *(nextNode as *mut OsMemFreeNodeHead));//&mut在引用
        OsMemMergeNode(&mut *nextNode);
    }
    OsMemFreeNodeAdd(&pool,&mut *newFreeNode);
}

//创建一个被使用的内存节点
#[inline]
pub unsafe fn OsMemCreateUsedNode ->&mut VOID(addr:&mut VOID){
    let mut node:*mut OsMemUsedNodeHead=addr as *mut OsMemUsedNodeHead;
#[cfg(LOSCFG_MEM_FREE_BY_TASKID == 1 || LOSCFG_TASK_MEM_USED == 1)]
{
    OsMemNodeSetTaskID(&mut *node);
}
#[cfg(LOSCFG_KERNEL_LMS)]
{
    let mut newNode:*mut OsMemNodeHead=node as *mut OsMemNodeHead;
    /////////g_lms没有检索到，一个结构体
    if g_lms != std::ptr::null_mut()
    {
        g_lms->mallocMark(&mut *newNode, OS_MEM_NEXT_NODE(&(*newNode)), OS_MEM_NODE_HEAD_SIZE);
    }
}
}

//初始化一个内存池
#[inline]
pub unsafe fn OsMemPoolInit->UINT32(pool:*mut VOID,size:UINT32){//pool是指针，不是引用
    let mut poolHead:*mut OsMemPoolHead=pool as *mut OsMemPoolHead;
    let mut newNode:*mut OsMemPoolHead=std::ptr::null_mut();
    let mut endNode:*mut OsMemPoolHead=std::ptr::null_mut();
    //没检索到memset_s
    memset_s(&mut *poolHead, size, 0, std::mem::size_of::<OsMemPoolHead>());
#[cfg(LOSCFG_KERNEL_LMS)]
{
    let mut resize:UINT32=0;
    if g_lms != std::ptr::null_mut() {
        resize = g_lms->init(&pool, size);
        size = if resize == 0 { size } else { resize };
    }
}
    
    poolHead->info.pool = pool;
    poolHead->info.totalSize = size;
    /*下面default attr: lock, not expand. 指定了内存池的默认属性是锁定的，并且不会进行扩展。*/
    poolHead->info.attr &= ~(OS_MEM_POOL_UNLOCK_ENABLE | OS_MEM_POOL_EXPAND_ENABLE);

    newNode = OS_MEM_FIRST_NODE(&(*pool));
    newNode->sizeAndFlag = (size - std::mem::size_of::<OsMemPoolHead>() - OS_MEM_NODE_HEAD_SIZE);
    newNode->ptr.prev = OS_MEM_END_NODE((pool as *const VOID), (size as usize));
    //fn os_mem_end_node(pool: *const VOID, size: usize) -> *const OsMemNodeHead 
    OS_MEM_SET_MAGIC(&mut *newNode);
    OsMemFreeNodeAdd(&mut pool, &mut *(newNode as *mut OsMemFreeNodeHead));

    /* The last mem node */
    endNode = OS_MEM_END_NODE((pool as *const VOID), (size as usize)) as *mut OsMemPoolHead;//返回值为*const OsMemNodeHead
    OS_MEM_SET_MAGIC(&mut *endNode);
#[cfg(OS_MEM_EXPAND_ENABLE==1)]
{
    endNode->ptr.next = std::ptr::null_mut();
    OsMemSentinelNodeSet(&mut *endNode, std::ptr::null_mut(), 0);
    //fn os_mem_sentinel_node_set(sentinel_node: &mut OsMemNodeHead, new_node: Option<Box<OsMemNodeHead>>, size: usize) 
}
#[cfg(OS_MEM_EXPAND_ENABLE==0)]
{
    endNode->sizeAndFlag = 0;
    endNode->ptr.prev = newNode;
    OS_MEM_NODE_SET_USED_FLAG(endNode->sizeAndFlag);
}
#[cfg(LOSCFG_MEM_WATERLINE == 1)]
{
    poolHead->info.curUsedSize = std::mem::size_of::<OsMemPoolHead>()+ OS_MEM_NODE_HEAD_SIZE;
    poolHead->info.waterLine = poolHead->info.curUsedSize;
}

#[cfg(LOSCFG_KERNEL_LMS)]
{
    if resize != 0 {
        OsLmsFirstNodeMark(&mut pool, &(*newNode));/////////要看rs代码
        //STATIC INLINE VOID OsLmsFirstNodeMark(VOID *pool, struct OsMemNodeHead *node)
    }
}
    LOS_OK;
}


#[cfg(LOSCFG_MEM_MUL_POOL == 1)]
{
//释放内存池所占用的资源
#[inline]
pub fn OsMemPoolDeInit(pool:&mut VOID,size:UINT32){
#[cfg(LOSCFG_KERNEL_LMS)]
{
    if g_lms != std::ptr::null_mut(){
        g_lms->deInit(pool);//没找到g_lms
    }
}
    memset_s(&mut *pool, size, 0,std::mem::size_of::<OsMemPoolHead>());
}

//向内存池链表中添加新的内存池
#[inline]
pub unsafe fn OsMemPoolAdd -> UINT32(pool:*mut VOID,size:UINT32){
    let mut nextPool:*mut VOID=G_POOL_HEAD;
    let mut curPool:*mut VOID=G_POOL_HEAD;
    let mut poolEnd:UINTPTR;
    while nextPool != std::ptr::null_mut(){
        poolEnd=(nextPool as UINTPTR)+LOS_MemPoolSizeGet((&pool));
        //extern UINT32 LOS_MemPoolSizeGet(const VOID *pool);在los_memory.h中    没找到rs代码
        if ((pool <= nextPool) && (((pool as UINTPTR) + size) > (nextPool as UINTPTR))) ||
        (((pool as UINTPTR) < poolEnd) && (((pool as UINTPTR) + size) >= poolEnd))
        {
            //检索不到PRINT_ERR
            println!("pool [0x%x, 0x%x) conflict with pool [0x%x, 0x%x)\n", pool as UINTPTR,
                      (pool as UINTPTR) + size, (nextPool as UINTPTR), (nextPool as UINTPTR) + LOS_MemPoolSizeGet(&nextPool));
            return LOS_NOK;
        }
        curPool = nextPool;
        nextPool = (nextPool as *mut OsMemPoolHead)->nextPool;
    }

    if G_POOL_HEAD==std::ptr::null_mut(){
        G_POOL_HEAD=pool;
    }else {
        (curPool as *mut OsMemPoolHead)->nextPool = pool;
    }
    (pool as *mut OsMemPoolHead)->nextPool = NULL;
    return LOS_OK;
}

//150行对应的#if (LOSCFG_MEM_MUL_POOL == 1)还没有#defif 