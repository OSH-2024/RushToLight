/*VOID OsTaskMemUsed(VOID *pool, UINT32 *tskMemInfoBuf, UINT32 tskMemInfoCnt)
{
    UINT32 args[2] = {(UINT32)(UINTPTR)tskMemInfoBuf, tskMemInfoCnt};
    OsAllMemNodeDoHandle(pool, GetTaskMemUsedHandle, (VOID *)args);
    return;
}
#endif*/
fn OsTaskMemUsed( pool:&mut,tskMemInfoBuf:&UINT32,tskMemInfoCnt:UINT32){//void*pool????
    let mut args :[UINT32;2]=[tskMemInfoBuf,tskMemInfoCnt];
    OsAllMemNodeDoHandle(&pool,&GetTaskMemUsedHandle,&args);//?????(VOID*)args
    //STATIC VOID OsAllMemNodeDoHandle(VOID *pool, VOID (*handle)(struct OsMemNodeHead *curNode, VOID *arg)
}

//#endif


/*#if (LOSCFG_MEM_WATERLINE == 1)
STATIC INLINE VOID OsMemWaterUsedRecord(struct OsMemPoolHead *pool, UINT32 size)
{
    pool->info.curUsedSize += size;
    if (pool->info.curUsedSize > pool->info.waterLine) {
        pool->info.waterLine = pool->info.curUsedSize;
    }
}
#else
STATIC INLINE VOID OsMemWaterUsedRecord(struct OsMemPoolHead *pool, UINT32 size)//inline内联函数
{//定义了一个静态内联函数，用于记录内存池中已使用的内存大小，但是实际上函数体中并没有对参数进行任何处理，
    可能是为了占位或者待以后扩展功能。
    (VOID)pool;
    (VOID)size;//将传入的参数pool和size强制转换为VOID类型，这样做的目的可能是为了避免编译器产生未使用参数的警告。
}
#endif*/
#[cfg(LOSCFG_MEM_WATERLINE == 1)]//#[cfg()] 属性是在编译时进行判断和处理的，不会在运行时起作用。
#[inline]
pub fn OsMemWaterUsedRecord(pool:&mut OsMemPoolHead,size:UINT32){
    //static inline void  static将函数的有效范围限制在文件内部，防止因Include函数多重定义，inline内联函数
    //pub公共对外部可见
    pool.info.curUsedSize +=size;
    if pool.info.curUsedSize>pool.info.waterLine {
        pool.info.waterLine=pool.info.curUsedSize;//是mut吗？
    }
}
#[cfg(LOSCFG_MEM_WATERLINE == 0)]
#[inline]
pub fn OsMemWaterUsedRecord(pool:&mut OsMemPoolHead,size:UINT32){////空
    let VOID::Pool(pool) = pool.into();//VOID在los_config.rs中用pub type定义了
    // 将 u32 类型的 pool 转换为 VOID 类型，并使用模式匹配和变量重命名
    let VOID::Size(size)=size.into();
}

/*#if OS_MEM_EXPAND_ENABLE
STATIC INLINE struct OsMemNodeHead *OsMemLastSentinelNodeGet(const struct OsMemNodeHead *sentinelNode)
{
    struct OsMemNodeHead *node = NULL;
    VOID *ptr = sentinelNode->ptr.next;
    UINT32 size = OS_MEM_NODE_GET_SIZE(sentinelNode->sizeAndFlag);

    while ((ptr != NULL) && (size != 0)) {
        node = OS_MEM_END_NODE(ptr, size);
        ptr = node->ptr.next;
        size = OS_MEM_NODE_GET_SIZE(node->sizeAndFlag);
    }

    return node;
}*/
/*pub struct OsMemNodeHead {
    pub ptr: OsMemNodeHeadUnion,
    pub size_and_flag: UINT32,
}
pub union OsMemNodeHeadUnion {
    pub prev: *mut OsMemNodeHead,
    pub next: *mut OsMemNodeHead,
}*/
#[cfg(OS_MEM_EXPAND_ENABLE)]
#[inline]
pub fn OsMemLastSentinelNodeGet ->OsMemNodeHead(sentinelNode :&OsMemNodeHead){//这里返回值不加&比较好？？
    //sentinelNode的所有权不从原来函数中移走，const怎么体现？？
    //const struct OsMemNodeHead *sentinelNode  不可变引用？？
    let mut node:OsMemNodeHead=OsMemNodeHead::new();//??
    let mut ptr=sentinelNode.ptr.next;
    let mut size:UINT32=OS_MEM_NODE_GET_SIZE(sentinelNode.sizeAndFlag);/////////
    //#define OS_MEM_NODE_GET_SIZE(sizeAndFlag) ((sizeAndFlag) & ~OS_MEM_NODE_ALIGNED_AND_USED_FLAG)
    while ptr&&size {
        node=OS_MEM_END_NODE(ptr,size);////////////
        //#define OS_MEM_END_NODE(pool, size) (struct OsMemNodeHead *)((UINT8 *)(pool) + (size) - OS_MEM_NODE_HEAD_SIZE)
        ptr=node.ptr.next;
        size =OS_MEM_NODE_GET_SIZE(node.sizeAndFlag);
    }
    return node;
}

/*STATIC INLINE BOOL OsMemSentinelNodeCheck(struct OsMemNodeHead *sentinelNode)
{
    if (!OS_MEM_NODE_GET_USED_FLAG(sentinelNode->sizeAndFlag)) {
        return FALSE;
    }

    if (!OS_MEM_MAGIC_VALID(sentinelNode)) {
        return FALSE;
    }

    return TRUE;
}*/

#[inline]
pub fn OsMemSentinelNodeCheck ->bool(sentinelNode:&OsMemNodeHead){
    if !OS_MEM_NODE_GET_USED_FLAG(sentinelNode.sizeAndFlag) {/////////////#define OS_MEM_NODE_GET_USED_FLAG(sizeAndFlag) ((sizeAndFlag) & OS_MEM_NODE_USED_FLAG)
        false;
    }
    if !OS_MEM_MAGIC_VALID(sentinelNode){//////////////#define OS_MEM_MAGIC_VALID(node)    TRUE
        false;
    }
    true;
}