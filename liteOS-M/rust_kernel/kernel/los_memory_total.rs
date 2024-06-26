mod los_memory_h.rs;
mod securec.rs;
mod los_arch.rs;
mod los_config.rs;
mod los_debug.rs;
mod los_hook.rs;
mod los_interrupt.rs;
mod los_task.rs;

#[cfg(feature = "LOSCFG_KERNEL_LMS")]
mod los_lms_pri;

#[cfg(feature = "LOSCFG_KERNEL_LMK")]
mod los_lmk;

// 定义用于在编译时控制是否启用某些非必要功能。
const OS_MEM_EXPAND_ENABLE: u32 = 0;

// 系统内存的起始地址
let mut m_aucSysMem0: *mut u8 = std::ptr::null_mut();

// g_memStart是一个数组，作为系统堆的存储空间
#[cfg(not(feature = "LOSCFG_SYS_EXTERNAL_HEAP"))]
static mut g_memStart: [u8; LOSCFG_SYS_HEAP_SIZE/* 在los_memory_h.rs里定义 */] = [0; LOSCFG_SYS_HEAP_SIZE/* 在los_memory_h.rs里定义 */]; // 初始化数组元素为0

// g_poolHead用于存储多个内存池的头部，如果配置为支持多内存池
#[cfg(feature = "LOSCFG_MEM_MUL_POOL")]
static mut g_poolHead: *mut c_void = std::ptr::null_mut();

// TLSF相关宏定义和内联函数，TLSF（Two-Level Segregated Fit）
const OS_MEM_BITMAP_MASK: u32 = 0x1F;

// 使用Rust库中的leading_zeros()函数替代原来的clt函数
// 计算bitmap中最高位1的位置，CLZ 表示统计前导零的位数
fn clz(bitmap: u32) -> u32 {
    bitmap.leading_zeros()
}

// 内联函数，用于找到位图中第一个设置为1的位
#[inline]
pub fn OsMemFFS(bitmap: u32) -> u16 {
    let new_bitmap = bitmap & !(bitmap + 1);
    OS_MEM_BITMAP_MASK - clz(new_bitmap) as u16
}

// 内联函数，用于找到位图中最后一个设置为1的位
#[inline]
pub fn OsMemFLS(bitmap: u32) -> u16 {
    OS_MEM_BITMAP_MASK - clz(bitmap) as u16
}

// 计算给定大小的对数
#[inline]
pub fn OsMemLog2(size: u32) -> u32 {
    if size > 0 {
        OsMemFLS(size) as u32
    } else {
        0
    }
}

#[inline]
pub fn OsMemFlGet(size: u32) -> u32 {
    if size < OS_MEM_SMALL_BUCKET_MAX_SIZE /* 在los_memory_h.rs里定义 */ {
        (size >> 2) - 1 /* 2: The small bucket setup is 4. */
    } else {
        OsMemLog2(size) - OS_MEM_LARGE_START_BUCKET /* 在los_memory_h.rs里定义 */ + OS_MEM_SMALL_BUCKET_COUNT /* 在los_memory_h.rs里定义 */
    }
}

// 根据给定的大小和桶级别计算内存请求大小所在的桶的级别
pub fn OsMemSlGet(size: u32, fl: u32) -> u32 {
    if fl < OS_MEM_SMALL_BUCKET_COUNT /* 在los_memory_h.rs里定义 */ || size < OS_MEM_SMALL_BUCKET_MAX_SIZE /* 在los_memory_h.rs里定义 */ {
        println!("fl or size is too small, fl = {}, size = {}", fl, size);
        return 0;
    }

    let sl = (size << OS_MEM_SLI /* 在los_memory_h.rs里定义 */) >> (fl - OS_MEM_SMALL_BUCKET_COUNT /* 在los_memory_h.rs里定义 */ + OS_MEM_LARGE_START_BUCKET /* 在los_memory_h.rs里定义 */);
    sl - (1 << OS_MEM_SLI /* 在los_memory_h.rs里定义 */)
}

// 在满足指定条件时会触发错误
#[cfg(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID", any(feature = "LOSCFG_BASE_CORE_TSK_LIMIT_64")))]
compile_error!("When enter here, LOSCFG_BASE_CORE_TSK_LIMIT larger than 63 is not supported");


struct OsMemUsedNodeHead {      //只在los_memory.c中用到，因此不必声明为pub类型
    pub header: OsMemNodeHead,
}

const OS_MEM_POOL_EXPAND_ENABLE: u32 = 0x01; // 内存池支持扩展
const OS_MEM_POOL_UNLOCK_ENABLE: u32 = 0x02; // 内存池支持无锁操

//仅在los_memory.c里使用的宏转换为函数
fn MEM_LOCK(pool: &OsMemPoolHead, state: &mut u32) {
    if (*pool).info.attr & OS_MEM_POOL_UNLOCK_ENABLE == 0 {
        *state = LOS_IntLock();/*los_interrupt.h里 */
    }
}

fn MEM_UNLOCK(pool: &OsMemPoolHead, state: u32) {
    if (*pool).info.attr & OS_MEM_POOL_UNLOCK_ENABLE == 0 {
        LOS_IntRestore(state);/*los_interrupt.h里 */
    }
}

/* 内存节点魔术数字，用于检测内存节点的完整性 */
pub const OS_MEM_NODE_MAGIC: u32 = 0xABCDDCBA;

const OS_MEM_NODE_USED_FLAG_1: u32 = 1 << 25;
const OS_MEM_NODE_ALIGNED_FLAG_1: u32 = 1 << 24;
const OS_MEM_NODE_LEAK_FLAG_1: u32 = 1 << 23;
const OS_MEM_NODE_LAST_FLAG_1: u32 = 1 << 22;

const OS_MEM_NODE_USED_FLAG_2: u32 = 1 << 31;
const OS_MEM_NODE_ALIGNED_FLAG_2: u32 = 1 << 30;
const OS_MEM_NODE_LEAK_FLAG_2: u32 = 1 << 29;
const OS_MEM_NODE_LAST_FLAG_2: u32 = 1 << 28;

// 条件编译：LOSCFG_TASK_MEM_USED != 1 && LOSCFG_MEM_FREE_BY_TASKID == 1
#[cfg(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID"))]
const OS_MEM_NODE_USED_FLAG: u32 = OS_MEM_NODE_USED_FLAG_1;
#[cfg(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID"))]
const OS_MEM_NODE_ALIGNED_FLAG: u32 = OS_MEM_NODE_ALIGNED_FLAG_1;
#[cfg(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID"))]
#[cfg(feature = "LOSCFG_MEM_LEAKCHECK")]
const OS_MEM_NODE_LEAK_FLAG: u32 = OS_MEM_NODE_LEAK_FLAG_1;
#[cfg(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID"))]
#[cfg(not(feature = "LOSCFG_MEM_LEAKCHECK"))]
const OS_MEM_NODE_LEAK_FLAG: u32 = 0;
#[cfg(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID"))]
#[cfg(feature = "OS_MEM_EXPAND_ENABLE")]
const OS_MEM_NODE_LAST_FLAG: u32 = OS_MEM_NODE_LAST_FLAG_1; // Sentinel Node
#[cfg(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID"))]
#[cfg(not(feature = "OS_MEM_EXPAND_ENABLE"))]
const OS_MEM_NODE_LAST_FLAG: u32 = 0;

// 条件编译：其他情况
#[cfg(not(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID")))]
const OS_MEM_NODE_USED_FLAG: u32 = OS_MEM_NODE_USED_FLAG_2;
#[cfg(not(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID")))]
const OS_MEM_NODE_ALIGNED_FLAG: u32 = OS_MEM_NODE_ALIGNED_FLAG_2;
#[cfg(not(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID")))]
#[cfg(feature = "LOSCFG_MEM_LEAKCHECK")]
const OS_MEM_NODE_LEAK_FLAG: u32 = OS_MEM_NODE_LEAK_FLAG_2;
#[cfg(not(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID")))]
#[cfg(not(feature = "LOSCFG_MEM_LEAKCHECK"))]
const OS_MEM_NODE_LEAK_FLAG: u32 = 0;
#[cfg(not(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID")))]
#[cfg(feature = "OS_MEM_EXPAND_ENABLE")]
const OS_MEM_NODE_LAST_FLAG: u32 = OS_MEM_NODE_LAST_FLAG_2; // Sentinel Node
#[cfg(not(all(not(feature = "LOSCFG_TASK_MEM_USED"), feature = "LOSCFG_MEM_FREE_BY_TASKID")))]
#[cfg(not(feature = "OS_MEM_EXPAND_ENABLE"))]
const OS_MEM_NODE_LAST_FLAG: u32 = 0;

// 定义一个用于表示内存节点已用、对齐、泄漏和最后一个节点的标志的组合宏
const OS_MEM_NODE_ALIGNED_AND_USED_FLAG: usize = OS_MEM_NODE_USED_FLAG
    | OS_MEM_NODE_ALIGNED_FLAG
    | OS_MEM_NODE_LEAK_FLAG
    | OS_MEM_NODE_LAST_FLAG;

fn OS_MEM_NODE_GET_ALIGNED_FLAG(size_and_flag: u32) -> u32 {
    size_and_flag & OS_MEM_NODE_ALIGNED_FLAG
}
// 设置节点的对齐标记
fn OS_MEM_NODE_SET_ALIGNED_FLAG(size_and_flag: &mut u32) {
    *size_and_flag |= OS_MEM_NODE_ALIGNED_FLAG;
}

// 从节点大小和标记信息中获取已用标记
fn OS_MEM_NODE_GET_USED_FLAG(size_and_flag: u32) -> u32 {
    size_and_flag & OS_MEM_NODE_USED_FLAG
}

// 设置节点的已用标记
fn OS_MEM_NODE_SET_USED_FLAG(size_and_flag: &mut u32) {
    *size_and_flag |= OS_MEM_NODE_USED_FLAG;
}

// 获取节点的大小（去除标记位）
fn OS_MEM_NODE_GET_SIZE(size_and_flag: u32) -> u32 {
    size_and_flag & !OS_MEM_NODE_ALIGNED_AND_USED_FLAG
}

// 间隙大小的已用标记
const OS_MEM_GAPSIZE_USED_FLAG: u32 = 0x80000000;
// 间隙大小的对齐标记
const OS_MEM_GAPSIZE_ALIGNED_FLAG: u32 = 0x40000000;

// 获取对齐后的间隙大小
fn OS_MEM_GET_ALIGNED_GAPSIZE(gapsize: u32) -> u32 {
    gapsize & !OS_MEM_GAPSIZE_ALIGNED_FLAG
}

// 获取间隙大小的对齐标记
fn OS_MEM_GET_GAPSIZE_ALIGNED_FLAG(gapsize: u32) -> u32 {
    gapsize & OS_MEM_GAPSIZE_ALIGNED_FLAG
}

// 设置间隙大小的对齐标记
fn OS_MEM_SET_GAPSIZE_ALIGNED_FLAG(gapsize: &mut u32) {
    *gapsize |= OS_MEM_GAPSIZE_ALIGNED_FLAG;
}

// 获取间隙大小的已用标记
fn OS_MEM_GET_GAPSIZE_USED_FLAG(gapsize: u32) -> u32 {
    gapsize & OS_MEM_GAPSIZE_USED_FLAG
}

// 检查间隙大小的对齐和已用标记
fn OS_MEM_GAPSIZE_CHECK(gapsize: u32) -> bool {
    os_mem_get_gapsize_aligned_flag(gapsize) != 0 && os_mem_get_gapsize_used_flag(gapsize) != 0
}

// 设置节点为最后一个节点的标记
fn OS_MEM_NODE_SET_LAST_FLAG(size_and_flag: &mut u32) {
    *size_and_flag |= OS_MEM_NODE_LAST_FLAG;
}

// 获取节点是否为最后一个节点的标记
fn OS_MEM_NODE_GET_LAST_FLAG(size_and_flag: u32) -> u32 {
    size_and_flag & OS_MEM_NODE_LAST_FLAG
}

// 获取节点的泄漏标记
fn OS_MEM_NODE_GET_LEAK_FLAG(size_and_flag: u32) -> u32 {
    size_and_flag & OS_MEM_NODE_LEAK_FLAG
}

fn OS_MEM_NODE_SET_LEAK_FLAG(size_and_flag: &mut u32) {
    *size_and_flag |= OS_MEM_NODE_LEAK_FLAG;
}
// 基本内存对齐大小，通常是指针的大小
const OS_MEM_ALIGN_SIZE: usize = std::mem::size_of::<usize>();

// 检查一个值是否是2的幂
fn OS_MEM_IS_POW_TWO(value: u32) -> bool {
    let value1 = value as usize;
    (value1 & (value1 - 1)) == 0
}

// 将指针p按照align_size大小对齐
fn OS_MEM_ALIGN(p: u32, align_size: usize) -> usize { //使用usize进行相关指针操作，防止无定义行为
    let p1 = p as usize;
    (p1 + align_size - 1) & !(align_size - 1)
}

// 检查值 a 是否按照对齐大小 b 进行了对齐,a可能是基本整数类型，也有可能是指针类型，使用泛型T接收不同类型的参数
pub fn OS_MEM_IS_ALIGNED<T>(a: T, b: usize) -> bool {
    let a_addr = &a as *const T as usize;
    (a_addr & (b - 1)) == 0
}

// 内存节点头部的大小
const OS_MEM_NODE_HEAD_SIZE: usize = std::mem::size_of::<OsMemUsedNodeHead>();

// 内存池的最小大小，至少要容纳一个节点头部和一个内存池头部
const OS_MEM_MIN_POOL_SIZE: usize = OS_MEM_NODE_HEAD_SIZE + std::mem::size_of::<OsMemPoolHead>();

// 最小的剩余内存块大小，应能至少容纳一个空闲内存节点头部
const OS_MEM_MIN_LEFT_SIZE: usize = std::mem::size_of::<OsMemFreeNodeHead>();

// 最小的可分配内存大小
const OS_MEM_MIN_ALLOC_SIZE: u32 = 8;

/* 获取下一个内存节点的宏，通过当前节点的sizeAndFlag字段 */    
fn OS_MEM_NEXT_NODE(node: &OsMemNodeHead) -> Option<&OsMemNodeHead> {
    let next_node_offset = OS_MEM_NODE_GET_SIZE(node.sizeAndFlag) as isize;
    let next_node_ptr = (node as &u8).offset(next_node_offset) as &OsMemNodeHead;   
    next_node_ptr
}

// 定义获取内存池中第一个内存节点的函数
fn OS_MEM_FIRST_NODE(pool: &c_void) -> Option<&OsMemNodeHead> {
    let pool_head_size = std::mem::size_of::<OsMemPoolHead>() as isize;
    let first_node_ptr = (pool as &u8).offset(pool_head_size) as &OsMemNodeHead;
    first_node_ptr
}

// 定义获取内存池末尾节点的函数
fn OS_MEM_END_NODE(pool: &c_void, size: usize) -> Option<&OsMemNodeHead> {
    // 计算内存池末尾节点的指针
    let end_node_ptr = (pool as &u8).offset(size as isize - OS_MEM_NODE_HEAD_SIZE as isize) as &OsMemNodeHead;
    end_node_ptr
}

// 定义判断中间地址是否在开始和结束地址之间（不包含结束地址）的函数
fn OS_MEM_MIDDLE_ADDR_OPEN_END(start_addr: &OsMemPoolHead, middle_addr: &c_void, end_addr: u32) -> bool {
    let start_addr_ptr = start_addr as &u8;
    let middle_addr_ptr = middle_addr as &u8;
    let end_addr = end_addr as &u8;

    // 判断中间地址是否在开始和结束地址之间（不包含结束地址）
    (start_addr <= middle_addr) && (middle_addr < end_addr)
}

// 定义判断中间地址是否在开始和结束地址之间（不包含结束地址）的函数
fn OS_MEM_MIDDLE_ADDR(start_addr: &OsMemPoolHead, middle_addr: &c_void, end_addr: u32) -> bool {
    let start_addr_ptr = start_addr as &u8;
    let middle_addr_ptr = middle_addr as &u8;
    let end_addr = end_addr as &u8;

    // 判断中间地址是否在开始和结束地址之间（包含结束地址）
    (start_addr <= middle_addr) && (middle_addr <= end_addr)
}

#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
#[inline]
fn OsMemAllocCheck(pool: &mut OsMemPoolHead, int_save: u32) -> u32;

#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
fn OS_MEM_SET_MAGIC(node: &mut OsMemNodeHead) {
    (*node).magic = OS_MEM_NODE_MAGIC;
}

#[cfg(not(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1))]
fn OS_MEM_SET_MAGIC(node: &mut OsMemNodeHead) {}

#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
fn OS_MEM_MAGIC_VALID(node: &OsMemNodeHead) -> bool {
    (*node).magic == OS_MEM_NODE_MAGIC
}

#[cfg(not(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1))]
fn OS_MEM_MAGIC_VALID(node: &OsMemPoolHead) -> bool {
    true
}

// 如果支持多内存区域配置，则定义与间隙节点相关的宏
#[cfg(LOSCFG_MEM_MUL_REGIONS == 1)]
const OS_MEM_GAP_NODE_MAGIC: usize = 0xDCBAABCD;

#[cfg(LOSCFG_MEM_MUL_REGIONS == 1)]
fn OS_MEM_MARK_GAP_NODE(node: &mut OsMemNodeHead) {
    (*node).ptr.prev = OS_MEM_GAP_NODE_MAGIC as &mut OsMemNodeHead;
}

#[cfg(LOSCFG_MEM_MUL_REGIONS == 1)]
fn OS_MEM_IS_GAP_NODE(node: &OsMemNodeHead) -> bool {
    (*node).ptr.prev == OS_MEM_GAP_NODE_MAGIC as &OsMemNodeHead
}

#[cfg(not(LOSCFG_MEM_MUL_REGIONS == 1))]
fn OS_MEM_MARK_GAP_NODE(node: &mut OsMemNodeHead) {
    // 当不支持多内存区域配置时，标记间隙节点的函数为空操作
}

#[cfg(not(LOSCFG_MEM_MUL_REGIONS == 1))]
fn OS_MEM_IS_GAP_NODE(node: &OsMemNodeHead) -> bool {
    false // 当不支持多内存区域配置时，间隙节点判断函数始终返回 false
}

// 添加空闲内存节点到内存的内联函数
#[inline]
fn OsMemFreeNodeAdd(pool: &mut c_void, node: &mut OsMemFreeNodeHead);

// 从内存池释放内存节点的内联函数
#[inline]
fn OsMemFree(pool: &mut OsMemPoolHead, node: &mut OsMemNodeHead) -> u32 ;

// 打印内存池信息的函数
fn OsMemInfoPrint(pool: &mut c_void);

#[cfg(any(LOSCFG_MEM_FREE_BY_TASKID == 1, LOSCFG_TASK_MEM_USED == 1))]
#[inline]
fn OsMemNodeSetTaskID(node: &mut OsMemUsedNodeHead) {
    (*node).header.taskID = LOS_CurTaskIDGet();
}

type HandleFn = fn(cur_node: &mut OsMemNodeHead, arg: &mut c_void); //函数指针类型

#[inline]
pub fn OsAllMemNodeDohandle(pool: &mut c_void, handle: HandleFn, arg:*const c_void){ 
    let poolInfo = pool as &mut OsMemPoolHead;
    let mut tmpNode: &mut OsMemNodeHead = std::ptr::null_mut();
    let mut endNode: &mut OsMemNodeHead = std::ptr::null_mut();
    let intsave: u32 = 0;
    if(pool.isnull()){
        PRINTK("input param is NULL\n"); //los_debug的宏
        return;
    }
    if (LOS_MemIntegrityCheck(pool)) {
        PRINTK("LOS_MemIntegrityCheck error\n");
        return;
    }
    MEM_LOCK(poolInfo, intSave);
    endNode = OS_MEM_END_NODE(pool, (*poolInfo).info.totalSize);
    tmpNode = OS_MEM_FIRST_NODE(pool);
    while(tmpNode <= endNode){
        if (tmpNode == endNode) {
#[cfg(feature = "OS_MEM_EXPAND_ENABLE")]
{   
            if (OsMemIsLastSentinelNode(endNode) == false) {
                let size: u32 = OS_MEM_NODE_GET_SIZE((*endNode)->sizeAndFlag);
                tmpNode = OsMemSentinelNodeGet(endNode) ;
                endNode = OS_MEM_END_NODE(tmpNode, size);
                continue;
            }
}
            break;
        }
        handle(tmpNode, arg);
        tmpNode = OS_MEM_NEXT_NODE(tmpNode);
    }
    MEM_UNLOCK(poolInfo, intSave);
}

#[cfg(LOSCFG_TASK_MEM_USED == 1)]{
    pub fn GetTaskMemUsedHandle(curNode: &mut OsMemNodeHead, arg: &mut c_void){
        let mut args = arg as &mut u32;
        let tskMemInfoBuf: &u32 = args; //双重转换
        let tskMemInfoCnt: u32 = *args.offset(1) as u32;
    #[cfg(not(feature = "LOSCFG_MEM_MUL_REGIONS"))]{ //暂时没有ifndef的对应语句
        if (OS_MEM_NODE_GET_USED_FLAG((*curNode).sizeAndFlag)) {
            if ((*curNode).taskID < tskMemInfoCnt) {
                tskMemInfoBuf[(*curNode).taskID] += OS_MEM_NODE_GET_SIZE((*curNode).sizeAndFlag);
            }
        }
    }
    #[cfg(feature = "LOSCFG_MEM_MUL_REGIONS")]{
        if (OS_MEM_NODE_GET_USED_FLAG((*curNode).sizeAndFlag) && !OS_MEM_IS_GAP_NODE(curNode)) {
            if ((*curNode).taskID < tskMemInfoCnt) {
                tskMemInfoBuf[(*curNode).taskID] += OS_MEM_NODE_GET_SIZE((*curNode).sizeAndFlag);
            }
        }
    }
        return;
    }

    fn OsTaskMemUsed(pool: &mut, tskMemInfoBuf: &u32, tskMemInfoCnt: u32){
        let mut args: [u32; 2] = [tskMemInfoBuf as usize as u32, tskMemInfoCnt];
        OsAllMemNodeDoHandle(pool, GetTaskMemUsedHandle, args as &mut c_void);
    }
}

#[cfg(LOSCFG_MEM_WATERLINE == 1)]
#[inline]
fn OsMemWaterUsedRecord(pool: &mut OsMemPoolHead, size: u32){
    (*pool).info.curUsedSize += size;
    if (*pool).info.curUsedSize > (*pool).info.waterLine {
        (*pool).info.waterLine = (*pool).info.curUsedSize;
    }
}
#[cfg(not(LOSCFG_MEM_WATERLINE == 1))]
#[inline]
fn OsMemWaterUsedRecord(pool:&mut OsMemPoolHead, size: u32){}


// 获取当前函数名称的宏
macro_rules! function_name {
    () => {{
        fn f() {}
        let name = std::any::type_name::<fn()>();
        &name[6..name.len() - 4]
    }};
}   //TOBECHECK

#[cfg(not(OS_MEM_EXPAND_ENABLE == 0))]
{
    #[inline]
    fn OsMemLastSentinelNodeGet(sentinelNode: &OsMemNodeHead) -> Option<&OsMemNodeHead> {
        let mut node: Option<&OsMemNodeHead> = None;
        let mut ptr: &mut c_void = (*sentinelNode).ptr.next as &mut c_void;    //ptr.next 是 OsMemNodeHead类型的指针，但是下面的函数传入参数类型都是void *,这里只能把它转换为void *
        let mut size: u32 = OS_MEM_NODE_GET_SIZE((*sentinelNode).sizeAndFlag);
        while (ptr != null_mut()) && (size != 0){
            node = OS_MEM_END_NODE(ptr, size);
            ptr = (*node).ptr.next;
            size = OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag);
        }
        return node;
    }

    #[inline]
    fn OsMemSentinelNodeCheck (sentinelNode: &OsMemNodeHead) -> bool {
        if !OS_MEM_NODE_GET_USED_FLAG((*sentinelNode).sizeAndFlag) {
            false;
        }
        if !OS_MEM_MAGIC_VALID(sentinelNode){    
            false;
        }
        true;
    }

    #[inline]
    fn OsMemIsLastSentinelNode(sentinelNode: &OsMemNodeHead) -> bool {
        if !OsMemSentinelNodeCheck(sentinelNode) {
            println!(
                "{} {}, The current sentinel node is invalid",
                function_name!(),
                line!()
            );
            true;
        }
    
        if OS_MEM_NODE_GET_SIZE((*sentinelNode).sizeAndFlag) == 0 || (*sentinelNode).ptr.next.is_null() {
             true;
        }
    
        false
    }

    #[inline]
    fn OsMemSentinelNodeSet(sentinelNode: &mut OsMemNodeHead, new_node: &mut c_void, size: u32) {
        if !(*sentinelNode).ptr.next.is_null() {
            sentinelNode = OsMemLastSentinelNodeGet(sentinelNode);
        }

        (*sentinelNode).sizeAndFlag = size;
        (*sentinelNode).ptr.next = new_node as &mut OsMemNodeHead;
        OS_MEM_NODE_SET_USED_FLAG(&mut (*sentinelNode).sizeAndFlag);
        OS_MEM_NODE_SET_LAST_FLAG(&mut (*sentinelNode).sizeAndFlag);
    }

    #[inline]
    fn OsMemSentinelNodeGet(node: &OsMemNodeHead) -> Option<&c_void> {
        if !OsMemSentinelNodeCheck(node) {
            return None;
        }
        (*node).ptr.next as &c_void    //next类型为&OsMemNodeHead，需转换
    }

    #[inline]
    fn PreSentinelNodeGet(pool: &u8, node: &OsMemNodeHead) -> Option<&OsMemNodeHead> {
        let mut next_node: &mut OsMemNodeHead = null_mut();
        let mut sentinel_node: &mut OsMemNodeHead = null_mut();

        sentinel_node = OS_MEM_END_NODE(pool, unsafe { (*(pool as &OsMemPoolHead)).info.totalSize });
        while !sentinel_node.is_null() {
            if OsMemIsLastSentinelNode(sentinel_node) {
                println!("PreSentinelNodeGet can not find node 0x{:x}", node as usize);
                return None;
            }
            next_node = OsMemSentinelNodeGet(sentinel_node) as &OsMemNodeHead;
            if next_node == node {
                return sentinel_node;
            }
            let next_size = OS_MEM_NODE_GET_SIZE((*sentinel_node).sizeAndFlag);
            sentinel_node = OS_MEM_END_NODE(next_node as &c_void, next_size);
        }

        None
    }

    #[inline]
    fn TryShrinkPool(pool: &c_void, node: &OsMemNodeHead) -> bool {
        let total_size = ((*node).ptr.prev as usize) - (node as usize);
        let node_size = OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag);

        if node_size != total_size {
            return false;
        }

        let pre_sentinel = PreSentinelNodeGet(pool, node);
        if pre_sentinel.is_null() {
            return false;
        }

        let my_sentinel = (*node).ptr.prev;
        if OsMemIsLastSentinelNode(my_sentinel) {
            (*pre_sentinel).ptr.next = null_mut() as &OsMemNodeHead;
            OsMemSentinelNodeSet(pre_sentinel, null_mut() as &c_void, 0);
        } else {
            unsafe {
                (*pre_sentinel).sizeAndFlag = (*my_sentinel).sizeAndFlag;
                (*pre_sentinel).ptr.next = (*my_sentinel).ptr.next;
            }
        }

        if OsMemLargeNodeFree(node as &mut OsMemNodeHead) != LOS_OK {   // OsMemLargeNodeFree 函数找不到
            println!("TryShrinkPool free {:?} failed!", node);
            return false;
        }

        true
    }

    // const PAGE_SIZE: u32 = 0x1000;
    /*
        It_los_lms.h
        #define PAGE_SIZE                        (0x1000U)
    */

    #[inline]
    fn OsMemPoolExpand(pool: &mut c_void, size: usize, intSave: u32) -> i32 {
        let mut try_count = MAX_SHRINK_PAGECACHE_TRY;                   //Max_SHRINK_PAGECACHE_TRY 找不到
        let mut pool_info = pool as &mut OsMemPoolHead;
        let mut newNode = null_mut() as &mut OsMemNodeHead;
        let mut endNode = OS_MEM_END_NODE(pool, pool_info.info.total_size);

        let size1 = (size + OS_MEM_NODE_HEAD_SIZE()).round_up_to(PAGE_SIZE);

        loop {
            new_node = LOS_PhysPagesAllocContiguous(size1 >> PAGE_SHIFT) as &mut OsMemNodeHead;    // PAGE_SHIFT 和 LOS_PhysPagesAllocContiguous 找不到， 返回类型不知道
            if newNode.is_null() {
                if try_count > 0 {
                    try_count -= 1;
                    MEM_UNLOCK(pool_info, int_save);
                    OsTryShrinkMemory(size1 >> PAGE_SHIFT);
                    MEM_LOCK(pool_info, intSave);
                    continue;
                }

                println!("OsMemPoolExpand alloc failed size = {}", size);
                return -1;
            }
        }

        (*newNode).sizeAndFlag = size1 - OS_MEM_NODE_HEAD_SIZE;
        (*newNode).ptr.prev = OS_MEM_END_NODE(newNode as &mut c_void, size1);
        OsMemSentinelNodeSet(endNode, newNode as &c_void, size1);
        OsMemFreeNodeAdd(pool, newNode as *mut c_void);

        endNode = OS_MEM_END_NODE(newNode as &mut c_void, size1);
        std::ptr::write_bytes(endNode as &mut u8, 0, std::mem::size_of::<OsMemNodeHead>()); 
        (*endNode).ptr.next = ptr::null_mut() as &OsMemNodeHead;
        OS_MEM_SET_MAGIC(endNode);
        OsMemSentinelNodeSet(endNode, ptr::null_mut() as &c_void, 0);
        OsMemWaterUsedRecord(pool_info, OS_MEM_NODE_HEAD_SIZE);
        0
    }  

    fn los_mem_expand_enable(pool: &mut c_void) {
        if pool.is_null() {
            return;
        }

        (*(pool as &mut OsMemPoolHead)).info.attr |= OS_MEM_POOL_EXPAND_ENABLE;
    }
}

/*
    los_lms_pri.h
    typedef struct {
        UINT32 (*init)(const c_void *pool, UINT32 size);
        c_void (*deInit)(const c_void *pool);
        c_void (*mallocMark)(const c_void *curNodeStart, const c_void *nextNodeStart, UINT32 nodeHeadSize);
        c_void (*freeMark)(const c_void *curNodeStart, const c_void *nextNodeStart, UINT32 nodeHeadSize);
        c_void (*simpleMark)(UINTPTR startAddr, UINTPTR endAddr, UINT32 value);
        c_void (*check)(UINTPTR checkAddr, BOOL isFreeCheck);
    } LmsHook;
    extern LmsHook* g_lms;

    #define LMS_SHADOW_AFTERFREE_U8        0xFF
*/

//可能有问题
#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
    #[inline]
    fn OsLmsFirstNodeMark(pool: &mut c_void, node: &mut OsMemNodeHead) {
        if g_lms.is_null()
        {
            return;
        }

        (*g_lms).simpleMark(pool as usize, node as usize, LMS_SHADOW_PAINT_U8);
        (*g_lms).simpleMark(node as usize, node as usize + OS_MEM_NODE_HEAD_SIZE, LMS_SHADOW_REDZONE_U8);
        (*g_lms).simpleMark(
            OS_MEM_NEXT_NODE(node) as usize,
            OS_MEM_NEXT_NODE(node) as usize + OS_MEM_NODE_HEAD_SIZE,
            LMS_SHADOW_REDZONE_U8,
        );
        (*g_lms).simpleMark(
            node as usize + OS_MEM_NODE_HEAD_SIZE,
            OS_MEM_NEXT_NODE(node) as usize,
            LMS_SHADOW_AFTERFREE_U8,
        );
    }

    #[inline]
    fn OsLmsAllocAlignMark(ptr: &mut c_void, aligned_ptr: &mut c_void, size: u32) {
        if g_lms.is_null() || ptr.is_null() {
            return;
        }

        let alloc_node = (ptr as &mut OsMemUsedNodeHead).offset(-1) as &mut OsMemNodeHead;

        if ptr != aligned_ptr {
            (*g_lms).simpleMark(ptr as usize, ptr as usize + std::mem::size_of::<u32>(), LMS_SHADOW_PAINT_U8);
            (*g_lms).simpleMark(ptr as usize + std::mem::size_of::<u32>(), aligned_ptr as usize, LMS_SHADOW_REDZONE_U8);
        }

        (*g_lms).simpleMark(LMS_ADDR_ALIGN(aligned_ptr as usize + size as usize), OS_MEM_NEXT_NODE(alloc_node) as usize, LMS_SHADOW_REDZONE_U8);
        
    }

    #[inline]
    fn OsLmsReallocMergeNodeMark(node: &OsMemNodeHead)
    {
        if g_lms.is_null()
        {
            return ;
        }
        (*g_lms).simpleMark(node as usize + OS_MEM_NODE_HEAD_SIZE, OS_MEM_NEXT_NODE(node) as usize, LMS_SHADOW_ACCESSIBLE_U8);
    }

    #[inline]
    fn OsLmsReallocSplitNodeMark(node: &OsMemNodeHead)
    {
        if g_lms.is_null()
        {
            return ;
        }
        (*g_lms).simpleMark(OS_MEM_NEXT_NODE(node) as usize, OS_MEM_NEXT_NODE(node) as usize + OS_MEM_NODE_HEAD_SIZE, LMS_SHADOW_REDZONE_U8);
        (*g_lms).simpleMark(OS_MEM_NEXT_NODE(node) as usize + OS_MEM_NODE_HEAD_SIZE, OS_MEM_NEXT_NODE(OS_MEM_NEXT_NODE(node)) as usize, LMS_SHADOW_AFTERFREE_U8);
    }

    #[inline]
    fn OsLmsReallocResizeMark(node: &OsMemNodeHead, resize: u32)
    {
        if g_lms.is_null()
        {
            return ;
        }
        (*g_lms).simpleMark(node as usize + resize, OS_MEM_NEXT_NODE(node) as usize, LMS_SHADOW_REDZONE_U8);
    }
}

#[cfg(LOSCFG_MEM_LEAKCHECK == 1)]   // LOSCFG_MEM_LEAKCHECK 未找到
mod mem_leakcheck
{
    struct OsMemLeakCheckInfo {
        node: &mut OsMemNodeHead,        
        linkReg: [usize; LOSCFG_MEM_RECORD_LR_CNT],  
    }

    static mut g_leakCheckRecord: [OsMemLeakCheckInfo; LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM] = [OsMemLeakCheckInfo {
        node: std::ptr::null_mut(),                    // 初始化 node 为 null 指针
        linkReg: [0; LOSCFG_MEM_RECORD_LR_CNT],        // 初始化 link_reg 数组为全 0
    }; LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM];

    let mut g_leakCheckRecordCnt: u32 = 0;

    #[inline]
    fn OsMemLeakCheckInfoRecord(node: &mut OsMemNodeHead) {
        let info = &mut g_leakCheckRecord[g_leakCheckRecordCnt as usize];

        if !OS_MEM_NODE_GET_LEAK_FLAG((*node).sizeAndFlag) {
            (*info).node = node;
            (*info).linkReg.copy_from_slice(&(*node).linkReg);
            OS_MEM_NODE_SET_LEAK_FLAG(&mut (*node).sizeAndFlag);
            g_leakCheckRecordCnt += 1;
            if g_leakCheckRecordCnt >= LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM as u32 {
                g_leakCheckRecordCnt = 0;
            }
        }
    }

    #[inline]
    fn OsMemLeakCheckInit() {
        let size = mem::size_of::<OsMemLeakCheckInfo>() * LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM;
        let ptr = g_leakCheckRecord.as_mut_ptr() as &mut u8;
        std::ptr::write_bytes(ptr, 0, size);
        g_leakCheckRecordCnt = 0;
    }

    #[inline]
    fn OsMemLinkRegisterRecord(node: &mut OsMemNodeHead) {
        let size = mem::size_of::<[usize; LOSCFG_MEM_RECORD_LR_CNT]>();
        let ptr = node.linkReg.as_mut_ptr() as &mut u8;
        std::ptr::write_bytes(ptr, 0, size);
        OsBackTraceHookCall(node.linkReg.as_mut_ptr(), LOSCFG_MEM_RECORD_LR_CNT, LOSCFG_MEM_OMIT_LR_CNT, 0);
        //c_void OsBackTraceHookCall(UINTPTR *LR, UINT32 LRSize, UINT32 jumpCount, UINTPTR SP)
    }
    #[inline]
    fn OsMemUsedNodePrint(node: &mut OsMemNodeHead){
        let mut count: u32;
        if (OS_MEM_NODE_GET_USED_FLAG((*node).sizeAndFlag) && !OS_MEM_IS_GAP_NODE(node)) {
            println!("0x{:x}: 0x{:x} ", node as usize, OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag));
            for (count = 0; count < LOSCFG_MEM_RECORD_LR_CNT; count++) {
                println!(" 0x{:x} ", (*node).linkReg[count]);
            }
            println!();
            OsMemLeakCheckInfoRecord(node);
        }
    }

    #[inline]
    fn OsMemUsedNodePrintHandle(node: &mut OsMemNodeHead, arg: &c_void){
        UNUSED(arg);
        OsMemUsedNodePrint(node);
        return;
    }
    
    fn LOS_MemUsedNodeShow(pool: &mut OsMemPoolHead){
        let mut count: u32;
        println!("\n\rnode          size    ");
        for count in 0..LOSCFG_MEM_RECORD_LR_CNT{
            println!("    LR{:u}   ", count);
        }
        OsMemLeakCheckInit();
        OsAllMemNodeDoHandle(pool, OsMemUsedNodePrintHandle, null_mut());
        return;
    }
    
    #[cfg(LOSCFG_KERNEL_PRINTF != 0)]
    fn OsMemNodeBacktraceInfo(tmpNode: &mut OsMemNodeHead, preNode: &mut OsMemNodeHead){
        println!("\n broken node head LR info: \n");
        for i in 0..LOSCFG_MEM_RECORD_LR_CNT{
            println!(" LR[{:d}]:0x{:x}\n", i, (*tmpNode).linkReg[i]);
        }
        println!("\n pre node head LR info: \n");
        for i in 0..LOSCFG_MEM_RECORD_LR_CNT{
            println!(" LR[{:d}]:0x{:x}\n", i, (*preNode).linkReg[i]);
        }
    }
}

#[inline]
fn OsMemFreeListIndexGet(size: u32) -> u32{
    let fl: u32 = OsMemFlGet(size);
    if (fl < OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/) {
        return fl;
    }
    let sl: u32 = OsMemSlGet(size, fl);
    return (OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/ + ((fl - OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/) << OS_MEM_SLI/*在los_memory_h.rs里定义*/) + sl);
}

#[inline]
pub fn OsMemFindCurSuitableBlock(poolHead: &mut OsMemPoolHead, index: u32, size: u32) -> Option<&OsMemFreeNodeHead> {
    let mut node: &OsMemPoolHead = (*poolHead).freeList[index];
    while(!node.isnull()){
        if ((*node).header.sizeAndFlag >= size) {
            return (node as &OsMemFreeNodeHead);
        }
        node = node->next;
    }
    return std::ptr::null_mut() as &OsMemFreeNodeHead;
}

#[inline]
pub fn OsMemNotEmptyIndexGet(poolHead: &mut OsMemPoolHead, index: u32) -> u32 {
    let mask: u32 = (*poolHead).freeListBitmap[index >> 5];
    mask &= ~((1 << (index & OS_MEM_BITMAP_MASK)) - 1);
    if (mask != 0) {
        index = OsMemFFS(mask) + (index & ~OS_MEM_BITMAP_MASK);
        return index;
    }

    return OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */;
}

#[inline]
fn OsMemFindNextSuitableBlock(pool: &mut c_void, size: u32, outIndex: &mut u32) -> Option<&OsMemFreeNodeHead> {
    let poolHead: &OsMemPoolHead  = pool as &mut OsMemPoolHead;
    let mut fl: u32 = OsMemFlGet(size);
    let mut index: u32 = 0;
    let mut curIndex = OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */;
    do{
        if (fl < OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/) {
            index = fl;
        } 
        else {
            let sl = OsMemSlGet(size, fl);
            curIndex = ((fl - OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/) << OS_MEM_SLI/*在los_memory_h.rs里定义*/) + sl + OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/;
            index = curIndex + 1;
        }

        let tmp = OsMemNotEmptyIndexGet(poolHead, index);
        if (tmp != OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */) {
            index = tmp;
            *outIndex = index;//change GOTO
            return poolHead->freeList[index] as *const c_void;
        }

        for (index = LOS_Align(index + 1, 32); index < OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */; index += 32) {
            /* 5: Divide by 32 to calculate the index of the bitmap array. */
            let mask = poolHead->freeListBitmap[index >> 5];
            if (mask != 0) {
                index = OsMemFFS(mask) + index;
                *outIndex = index;
                return poolHead->freeList[index] as &OsMemFreeNodeHead;
            }
        }
    } while(0);
    if (curIndex == OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */) {
        return ptr::null_mut() as &OsMemFreeNodeHead;
    }
    *outIndex = curIndex;
    return OsMemFindCurSuitableBlock(poolHead, curIndex, size);
}

#[inline]
fn OsMemSetFreeListBit(head: &mut OsMemPoolHead, index: u32){
    (*head).freeListBitmap[index >> 5] |= 1U << (index & 0x1f);
}

#[inline]
fn OsMemClearFreeListBit(head: &mut OsMemPoolHead, index: u32){
    (*head).freeListBitmap[index >> 5] &= ~(1U << (index & 0x1f));
}

#[inline]
fn OsMemListAdd(pool: &mut OsMemPoolHead, listIndex: u32, node: &mut OsMemFreeNodeHead){
    let mut firstNode: &OsMemPoolHead = (*pool).freeList[listIndex];
    if (!firstNode.isnull()) { //引用不方便检测isnull
        (*firstNode).prev = node;
    }
    (*node).prev = ptr::null_mut() as &OsMemFreeNodeHead;
    (*node).next = firstNode;
    (*pool).freeList[listIndex] = node;
    OsMemSetFreeListBit(pool, listIndex);
    OS_MEM_SET_MAGIC(&mut ((*node).header));
}

#[inline]
fn OsMemListDelete(pool: &mut OsMemPoolHead, listIndex: u32, node: &mut OsMemFreeNodeHead)
{
    if (node == pool->freeList[listIndex]) {
        (*pool).freeList[listIndex] = (*node).next;
        if ((*node).next.isnull()) { 
            OsMemClearFreeListBit(pool, listIndex);
        } else {
            (*node).next.prev = ptr::null_mut() as &mut OsMemFreeNodeHead; 
        }
    } else {
        (*node).next.prev = (*node).next;
        if (!node->next.isnull()) {
            (*node).next.prev = (*node).prev;
        }
    }
    OS_MEM_SET_MAGIC(&mut (*node).header);
}

//向内存池中的空闲节点链表中添加新的空闲节点
#[inline]
fn OsMemFreeNodeAdd(pool: &mut c_void, node: &mut OsMemFreeNodeHead){
    let mut index: u32 = OsMemFreeListIndexGet((*node).header.sizeAndFlag);
    if(index>=OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */){
        LOS_Panic("The index of free lists is error, index = {:u}\n", index);
    }
    OsMemListAdd(pool as &mut OsMemPoolHead, index, node);
}

//从内存池中的空闲节点链表中删除指定的空闲节点
#[inline]
fn OsMemFreeNodeDelete(pool:&mut c_void, mode: &mut OsMemFreeNodeHead){
    let mut index: u32 = OsMemFreeListIndexGet(node.header.sizeAndFlag);
    OsMemListDelete(&pool,index,&node);
}

#[inline]
fn OsMemFreeNodeGet(pool: &mut c_void, size: u32)-> Option<&OsMemNodeHead>{
    let mut poolHead: &mut OsMemPoolHead = pool as &mut OsMemPoolHead;
    let mut index: u32 = 0;
    let first_node: &OsMemFreeNodeHead = OsMemFindNextSuitableBlock(pool, size, &mut index);
    if let Some(mut first_node) = first_node {
        OsMemListDelete(pool, index, first_node);
        Some(&mut first_node.header)
    } else {
        None
    }
}

#[inline]
fn OsMemMergeNode(node: &mut OsMemNodeHead){
    let mut nextNode: &mut OsMemNodeHead = null_mut();
    (*node).ptr.prev.sizeAndFlag += (*node).sizeAndFlag;
    let mut temp: u32 = (node as usize) + (*node).sizeAndFlag;
    nextNode = temp as &mut OsMemNodeHead;
    if !OS_MEM_NODE_GET_LAST_FLAG((*nextNode).sizeAndFlag) && !OS_MEM_IS_GAP_NODE(nextNode)
    {
        (*nextNode).ptr.prev = (*node).ptr.prev;
    }
}

#[inline]
fn OsMemSplitNode(pool: &mut c_void, allocNode: &mut OsMemNodeHead, allocSize: u32) {
    let mut newFreeNode: &mut OsMemFreeNodeHead = null_mut();
    let mut nextNode: &mut OsMemNodeHead = null_mut();
    newFreeNode = (((allocNode as &mut u8).offset(allocSize)) as &mut c_void) as &mut OsMemFreeNodeHead;
    (*newFreeNode).header.ptr.prev = allocNode;
    (*newFreeNode).header.sizeAndFlag = (*allocNode).sizeAndFlag - allocSize;
    (*allocNode).sizeAndFlag = allocSize;
    nextNode = OS_MEM_NEXT_NODE(&mut (*newFreeNode).header);
    if !OS_MEM_NODE_GET_LAST_FLAG((*nextNode).sizeAndFlag) && !OS_MEM_IS_GAP_NODE(nextNode)
    {
        (*nextNode).ptr.prev = &mut (*newFreeNode).header;
        if(!OS_MEM_NODE_GET_USED_FLAG((*nextNode).sizeAndFlag))
        {
            OsMemFreeNodeDelete(pool, nextNode as &mut OsMemFreeNodeHead);
            OsMemMergeNode(nextNode);
        }
    }
    OsMemFreeNodeAdd(pool, newFreeNode);
}

//创建一个被使用的内存节点
#[inline]
fn OsMemCreateUsedNode (addr: &mut c_void) -> Option<&c_void> {
    let mut node: &mut OsMemUsedNodeHead = addr as &mut OsMemUsedNodeHead;
#[cfg(any(LOSCFG_MEM_FREE_BY_TASKID == 1, LOSCFG_TASK_MEM_USED == 1))] 
{
    OsMemNodeSetTaskID(node);
}
#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
    let mut newNode: &mut OsMemNodeHead = node as &mut OsMemNodeHead;
    if !g_lms.is_null()
    {
        (*g_lms).mallocMark(&mut *newNode, OS_MEM_NEXT_NODE(&(*newNode)), OS_MEM_NODE_HEAD_SIZE);
    }
}
    return node.offset(1) as &mut c_void;
}

//初始化一个内存池
#[inline]
fn OsMemPoolInit(pool: &mut c_void, size: u32) -> u32{//pool是指针，不是引用
    let mut poolHead: &mut OsMemPoolHead = pool as &mut OsMemPoolHead;
    let mut newNode: &mut OsMemNodeHead = null_mut();
    let mut endNode: &mut OsMemNodeHead = null_mut();
    memset_s(&mut *poolHead, size, 0, std::mem::size_of::<OsMemPoolHead>());
#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
    let mut resize: u32 = 0;
    if !g_lms.is_null() {
        resize = (*g_lms).init(pool, size);
        size = if resize == 0 { size } else { resize };
    }
}
    (*poolHead).info.pool = pool;
    (*poolHead).info.totalSize = size;
    (*poolHead).info.attr &= ~(OS_MEM_POOL_UNLOCK_ENABLE | OS_MEM_POOL_EXPAND_ENABLE);

    newNode = OS_MEM_FIRST_NODE(pool);
    (*newNode).sizeAndFlag = (size - std::mem::size_of::<OsMemPoolHead>() - OS_MEM_NODE_HEAD_SIZE);
    (*newNode).ptr.prev = OS_MEM_END_NODE(pool, size);
    OS_MEM_SET_MAGIC(newNode);
    OsMemFreeNodeAdd(pool, &mut newNode as &mut OsMemFreeNodeHead);

    endNode = OS_MEM_END_NODE(pool, size) as &mut OsMemPoolHead;    //返回值为*const OsMemNodeHead
    OS_MEM_SET_MAGIC(endNode);
#[cfg(OS_MEM_EXPAND_ENABLE == 1)]
{
    (*endNode).ptr.next = null_mut();
    OsMemSentinelNodeSet(endNode, null_mut(), 0);
    //fn os_mem_sentinel_node_set(sentinel_node: &mut OsMemNodeHead, new_node: Option<Box<OsMemNodeHead>>, size: usize) 
}
#[cfg(OS_MEM_EXPAND_ENABLE==0)]
{
    (*endNode).sizeAndFlag = 0;
    (*endNode).ptr.prev = newNode;
    OS_MEM_NODE_SET_USED_FLAG((*endNode).sizeAndFlag);
}
#[cfg(LOSCFG_MEM_WATERLINE == 1)]
{
    (*poolHead).info.curUsedSize = std::mem::size_of::<OsMemPoolHead>()+ OS_MEM_NODE_HEAD_SIZE;
    (*poolHead).info.waterLine = (*poolHead).info.curUsedSize;
}

#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
    if resize != 0 {
        OsLmsFirstNodeMark(pool, newNode);
    }
}
    LOS_OK;
}


#[cfg(LOSCFG_MEM_MUL_POOL == 1)]
{
    fn OsMemPoolDeInit(pool: &mut c_void, size: u32){
    #[cfg(feature = "LOSCFG_KERNEL_LMS")]
    {
        if !g_lms.is_null() {
            (*g_lms).deInit(pool);
        }
    }
        memset_s(&mut *pool, size, 0,std::mem::size_of::<OsMemPoolHead>());
    }

    //向内存池链表中添加新的内存池
    fn OsMemPoolAdd(pool: &mut c_void, size: u32) -> u32 {
        let mut nextPool: &mut c_void = g_poolHead;
        let mut curPool: &mut c_void = g_poolHead;
        let mut poolEnd: usize = 0;
        while nextPool != null_mut(){
            poolEnd= (nextPool as usize) + LOS_MemPoolSizeGet(nextPool);
            if ((pool <= nextPool) && (((pool as usize) + size) > (nextPool as usize))) ||
            (((pool as usize) < poolEnd) && (((pool as usize) + size) >= poolEnd))
            {
                println!("pool [0x{:x}, 0x{:x}) conflict with pool [0x{:x}, 0x{:x})\n", pool as usize,
                        (pool as usize) + size, (nextPool as usize), (nextPool as usize) + LOS_MemPoolSizeGet(nextPool));
                return LOS_NOK;
            }
            curPool = nextPool;
            nextPool = (*(nextPool as &mut OsMemPoolHead)).nextPool;
        }

        if g_poolHead.is_null() {
            g_poolHead = pool;
        }else {
            (*(curPool as &mut OsMemPoolHead)).nextPool = pool;
        }
        (*(pool as &mut OsMemPoolHead)).nextPool = null_mut();
        return LOS_OK;
    }

    fn OsMemPoolDelete(pool: &mut c_void) -> u32 {
        let mut ret: u32 = LOS_NOK;
        let mut next_pool: &mut c_void = null_mut();
        let mut cur_pool: &mut c_void = null_mut();
        loop {
            if pool == &mut g_poolHead as &const c_void {
                g_poolHead = (*(g_poolHead as &OsMemPoolHead)).nextPool as &c_void;
                ret = LOS_OK;
                break;
            }

            cur_pool = g_poolHead;
            next_pool = g_poolHead;

            while !next_pool.is_null(){
                if pool == next_pool { 
                    if let Some(pool_head) = g_poolHead as_mut()
                    {
                        (*(cur_pool as &mut OsMemPoolHead)).nextPool = (*(pool_head as &const OsMemPoolHead)).nextPool; 
                        ret = LOS_OK;
                        break;
                    }
                }
                cur_pool = next_pool;
                next_pool = (*(next_pool as &mut OsMemPoolHead)).nextPool;
            }
            break;
        }
        ret
    }
}

pub fn LOS_MemInit(pool: &mut c_void, size: u32) -> u32 {

    if pool.is_null() || size <= OS_MEM_MIN_POOL_SIZE {
        return LOS_NOK;
    }

    if (pool as usize) & (OS_MEM_ALIGN_SIZE - 1) != 0 || size & (OS_MEM_ALIGN_SIZE - 1) != 0 {
        println!("LiteOS heap memory address or size configured not aligned: address: 0x{:x}, size: 0x{:x}, alignsize: {}", pool as usize, size, OS_MEM_ALIGN_SIZE);
        return LOS_NOK;
    }

    if OsMemPoolInit(pool, size) != 0 {
        return LOS_NOK;
    }

#[cfg(LOSCFG_MEM_MUL_POOL == 1)]
{
    if OsMemPoolAdd(pool, size) != 0 {
        OsMemPoolDeInit(pool, size);
        return LOS_NOK;
    }
}
    OsHookCall(LOS_HOOK_TYPE_MEM_INIT, pool, size);
    //找不到LOS_HOOK_TYPE_MEM_INIT TO BE DONE
    //hook_call函数找不到在哪
    LOS_OK
}

#[cfg(LOSCFG_MEM_MUL_POOL == 1)]
{
    fn LOS_MemDeInit(pool: &mut c_void) -> u32 {
        let tmpPool: &OsMemPoolHead = pool as &OsMemPoolHead;
        if tmpPool.is_null() {
            return LOS_NOK;
        }

        let tmp_pool_info = &pool.info;

        if (*tmpPool).info.pool != pool || (*tmpPool).info.totalSize <= OS_MEM_MIN_POOL_SIZE {
            return LOS_NOK;
        }

        if OsMemPoolDelete(tmpPool) != 0 {
            return LOS_NOK;
        }

        OsMemPoolDeInit(pool, (*tmpPool).info.totalSize);

        OsHookCall(LOS_HOOK_TYPE_MEM_DEINIT, tmpPool);  // LOS_HOOK_TYPE_MEM_DEINIT 未找到

        LOS_OK
    }

    fn LOS_MemPoolList() -> u32 {
        let mut next_pool = g_poolHead;
        let mut index = 0;

        while !next_pool.is_null() {
            println!("pool{:u} :", index);
            index += 1;
            OsMemInfoPrint(next_pool);
            next_pool = (*(next_pool as &mut OsMemPoolHead)).next_pool ;
        }
        index
    }
}

#[inline]
fn OsMemAlloc(pool: &mut OsMemPoolHead, size: u32, int_save: u32) -> Option<&mut c_void> {
    let mut alloc_node: &mut OsMemNodeHead = null_mut();
    #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
    if OsMemAllocCheck(pool, int_save) == LOS_NOK {
        return null_mut();
    }
    let allocSize = OS_MEM_ALIGN(size + OS_MEM_NODE_HEAD_SIZE, OS_MEM_ALIGN_SIZE);
    loop {
        let alloc_node = OsMemFreeNodeGet(pool, alloc_size);
        if alloc_node.is_null() {
            #[cfg(not(OS_MEM_EXPAND_ENABLE == 0))]
            {
                if (*pool).info.attr & OS_MEM_POOL_EXPAND_ENABLE != 0 {
                    let ret = OsMemPoolExpand(pool, alloc_size, int_save);
                    if ret == 0 {
                        continue;
                    }
                }
            }
            #[cfg(LOSCFG_KERNEL_LMK == 1)]
            {
                let kill_ret = LOS_LmkTasksKill();
                if kill_ret == LOS_OK {
                    continue;
                }
            }
            println!("---------------------------------------------------\
                      --------------------------------------------------------");
            MEM_UNLOCK(pool, int_save);
            OsMemInfoPrint(pool);
            MEM_LOCK(pool, int_save);
            println!("[{}] No suitable free block, require free node size: 0x{:x}",
                     "OsMemAlloc", alloc_size);
            println!("----------------------------------------------------\
                      -------------------------------------------------------");
            return null_mut();
        }  
    }
    
    if alloc_size + OS_MEM_MIN_LEFT_SIZE <= (*alloc_node).sizeAndFlag {
        OsMemSplitNode(pool, alloc_node, alloc_size);
    }

    OS_MEM_NODE_SET_USED_FLAG((*alloc_node).sizeAndFlag);
    OsMemWaterUsedRecord(pool, OS_MEM_NODE_GET_SIZE((*alloc_node).sizeAndFlag));

    #[cfg(LOSCFG_MEM_LEAKCHECK == 1)]
    OsMemLinkRegisterRecord(alloc_node);

    return OsMemCreateUsedNode(alloc_node as &mut c_void);
}

#[inline]
fn LOS_MemAlloc(pool: &mut c_void, size: u32) -> Option<&mut c_void> {
    if pool.is_null() || size == 0 {
        return null_mut();
    }

    let size1 = if size < OS_MEM_MIN_ALLOC_SIZE {
        OS_MEM_MIN_ALLOC_SIZE
    } else {
        size
    };

    let pool_head = pool as &mut OsMemPoolHead;
    let mut ptr: &mut c_void = null_mut();
    let mut int_save = 0;

    MEM_LOCK(pool_head, int_save);
    {
        if !(OS_MEM_NODE_GET_USED_FLAG(size1) || OS_MEM_NODE_GET_ALIGNED_FLAG(size1)) {
            ptr = OsMemAlloc(pool_head, size1, int_save);
        }
    }
    
    MEM_UNLOCK(pool_head, int_save);

    OsHookCall(LOS_HOOK_TYPE_MEM_ALLOC, pool, ptr, size1);

    ptr
}

fn LOS_MemAllocAlign(pool: *mut c_void, size: u32, boundary: u32) -> Option<&mut c_void> {
    let mut gap_size: u32 = 0;

    if pool.is_null() || size == 0 || boundary == 0 || !OS_MEM_IS_POW_TWO(boundary) ||
        !OS_MEM_IS_ALIGNED(boundary, std::mem::size_of::<*mut c_void>()) {
        return null_mut();
    }

    let mut adjusted_size = size;
    if adjusted_size < OS_MEM_MIN_ALLOC_SIZE {
        adjusted_size = OS_MEM_MIN_ALLOC_SIZE;
    }

    if boundary.checked_sub(std::mem::size_of::<u32>()).unwrap_or(0) > u32::MAX - adjusted_size {
        return null_mut();
    }
    //  std::mem::size_of::<u32>() 返回 u32 类型的大小（以字节为单位）。
    //  boundary.checked_sub(std::mem::size_of::<u32>()) 用于计算 boundary 减去 u32 类型的大小，如果结果超出了 u32 类型的范围，则返回 None。否则返回差值。
    //  unwrap_or(0) 如果结果是 Some(value)，则返回 value；如果结果是 None，则返回 0

    let use_size = (adjusted_size + boundary) - std::mem::size_of::<u32>();
    if OS_MEM_NODE_GET_USED_FLAG(use_size) || OS_MEM_NODE_GET_ALIGNED_FLAG(use_size) {
        return null_mut();
    }

    let pool_head = pool as &mut OsMemPoolHead;
    let mut int_save = 0;
    let mut ptr: &mut c_void = null_mut();
    let mut aligned_ptr: &mut c_void = null_mut();

    MEM_LOCK(pool_head, &mut int_save);
    loop {
        ptr = OsMemAlloc(pool_head, use_size, int_save);
        aligned_ptr = OS_MEM_ALIGN(ptr, boundary) as &mut c_void;
        if ptr == aligned_ptr {
        #[cfg(feature = "LOSCFG_KERNEL_LMS")]
        {
            OsLmsAllocAlignMark(ptr, aligned_ptr, size);
        }
            break;
        }

        gap_size = (aligned_ptr as u8 - ptr as u8) as u32;
        let alloc_node = (ptr as &mut OsMemUsedNodeHead).offset(-1);
        OS_MEM_NODE_SET_ALIGNED_FLAG((*alloc_node).header.sizeAndFlag);
        OS_MEM_SET_GAPSIZE_ALIGNED_FLAG(gap_size);

        *((aligned_ptr as &mut u32).offset(-1)) = gap_size;

        #[cfg(feature = "LOSCFG_KERNEL_LMS")]
        {
            OsLmsAllocAlignMark(ptr, aligned_ptr, size);
        }
        ptr = aligned_ptr;
        break;
    }
    MEM_UNLOCK(pool_head, int_save);
    OsHookCall(LOS_HOOK_TYPE_MEM_ALLOCALIGN, pool, ptr, size, boundary);
    ptr
}

#[inline]
fn OsMemAddrValidCheck(pool: &OsMemPoolHead, addr: &c_void) -> bool {
    let mut size = (*pool).info.totalSize;
    if OS_MEM_MIDDLE_ADDR_OPEN_END(pool.offset(1), addr, (pool as usize + size) as u32) {
        return true;
    }
#[cfg(not(OS_MEM_EXPAND_ENABLE == 0))]
{
        let mut node: &mut OsMemNodeHead = null_mut();
        let mut sentinel: &mut OsMemNodeHead = OS_MEM_END_NODE(pool, size);
        while !OsMemIsLastSentinelNode(sentinel) {
            size = OS_MEM_NODE_GET_SIZE((*sentinel).sizeAndFlag) ;
            node = OsMemSentinelNodeGet(sentinel);
            sentinel = OS_MEM_END_NODE(node, size);
            if OS_MEM_MIDDLE_ADDR_OPEN_END(node, addr as usize, (node as usize + size) as usize) {
                return true;
            }
        }
}
    false
}

#[inline]
fn OsMemIsNodeValid(node: &OsMemNodeHead, start_node: &OsMemNodeHead, 
                    end_node: &OsMemNodeHead, pool_info: &OsMemPoolHead) -> bool {
    if !OS_MEM_MIDDLE_ADDR(start_node, node, end_node) {
        return false;
    }

    if OS_MEM_NODE_GET_USED_FLAG((*node).sizeAndFlag) {
        if !OS_MEM_MAGIC_VALID(node) {
            return false;
        }
        return true;
    }

    if !OsMemAddrValidCheck(pool_info, (*node).ptr.prev) {
        return false;
    }

    true
}

#[inline]
fn OsMemCheckUsedNode(pool: &OsMemPoolHead, node: &OsMemNodeHead) -> u32 {
    let mut start_node = OS_MEM_FIRST_NODE(pool) as &OsMemNodeHead;
    let mut end_node = OS_MEM_END_NODE(pool, (*pool).info.totalSize) as &OsMemNodeHead;
    let mut next_node: &OsMemNodeHead = null_mut();
    let mut done_flag = false;

    loop {
        loop {
            if OS_MEM_IS_GAP_NODE(node) {
                break;
            }

            if !OsMemIsNodeValid(node, start_node, end_node, pool) {
                break;
            }

            if !OS_MEM_NODE_GET_USED_FLAG((*node).sizeAndFlag) {
                break;
            }

            next_node = OS_MEM_NEXT_NODE(node);
            if !OsMemIsNodeValid(next_node, start_node, end_node, pool) {
                break;
            }

            if !OS_MEM_NODE_GET_LAST_FLAG((*next_node).sizeAndFlag) && !OS_MEM_IS_GAP_NODE(next_node){
                if (*next_node).ptr.prev != node {
                    break;
                }
            }

            if node != start_node &&
                (!OsMemIsNodeValid((*node).ptr.prev, start_node, end_node, pool) || OS_MEM_NEXT_NODE((*node).ptr.prev) != node) {
                break;
            }
            done_flag = true;
        }

        if !done_flag {
#[cfg(not(OS_MEM_EXPAND_ENABLE == 0))]
{
                if !OsMemIsLastSentinelNode(end_node) {
                    start_node = OsMemSentinelNodeGet(end_node);
                    end_node = OS_MEM_END_NODE(start_node, OS_MEM_NODE_GET_SIZE((*end_node).sizeAndFlag));
                    continue;
                }
}
            return LOS_NOK;
        }
        break;
    }

    LOS_OK
}

#[inline]
fn OsMemFree(pool: &mut OsMemPoolHead, node: &mut OsMemNodeHead) -> u32 {
    let ret = OsMemCheckUsedNode(pool, node);
    if ret != LOS_OK {
        println!("OsMemFree check error!\n");
        return ret;
    }

#[cfg(LOSCFG_MEM_WATERLINE == 1)]
{
    (*pool).info.curUsedSize -= OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag);
}

    (*node).sizeAndFlag = OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag);

#[cfg(LOSCFG_MEM_LEAKCHECK == 1)]
{
    OsMemLinkRegisterRecord(node);
}

#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
    let next_node_backup: &OsMemNodeHead = OS_MEM_NEXT_NODE(node);
    let cur_node_backup: &OsMemNodeHead = node;
    if !g_lms.is_null() {
        (*g_lms).check((node as usize + OS_MEM_NODE_HEAD_SIZE) as usize, true);
    }
}
    let pre_node: &OsMemNodeHead = (*node).ptr.prev ;
    if !pre_node.is_null() && !OS_MEM_NODE_GET_USED_FLAG((*pre_node).sizeAndFlag) {
        OsMemFreeNodeDelete(pool, pre_node as &mut OsMemFreeNodeHead);
        OsMemMergeNode(node);
        node = pre_node;
    }

    let next_node: &OsMemNodeHead = OS_MEM_NEXT_NODE(node);
    if !next_node.is_null() && !OS_MEM_NODE_GET_USED_FLAG((*next_node).sizeAndFlag) {
        OsMemFreeNodeDelete(pool, next_node as &mut OsMemFreeNodeHead);
        OsMemMergeNode(next_node);
    }

#[cfg(not(OS_MEM_EXPAND_ENABLE == 0))]
{
    if (*pool).info.attr & OS_MEM_POOL_EXPAND_ENABLE != 0 {
        let first_node: &OsMemNodeHead = OS_MEM_FIRST_NODE(pool) ;
        if ((*node).ptr.prev > node) && (node != first_node) {
            if TryShrinkPool(pool, node) {
                return LOS_OK;
            }
        }
    }
}
    OsMemFreeNodeAdd(pool, node as &mut OsMemFreeNodeHead);

#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
    if !g_lms.is_null() {
        unsafe {
            (*g_lms).freeMark(cur_node_backup, next_node_backup, OS_MEM_NODE_HEAD_SIZE);
        }
    }
}
    ret
}

#[inline]
fn OsGetRealPtr(pool: &c_void, ptr: &mut c_void) -> Option<&mut c_void> {
    let mut real_ptr = ptr;
    let gap_size = *(ptr.offset(-(mem::size_of::<u32>() as isize)) as &u32);
    if os_mem_gapsize_check(gap_size) {
        println!("[{}:{}]gapSize:0x{:x} error", function_name!(), line!(), gap_size);
        return null_mut();
    }
    if OS_MEM_GET_GAPSIZE_ALIGNED_FLAG(gap_size) != 0 {
        let gap_size_aligned = OS_MEM_GET_ALIGNED_GAPSIZE(gap_size);
        if gap_size_aligned & (OS_MEM_ALIGN_SIZE - 1) != 0 ||
            gap_size_aligned > (ptr as usize - OS_MEM_NODE_HEAD_SIZE - pool as usize) {
                println!("[{}:{}]gapSize:0x{:x} error", function_name!(), line!(), gap_size);
            return null_mut();
        }
        real_ptr = (ptr as usize - gap_size_aligned as usize) as &mut c_void;
    }
    real_ptr
}

pub fn LOS_MemFree(pool: &mut c_void, ptr: &mut c_void) -> u32 {
    if pool.isnull() || ptr.is_null() || !OS_MEM_IS_ALIGNED(pool, std::mem::size_of::<&mut c_void>()) || !OS_MEM_IS_ALIGNED(ptr, std::mem::size_of::<&mut c_void>()) {
        return LOS_NOK;
    }

    OsHookCall(LOS_HOOK_TYPE_MEM_FREE, pool, ptr);

    let mut ret = LOS_NOK;
    let pool_head= pool as &mut OsMemPoolHead;
    let mut node: &mut OsMemPoolHead = null_mut();
    let mut int_save: u32 = 0;

    MEM_LOCK(pool_head, &mut int_save);
    loop {
        let real_ptr = OsGetRealPtr(pool, ptr);
        if real_ptr.isnull() {
            break;
        }
        node = ((real_ptr as usize) - OS_MEM_NODE_HEAD_SIZE) as &mut OsMemNodeHead;
        ret = OsMemFree(pool_head, node);
        break;
    }
    MEM_UNLOCK(pool_head, int_save);

    ret
}

#[inline]
fn OsMemReAllocSmaller(pool: &mut c_void, alloc_size: u32, node: &mut OsMemNodeHead, node_size: u32) {
#[cfg(LOSCFG_MEM_WATERLINE == 1)]
{
    let mut pool_info = pool as &mut OsMemPoolHead;
}
    (*node).sizeAndFlag = node_size;
    if (alloc_size + OS_MEM_MIN_LEFT_SIZE) <= node_size {
        OsMemSplitNode(pool, node, alloc_size);
#[cfg(LOSCFG_MEM_WATERLINE == 1)]
{
        (*pool_info).info.curUsedSize -= node_size - alloc_size;
}
#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
        OsLmsReallocSplitNodeMark(node);
}
        
    } else {
#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
        OsLmsReallocResizeMark(node, alloc_size);
}
    }
    OS_MEM_NODE_SET_USED_FLAG((*node).sizeAndFlag);
#[cfg(LOSCFG_MEM_LEAKCHECK == 1)]
{
    OsMemLinkRegisterRecord(node);
}
}

#[inline]
fn OsMemMergeNodeForReAllocBigger(pool: &mut c_void, alloc_size: u32, node: &mut OsMemNodeHead, node_size: u32, next_node: &mut OsMemNodeHead) {
    (*node).sizeAndFlag = node_size;
    OsMemFreeNodeDelete(pool, next_node as &mut OsMemFreeNodeHead);
    OsMemMergeNode(next_node);
#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
    OsLmsReallocMergeNodeMark(node);
}
    if (alloc_size + OS_MEM_MIN_LEFT_SIZE) <= (*node).sizeAndFlag {
        OsMemSplitNode(pool, node, alloc_size);
#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
    OsLmsReallocSplitNodeMark(node);
}
    } else {
#[cfg(feature = "LOSCFG_KERNEL_LMS")]
{
    OsLmsReallocResizeMark(node, alloc_size);
}
    }
    OS_MEM_NODE_SET_USED_FLAG((*node).sizeAndFlag);
    OsMemWaterUsedRecord(pool as &mut OsMemPoolHead, OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag) - node_size);
#[cfg(LOSCFG_MEM_LEAKCHECK == 1)]
{
    OsMemLinkRegisterRecord(node);
}
}

#[inline]
fn OsMemRealloc(pool: &mut OsMemPoolHead, ptr: &c_void, node: &mut OsMemNodeHead, size: u32, int_save: u32) -> Option<&mut c_void> {

    let mut next_node: &mut OsMemNodeHead = null_mut();
    let alloc_size: u32 = OS_MEM_ALIGN(size + OS_MEM_NODE_HEAD_SIZE, OS_MEM_ALIGN_SIZE);
    let node_size: u32 = OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag);
    let mut tmp_ptr: &mut c_void = null_mut();

    if node_size >= alloc_size {
        OsMemReAllocSmaller(pool, alloc_size, node, node_size);
        return ptr as &mut c_void;
    }

    next_node = OS_MEM_NEXT_NODE(node);
    if !OS_MEM_NODE_GET_USED_FLAG((*next_node).sizeAndFlag) && ((*next_node).sizeAndFlag + node_size) >= alloc_size {
        OsMemMergeNodeForReAllocBigger(pool, alloc_size, node, node_size, next_node);
        return ptr as &mut c_void;
    }

    tmp_ptr = OsMemAlloc(pool, size, int_save);
    if !tmp_ptr.is_null() {
        if memcpy_s(tmp_ptr, size as usize, ptr, (node_size - OS_MEM_NODE_HEAD_SIZE) as usize) != EOK {
            MEM_UNLOCK(pool, int_save);
            LOS_MemFree(pool as &mut c_void, tmp_ptr as &mut c_void);
            MEM_LOCK(pool, int_save);
            return null_mut();
        }
        OsMemFree(pool, node);
    }
    tmp_ptr
}

fn LOS_MemRealloc(pool: &mut c_void, ptr: &mut c_void, size: u32) -> Option<&mut c_void> {
    if pool.is_null() || OS_MEM_NODE_GET_USED_FLAG(size) || OS_MEM_NODE_GET_ALIGNED_FLAG(size) {
        return null_mut();
    }

    OsHookCall(LOS_HOOK_TYPE_MEM_REALLOC, pool, ptr, size);

    if ptr.is_null() {
        return LOS_MemAlloc(pool, size);
    }

    if size == 0 {
        LOS_MemFree(pool, ptr);
        return null_mut();
    }

    let size = if size < OS_MEM_MIN_ALLOC_SIZE {
        OS_MEM_MIN_ALLOC_SIZE
    } else {
        size
    };

    let pool_head: &mut OsMemNodeHead = pool as &mut OsMemPoolHead;
    let mut node: &mut OsMemNodeHead = null_mut();
    let mut new_ptr: &mut c_void = null_mut();
    let mut int_save: u32 = 0;

    MEM_LOCK(pool_head, int_save);
    loop {
        ptr = OsGetRealPtr(pool, ptr);
        if ptr.is_null() {
            break;
        }

        node = (ptr as usize - std::mem::size_of::<OsMemNodeHead>()) as &mut OsMemNodeHead;
        if OsMemCheckUsedNode(pool, node) != 0 {
            break;
        }

        new_ptr = OsMemRealloc(pool, ptr, node, size, int_save);
    }
    MEM_UNLOCK(pool_head, int_save);

    new_ptr
}

#[cfg(LOSCFG_MEM_FREE_BY_TASKID == 1)]
{
    fn MemNodeFreeByTaskIDHandle(cur_node: &mut OsMemNodeHead, arg: &mut c_void) {
        let args = arg as &mut u32;
        let task_id = *args;
        let pool_head = (*(args.offset(1)) as usize) as &mut OsMemPoolHead;
        
        if !OS_MEM_NODE_GET_USED_FLAG((*cur_node).sizeAndFlag) {
            return;
        }
        let node = cur_node as &mut OsMemUsedNodeHead;
        if (*node).header.taskID == task_id {
            OsMemFree(pool_head, &mut (*node).header);
        }
    }

    pub fn LOS_MemFreeByTaskID(pool: &mut c_void, task_id: u32) -> u32 {
        let args: [u32; 2] = [task_id, (pool as usize) as u32];

        if pool.is_null() || task_id >= LOSCFG_BASE_CORE_TSK_LIMIT/*在los_config_h.rs里定义*/ {
            return LOS_NOK;
        }

        OsAllMemNodeDoHandle(pool, MemNodeFreeByTaskIDHandle, args as &mut c_void);
        LOS_OK
    }
}

pub fn LOS_MemPoolSizeGet(pool: &c_void) -> u32{
    let mut count: u32 = 0;
    if pool.is_null() {
        LOS_NOK;
    }

    count += (*(pool as &mut OsMemPoolHead)).info.totalSize;
#[cfg(LOSCFG_MEM_MUL_REGIONS==1)] {
    count -= (*(pool as &mut OsMemPoolHead)).info.totalGapSize;
}
#[cfg(not(OS_MEM_EXPAND_ENABLE == 0))]{
    let mut size: u32 = 0;
    let mut node: &mut OsMemNodeHead = null_mut();
    let mut sentinel: &mut OsMemNodeHead = OS_MEM_END_NODE(pool, count);

    while !OsMemIsLastSentinelNode(sentinel) {
        size = OS_MEM_NODE_GET_SIZE((*sentinel).sizeAndFlag);
        node = OsMemSentinelNodeGet(sentinel);
        sentinel = OS_MEM_END_NODE(node, size);
        count += size;
    }       
}
    count;
}

fn MemUsedGetHandle(curNode: &OsMemNodeHead, arg: &mut c_void){
    let mut memUsed: &mut u32 = arg as &mut u32;
    if OS_MEM_IS_GAP_NODE(curNode) {
        *memUsed += OS_MEM_NODE_HEAD_SIZE;
    } else if OS_MEM_NODE_GET_USED_FLAG((*curNode).sizeAndFlag) {
        *memUsed += OS_MEM_NODE_GET_SIZE((*curNode).sizeAndFlag);
    }
    return;
}

pub fn LOS_MemTotalUsedGet(pool: &mut c_void) -> u32{
    let mut memUsed: u32 = 0;
    if pool.is_null() {
        LOS_NOK;
    }
    OsAllMemNodeDohandle(pool, MemUsedGetHandle, memUsed as &mut c_void);
    memUsed;
}

#[inline]
fn OsMemMagicCheckPrint(tmpNode: &mut *mut OsMemNodeHead) ->c_void{ // TOBECHECK
#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
{
    println!("[:s], {:d}, memory check error!\n"
    "memory used but magic num wrong, magic num = 0x{:x}\n",
    function_name!(), line!(), (*tmpNode)->magic);
}
}

fn OsMemAddrValidCheckPrint(pool: &c_void, tmpNode: &mut *mut OsMemFreeNodeHead) -> u32 {
    if ((*tmpNode)->prev != null_mut()) && !OsMemAddrValidCheck(pool as &OsMemPoolHead, (*(*tmpNode)).prev as &c_void) {
        println!("[{:s}], {:d}, memory check error!\n"
        " freeNode.prev: {:p} is out of legal mem range\n",
        function_name!(), line!(), (*tmpNode)->prev);
        return LOS_NOK;
    }
    if ((*tmpNode)->next != null_mut()) && !OsMemAddrValidCheck(pool as &OsMemPoolHead, (*(*tmpNode)).next as &c_void) {
        println!("[{:s}], {:d}, memory check error!\n"
        " freeNode.prev: {:p} is out of legal mem range\n",
        function_name!(), line!(), (*tmpNode)->prev);
        return LOS_NOK;
    }
    LOS_OK;
}

fn OsMemIntegrityCheckSub(tmpNode: &mut *mut OsMemNodeHead, pool: &c_void) ->u32 {
    if !OS_MEM_MAGIC_VALID(*tmpNode) {
        OsMemMagicCheckPrint(tmpNode);
        return LOS_NOK;
    }
    if !OsMemAddrValidCheck(pool as *const OsMemPoolHead,(*tmpNode)->ptr.prev as *const c_void) {
        println!("[{:s}], {:d}, memory check error!\n"
        " node prev: {:p} is out of legal mem range\n",function_name!(), line!(), (*(*tmpNode)).ptr.next);
        return LOS_NOK;
    }
    //fn OS_MEM_NODE_GET_USED_FLAG(size_and_flag: u32) -> u32 
    if !OS_MEM_NODE_GET_USED_FLAG((*tmpNode)->sizeAndFlag) {
        if OsMemAddrValidCheckPrint(pool, tmpNode as &mut *mut OsMemNodeHead) {
            return LOS_NOK;
        }
    }
    LOS_OK;
}

fn OsMemFreeListNodeCheck(pool: &OsMemPoolHead, node: &OsMemFreeNodeHead) -> u32 {
    if OsMemAddrValidCheck(pool, node as &c_void) ||
        (((*node).prev != null_mut()) && !OsMemAddrValidCheck(pool, (*node).prev as &c_void)) ||
        (((*node).next != null_mut()) && !OsMemAddrValidCheck(pool, (*node).next as &c_void))||
        !OsMemAddrValidCheck(pool, (*node).header.ptr.prev as &c_void) {
            return LOS_NOK;
        }
        //fn OS_MEM_IS_ALIGNED(a: u32, b: usize)
    if !OS_MEM_IS_ALIGNED(node as u32,std::mem::size_of::<&mut c_void>()) ||
    !OS_MEM_IS_ALIGNED((*node).prev as u32,std::mem::size_of::<&mut c_void>()) ||
    !OS_MEM_IS_ALIGNED((*node).next as u32,std::mem::size_of::<&mut c_void>()) ||
    !OS_MEM_IS_ALIGNED((*node).header.ptr.prev as u32,std::mem::size_of::<&mut c_void>()) {
        return LOS_NOK;
    }
    LOS_OK;
}

fn OsMemPoolHeadCheck(pool: &OsMemPoolHead){
    let mut tmpNode: &mut OsMemFreeNodeHead = null_mut();
    let mut index: u32 = 0;
    let mut flag: u32 = 0;
    if ((*pool).info.pool != pool) || !OS_MEM_IS_ALIGNED(pool, std::mem::size_of::<*mut c_void>()) {
        println!("wrong mem pool addr: {:p}, func: {:s}, line: {:d}\n", pool, function_name!(), line!());
        return;
    }
    while index < OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */ {
        tmpNode = (*pool).freeList[index];
        while !tmpNode.is_null() {
            if OsMemFreeListNodeCheck(pool, tmpNode as &OsMemFreeNodeHead) {
                flag = 1;
                println!("FreeListIndex: {:u}, node: {:p}, bNode: {:p}, prev:{:p}, next: {:p}\n",
                          index, tmpNode, (*tmpNode).header.ptr.prev, (*tmpNode).prev, (*tmpNode).next);
            }
            tmpNode = (*tmpNode).next;
        }

        index ++;
    }

    if flag {
        println!("mem pool info: poolAddr: {:p}, poolSize: 0x{:x}\n", pool, (*pool).info.totalSize);
#[cfg(LOSCFG_MEM_WATERLINE == 1)] {
        println!("mem pool info: poolWaterLine: 0x{:x}, poolCurUsedSize: 0x{:x}\n", (*pool).info.waterLine,
        (*pool).info.curUsedSize);
}
#[cfg(not(OS_MEM_EXPAND_ENABLE == 0))] {
        let mut size: u32 = 0;
        let mut node: &mut OsMemNodeHead = null_mut();
        let mut sentinel: &mut OsMemNodeHead = OS_MEM_END_NODE(pool, (*pool).info.totalSize);
        while !OsMemIsLastSentinelNode(sentinel) {
            size = OS_MEM_NODE_GET_SIZE((*sentinel).sizeAndFlag);
            node = OsMemSentinelNodeGet(sentinel);
            sentinel = OS_MEM_END_NODE(node, size);
            println!("expand node info: nodeAddr: 0x{:x}, nodeSize: 0x{:x}\n", node, size);
        }
}
    }
}

fn OsMemIntegrityCheck(pool: &OsMemPoolHead, tmp_node: &mut *mut OsMemNodeHead, pre_node: &mut *mut OsMemNodeHead) -> u32 {
    let mut end_node = OS_MEM_END_NODE(pool, (*pool).info.totalSize);

    OsMemPoolHeadCheck(pool);

    *pre_node = OS_MEM_FIRST_NODE(pool);
    loop {
        while *tmp_node < end_node {
            if OS_MEM_IS_GAP_NODE(*tmp_node) {
                *tmp_node = OS_MEM_NEXT_NODE(*tmp_node);
                continue;
            }
            if OsMemIntegrityCheckSub(tmp_node, pool) == LOS_NOK {
                return LOS_NOK;
            }
            *pre_node = *tmp_node;
            *tmp_node = OS_MEM_NEXT_NODE(*tmp_node);
        }
#[cfg(not(os_mem_expand_enable == 0))]
{
        if !OsMemIsLastSentinelNode(*tmp_node) {
            *pre_node = OsMemSentinelNodeGet(*tmp_node);
            end_node = OS_MEM_END_NODE(*pre_node, OS_MEM_NODE_GET_SIZE((*tmp_node).sizeAndFlag));
            continue;
        } 
}
        break;
    }
    LOS_OK
}

#[cfg(LOSCFG_KERNEL_PRINTF != 0)]
{
    fn OsMemNodeInfo(tmp_node: &OsMemNodeHead, pre_node: &OsMemNodeHead) {
        let mut used_node: &OsMemUsedNodeHead = null_mut();
        let mut free_node: &OsMemFreeNodeHead = null_mut();

        if tmp_node == pre_node {
            println!("\n the broken node is the first node\n");
        }

        if OS_MEM_NODE_GET_USED_FLAG((*tmp_node).sizeAndFlag) {
            used_node = tmp_node as &OsMemUsedNodeHead;
    #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
    {
        println!(
            "\n broken node head: {:?}  0x{:x}  0x{:x}, ",
            (*used_node).header.ptr.prev,
            (*used_node).header.magic,
            (*used_node).header.size_and_flag
        );
    }
    #[cfg(not(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1))]
    {
        println!(
            "\n broken node head: {:?}  0x{:x}, ",
            (*used_node).header.ptr.prev,
            (*used_node).header.size_and_flag
        );
    }
        } else {
            free_node = tmp_node as &OsMemFreeNodeHead;
    #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
    {   
            println!(
                "\n broken node head: {:?}  {:?}  {:?}  0x{:x}  0x{:x}, ",
                (*free_node).header.ptr.prev,
                (*free_node).next,
                (*free_node).prev,
                (*free_node).header.magic,
                (*free_node).header.size_and_flag
            );
    }
    #[cfg(not(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1))]
    {
            println!(
                "\n broken node head: {:?}  {:?}  {:?}  0x{:x}, ",
                (*free_node).header.ptr.prev,
                (*free_node).next,
                (*free_node).prev,
                (*free_node).header.size_and_flag
            );
            
    }

        if OS_MEM_NODE_GET_USED_FLAG((*pre_node).sizeAndFlag) {
            used_node = pre_node &OsMemUsedNodeHead;
    #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
    {
            println!(
                "prev node head: {:?}  0x{:x}  0x{:x}\n ",
                (*used_node).header.ptr.prev,
                (*used_node).header.magic,
                (*used_node).header.size_and_flag
            );
    }
    #[cfg(not(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1))]
    {
            println!(
                "prev node head: {:?}  0x{:x}\n ",
                (*used_node).header.ptr.prev,
                (*used_node).header.size_and_flag
            );
    }
        } else {
            free_node = pre_node as &OsMemFreeNodeHead;
    #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
    {   
            println!(
                "prev node head: {:?}  {:?}  {:?}  0x{:x}  0x{:x}, ",
                (*free_node).header.ptr.prev,
                (*free_node).next,
                (*free_node).prev,
                (*free_node).header.magic,
                (*free_node).header.size_and_flag
            );
    }
    #[cfg(not(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1))]
    {
            println!(
                "prev node head: {:?}  {:?}  {:?}  0x{:x}, ",
                (*free_node).header.ptr.prev,
                (*free_node).next,
                (*free_node).prev,
                (*free_node).header.size_and_flag
            );
    }
        }

    #[cfg(LOSCFG_MEM_LEAKCHECK == 1)]
    {
        OsMemNodeBacktraceInfo(tmp_node, pre_node);
    }
        }
    }
}

#[derive(Default)]
struct OsMemIntegrityCheckInfo {
    pre_node: OsMemNodeHead,
    err_node: OsMemNodeHead,
}

let g_integrity_check_record: OsMemIntegrityCheckInfo = Default::default();

/// 自定义的memcpy函数，将src指针指向的数据复制到dst指针指向的位置，复制长度为count个元素。
unsafe fn memcpy<T>(dst: *mut T, src: *const T, count: usize) {
    ptr::copy_nonoverlapping(src, dst, count);
}

#[inline]
fn OsMemCheckInfoRecord(err_node: &OsMemNodeHead, pre_node: &OsMemNodeHead) {
    let size_of_node = std::mem::size_of::<OsMemNodeHead>();
    memcpy(&mut g_integrity_check_record.preNode, pre_node, size_of_node);
    memcpy(&mut g_integrity_check_record.errNode, err_node, size_of_node);
}

fn OsMemIntegrityCheckError(
    pool: &mut OsMemPoolHead,
    tmp_node: &OsMemNodeHead,
    pre_node: &OsMemNodeHead,
    int_save: u32,
) {
#[cfg(not(LOSCFG_KERNEL_PRINTF == 0))]
{
    OsMemNodeInfo(tmp_node, pre_node);
}
    OsMemCheckInfoRecord(tmp_node, pre_node);

    #[cfg(any(LOSCFG_MEM_FREE_BY_TASKID == 1, LOSCFG_TASK_MEM_USED == 1))]
    {
        let mut task_cb: &mut LosTaskCB = null_mut();
        if OS_MEM_NODE_GET_USED_FLAG((*pre_node).sizeAndFlag) {
            let used_node = pre_node as &OsMemUsedNodeHead;
            let task_id = (*used_node).header.taskID;
            if task_id >= LOSCFG_BASE_CORE_TSK_LIMIT/*在los_config_h.rs里定义*/ {
                MEM_UNLOCK(pool, int_save);
                LOS_Panic("Task ID {:u} in pre node is invalid!\n", task_id);
            }

            task_cb = OS_TCB_FROM_TID(task_id);
            if (*task_cb).task_status & OS_TASK_STATUS_UNUSED != 0 || (*task_cb).task_entry.is_null() {
                MEM_UNLOCK(pool, int_save);
                LOS_Panic("\r\nTask ID {:u} in pre node is not created!\n", task_id);
            }

        } else {
            println!("The prev node is free");
        }
        MEM_UNLOCK(pool, int_save);

        println!(
            "cur node: 0x{:x}, pre node: 0x{:x}, pre node was allocated by task: {}, {}",
            tmp_node as u32,
            pre_node as u32,
            task_cb.taskID,
            task_cb.taskName
        );
        
        LOS_Panic("Memory integrity check error!\n");
    }

    #[cfg(not(any(LOSCFG_MEM_FREE_BY_TASKID == 1, LOSCFG_TASK_MEM_USED == 1)))]
    {
        MEM_UNLOCK(pool, int_save);
        LOS_Panic(
            "Memory integrity check error, cur node: 0x{:x}, pre node: 0x{:x}\n",
            tmp_node,
            pre_node
        );
    }
}

#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK == 1)]
fn OsMemAllocCheck(pool: &mut OsMemPoolHead, int_save: u32) -> u32 {
    let mut tmp_node: &OsMemNodeHead = null_mut();
    let mut pre_node: &OsMemNodeHead = null_mut();

    if OsMemIntegrityCheck(pool, &mut tmp_node, &mut pre_node) {
        OsMemIntegrityCheckError(pool, tmp_node, pre_node, int_save);
        return LOS_NOK;
    }
    LOS_OK
}


fn LOS_MemIntegrityCheck(pool: &c_void) -> u32 {
    // 检查输入指针是否为空
    if pool.is_null() {
        return LOS_NOK;
    }

    let pool_head = pool as &mut OsMemPoolHead;
    let mut tmp_node: &mut OsMemNodeHead = null_mut();
    let mut pre_node: &mut OsMemNodeHead = null_mut();
    let mut int_save = 0;

    MEM_LOCK(pool_head, int_save);
    if OsMemIntegrityCheck(pool_head, &mut tmp_node, &mut pre_node) {
        OsMemIntegrityCheckError(pool_head, tmp_node, pre_node, int_save);
        return LOS_NOK;
    } else {
        MEM_UNLOCK(pool_head, int_save);
        return LOS_OK;
    }
    
}

#[inline]
fn OsMemInfoGet(node: &mut OsMemNodeHead, pool_status: &mut LOS_MEM_POOL_STATUS) {
    let mut total_used_size = 0;
    let mut total_free_size = 0;
    let mut used_node_num = 0;
    let mut free_node_num = 0;
    let mut max_free_size = 0;
    let mut size: u32 = 0;

    if !OS_MEM_NODE_GET_USED_FLAG((*node).sizeAndFlag) {
        size = OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag);
        free_node_num += 1;
        total_free_size += size;
        if max_free_size < size {
            max_free_size = size;
        }
    } else {
        if OS_MEM_IS_GAP_NODE(node) {
            size = OS_MEM_NODE_HEAD_SIZE;
        } else {
            size = OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag);
        }
        used_node_num += 1;
        total_used_size += size;
    }

    (*pool_status).total_used_size += total_used_size;
    (*pool_status).total_free_size += total_free_size;
    (*pool_status).max_free_node_size = (*pool_status).max_free_node_size.max(max_free_size);
    (*pool_status).used_node_num += used_node_num;
    (*pool_status).free_node_num += free_node_num;
}

fn OsMemNodeInfoGetHandle(cur_node: &mut OsMemNodeHead, arg: &mut c_void) {
    let pool_status = arg as &mut LOS_MEM_POOL_STATUS;
    OsMemInfoGet(cur_node, pool_status);
}

pub fn LOS_MemInfoGet(pool: &mut c_void, pool_status: &mut LOS_MEM_POOL_STATUS) -> u32 {
    let pool_info = pool as &mut OsMemPoolHead;
    let mut int_save = 0;

    if pool_status.is_null() {
        println!("can't use NULL addr to save info");
        return LOS_NOK;
    }

    if pool.is_null() || (*pool_info).info.pool != pool {
        println!("wrong mem pool addr: {:p}, line:{}", pool_info as usize, line!());
        return LOS_NOK;
    }

    memset_s(&mut *poolStatus, std::mem::size_of::<LOS_MEM_POOL_STATUS>, 0, std::mem::size_of::<LOS_MEM_POOL_STATUS>)

    OsAllMemNodeDoHandle(pool, os_mem_node_info_get_handle, pool_status as *mut c_void);

    MEM_LOCK(pool_info, int_save);
    #![cfg(LOSCFG_MEM_WATERLINE == 1)]
    (*pool_status).usage_water_line = (*pool_info).info.water_line;
    MEM_UNLOCK(pool_info, int_save);

    LOS_OK
}

fn OsMemInfoPrint(pool: &mut c_void) {
    #[cfg(LOSCFG_KERNEL_PRINTF != 0)]
    {
        let pool_info = pool as &mut OsMemPoolHead;
        let mut status: LOS_MEM_POOL_STATUS = Default::default();

        if LOS_MemInfoGet(pool, &mut status) == LOS_NOK {
            return;
        }

        #[cfg(LOSCFG_MEM_WATERLINE == 1)]
        {
            println!("pool addr          pool size    used size     free size    max free node size   used node num     free node num      UsageWaterLine");
            println!("---------------    --------     -------       --------     --------------       -------------      ------------      ------------");
            println!("{:-16p}   0x{:08x}   0x{:08x}    0x{:08x}   0x{:016x}   0x{:013x}    0x{:013x}    0x{:013x}",
                     (*pool_info).info.pool, LOS_MemPoolSizeGet(pool), (*status).totalUsedSize,
                     (*status).totalFreeSize, (*status).maxFreeNodeSize, (*status).usedNodeNum,
                     (*status).freeNodeNum, (*status).usageWaterLine);
        }
        #[cfg(not(LOSCFG_MEM_WATERLINE == 1))]
        {
            println!("pool addr          pool size    used size     free size    max free node size   used node num     free node num");
            println!("---------------    --------     -------       --------     --------------       -------------      ------------");
            println!("{:-16p}   0x{:08x}   0x{:08x}    0x{:08x}   0x{:016x}   0x{:013x}    0x{:013x}",
                     (*pool_info).info.pool, LOS_MemPoolSizeGet(pool), (*status).totalUsedSize,
                     (*status).totalFreeSize, (*status).maxFreeNodeSize, (*status).usedNodeNum,
                     (*status).freeNodeNum);
        }
    }
}

fn LOS_MemFreeNodeShow(pool: &mut c_void) -> u32 {
    #[cfg(LOSCFG_KERNEL_PRINTF != 0)]
    {
        let pool_info = pool as &mut OsMemPoolHead;

        if pool_info.is_null() || (pool as usize) != (*pool_info).info.pool as usize {
            println!("wrong mem pool addr: {:p}, line: {}", pool_info as u8, line!());
            return LOS_NOK;
        }

        let mut node: &mut OsMemFreeNodeHead = null_mut();
        let mut count_num: [u32; OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */] = [0; OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */];
        let mut index: u32;
        let mut int_save = 0;

        MEM_LOCK(pool_info, int_save);
        for index in 0..OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */ {
            node = (*pool_info).free_list[index];
            while !node.is_null() {
                node = (*node).next;
                count_num[index] += 1;
            }
        }
        MEM_UNLOCK(pool_info, int_save);

        println!("\n   ************************ left free node number**********************");
        for index in 0..OS_MEM_FREE_LIST_COUNT/* 在los_memory_h.rs里定义 */ {
            if count_num[index] == 0 {
                continue;
            }

            print!("free index: {:03}, ", index);
            if index < OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/ {
                println!("size: [0x{:x}], num: {}", (index + 1) << 2, count_num[index]);
            } else {
                let val = 1 << (((index - OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/) >> OS_MEM_SLI/*在los_memory_h.rs里定义*/) + OS_MEM_LARGE_START_BUCKET/*在los_memory_h.rs里定义*/);
                let offset = val >> OS_MEM_SLI/*在los_memory_h.rs里定义*/;
                println!("size: [0x{:x}, 0x{:x}], num: {}",
                        (offset * ((index - OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/) % (1 << OS_MEM_SLI/*在los_memory_h.rs里定义*/))) + val,
                        ((offset * (((index - OS_MEM_SMALL_BUCKET_COUNT/*在los_memory_h.rs里定义*/) % (1 << OS_MEM_SLI/*在los_memory_h.rs里定义*/)) + 1)) + val - 1),
                        count_num[index]);
            }
        }
        println!("\n   ********************************************************************\n");

    }
    LOS_OK
}

pub fn LOS_MemUnlockEnable(pool: &mut c_void) {
    if pool.is_null() {
        return;
    }
    (*(pool as &mut OsMemPoolHead)).info.attr |= OS_MEM_POOL_UNLOCK_ENABLE;
}

#[cfg(LOSCFG_MEM_MUL_REGIONS == 1)]
{
    #[inline]
    fn OsMemMulRegionsParamCheck(pool: &mut c_void, mem_regions: &LosMemRegion, mem_region_count: u32) -> u32 {
        let mut last_start_address: &mut c_void = null_mut();
        let mut cur_start_address: &mut c_void;
        let mut last_length: u32 = 0;
        let mut cur_length: u32;
        let mut region_count = 0;

        if !pool.is_null() && (*(pool as &mut OsMemPoolHead)).info.pool != pool {
            println!("wrong mem pool addr: {:p}, func: {}, line: {}", pool, function_name!(), line!());
            return LOS_NOK;
        }

        if !pool.is_null() {
            last_start_address = pool;
            last_length = (*(pool as &mut OsMemPoolHead)).info.total_size;
        }

        while region_count < mem_region_count {
            let cur_start_address = (*mem_region).start_address;
            let cur_length = (*mem_region).length;
        
            if cur_start_address.is_null() || cur_length == 0 {
                println!("Memory address or length configured wrongly: address: {:p}, the length: 0x{:x}", cur_start_address as usize, cur_length);
                return LOS_NOK;
            }
        
            if (cur_start_address as usize) & (OS_MEM_ALIGN_SIZE - 1) != 0 || (cur_length & (OS_MEM_ALIGN_SIZE - 1)) != 0 {
                println!("Memory address or length configured not aligned: address: {:p}, the length: 0x{:x}, align size: {}", cur_start_address as usize, cur_length, OS_MEM_ALIGN_SIZE);
                return LOS_NOK;
            }
        
            if !last_start_address.is_null() && (last_start_address as usize + last_length) >= cur_start_address as usize {
                println!("Memory regions overlapped, the last start address: {:p}, the length: 0x{:x}, the current start address: {:p}", last_start_address as usize, last_length, cur_start_address as usize);
                return LOS_NOK;
            }
        
            mem_region += 1;
            region_count += 1;
            last_start_address = cur_start_address;
            last_length = cur_length;
        }
        
        LOS_OK
    }

    #[inline]
    pub fn OsMemMulRegionsLink(poolHead: &mut OsMemPoolHead,lastStartAddress: &mut c_void, lastLength: u32,
        lastEndNode: &mut OsMemNodeHead, memRegion: &LosMemRegion)
    {
        let mut curLength: u32 = 0;
        let mut gapSize: u32 = 0;
        let mut curEndNode: &mut OsMemNodeHead = null_mut();
        let mut curFreeNode: &mut OsMemNodeHead = null_mut();
        let mut curStartAddress: *mut c_void = null_mut();

        curStartAddress = (*memRegion).startAddress;
        curLength = (*memRegion).length;
    #[cfg(feature = "LOSCFG_KERNEL_LMS")]{  
        let mut resize: u32 = 0;
        if (!g_lms.isnull()) {   
            resize = (*g_lms).init(curStartAddress, curLength);
            curLength = (resize == 0) ? curLength : resize;
        }
    }

        gapSize = (curStartAddress (as &u8)) - ((poolHead)(as &u8) + (*poolHead).info.totalSize);
        (*lastEndNode).sizeAndFlag = gapSize + OS_MEM_NODE_HEAD_SIZE;
        OS_MEM_SET_MAGIC(lastEndNode);
        OS_MEM_NODE_SET_USED_FLAG((*lastEndNode).sizeAndFlag);

        OS_MEM_MARK_GAP_NODE(lastEndNode);

        (*poolHead).info.totalSize += (curLength + gapSize);
        (*poolHead).info.totalGapSize += gapSize;

        curFreeNode = curStartAddress as &mut OsMemNodeHead;
        (*curFreeNode).sizeAndFlag = curLength - OS_MEM_NODE_HEAD_SIZE;
        (*curFreeNode).ptr.prev = lastEndNode;
        OS_MEM_SET_MAGIC(curFreeNode);
        OsMemFreeNodeAdd(poolHead, curFreeNode as &OsMemFreeNodeHead);

        curEndNode = OS_MEM_END_NODE(curStartAddress, curLength);
        (*curEndNode).sizeAndFlag = 0;
        (*curEndNode).ptr.prev = curFreeNode;
        OS_MEM_SET_MAGIC(curEndNode);
        OS_MEM_NODE_SET_USED_FLAG((*curEndNode).sizeAndFlag);

    #[cfg(LOSCFG_MEM_WATERLINE == 1)]{
        (*poolHead).info.curUsedSize += OS_MEM_NODE_HEAD_SIZE;
        (*poolHead).info.waterLine = (*poolHead).info.curUsedSize;
    }

    }

    pub fn LOS_MemRegionsAdd(pool: &mut OsMemPoolHead, memRegions: &LosMemRegion , memRegionCount: u32) -> u32
    {
        let mut ret: u32 = 0;
        let mut lastLength: u32 = 0;
        let mut curLength: u32 = 0;
        let mut regionCount: u32 = 0;
        let mut poolHead: &mut OsMemPoolHead = null_mut(); //之前这几个变量是指针
        let mut lastEndNode: &mut OsMemNodeHead = null_mut();
        let mut firstFreeNode: &mut OsMemNodeHead = null_mut();
        let memRegion: &LosMemRegion = null_mut();
        let mut lastStartAddress: &mut c_void = null_mut();
        let mut curStartAddress: &mut c_void = null_mut();


        ret = OsMemMulRegionsParamCheck(pool, memRegions, memRegionCount);
        if (ret != LOS_OK) {
            return ret;
        }

        memRegion = memRegions;
        regionCount = 0;
        if (!pool.isnull()) { 
            poolHead = pool as &mut OsMemPoolHead;
            lastStartAddress = pool;
            lastLength = (*poolHead).info.totalSize;
        } else { 
            lastLength = (*memRegion).length;
            poolHead = ((*memRegion).startAddress) as &mut OsMemPoolHead;
            ret = LOS_MemInit(lastStartAddress, lastLength);
            if (ret != LOS_OK) {
                return ret;
            }
            memRegion++;
            regionCount++;
        }

        firstFreeNode = OS_MEM_FIRST_NODE(lastStartAddress); //这里是裸指针
        lastEndNode = OS_MEM_END_NODE(lastStartAddress, (*poolHead).info.totalSize);
        while (regionCount < memRegionCount) {
            curStartAddress = (*memRegion).startAddress;
            curLength = (*memRegion).length;

            OsMemMulRegionsLink(poolHead, lastStartAddress, lastLength, lastEndNode, memRegion);
            lastStartAddress = curStartAddress;
            lastLength = curLength;
            lastEndNode = OS_MEM_END_NODE(poolHead, (*poolHead).info.totalSize);
            memRegion++;
            regionCount++;
        }

        (*firstFreeNode).ptr.prev = lastEndNode;
        return ret;
    }
}


pub fn OsMemSystemInit -> u32 {
    let ret: u32 = 0;
#[cfg(LOSCFG_SYS_EXTERNAL_HEAP == 0)]
{
    m_aucSysMem0 = g_memStart;
}
#[cfg(not(LOSCFG_SYS_EXTERNAL_HEAP == 0))] 
{
    _aucSysMem0 = LOSCFG_SYS_HEAP_ADDR;
} 
    ret = LOS_MemInit(m_aucSysMem0, LOSCFG_SYS_HEAP_SIZE/* 在los_memory_h.rs里定义 */);
    println!("LiteOS heap memory address: {:p}, size: 0x{:x}", m_aucSysMem0, LOSCFG_SYS_HEAP_SIZE/* 在los_memory_h.rs里定义 */ as usize);
    return ret;
}

#[cfg(LOSCFG_PLATFORM_EXC == 1)]
{
    fn OsMemExcInfoGetSub(pool: &mut OsMemPoolHead, mem_exc_info: &mut MemInfoCB) {
        let mut tmp_node: &mut OsMemNodeHead = null_mut();
        let mut task_id: u32 = OS_TASK_ERRORID;
        let mut int_save: u32 = 0;

        memset_s(&mut *mem_exc_info, std::mem::size_of::<MemInfoCB>, 0, std::mem::size_of::<MemInfoCB>);
        MEM_LOCK(pool, int_save);
        (*mem_exc_info).type = MEM_MANG_MEMORY;
        (*mem_exc_info).startAddr = (*pool).info.pool as UINTPTR;
        (*mem_exc_info).size = (*pool).info.totalSize;
        (*mem_exc_info).free = (*pool).info.totalSize - (*pool).info.curUsedSize;

        let first_node: &OsMemNodeHead = OS_MEM_FIRST_NODE(pool);
        let end_node: &mut OsMemNodeHead = OS_MEM_END_NODE(pool, (*pool).info.totalSize);

        tmp_node = first_node;
        while tmp_node < end_node {
            (*mem_exc_info).blockSize += 1;
            if OS_MEM_NODE_GET_USED_FLAG((*tmp_node).size_and_flag) != 0 {
                if !OS_MEM_MAGIC_VALID(tmp_node) ||
                    OsMemAddrValidCheck(pool, (*tmp_node).ptr.prev) == 0 {
    #[cfg(any(LOSCFG_MEM_FREE_BY_TASKID == 1, LOSCFG_TASK_MEM_USED == 1))]
    {
                    task_id = (*(tmp_node as &mut OsMemUsedNodeHead)).header.taskID;
    }
                    (*mem_exc_info).errorAddr = ((*tmp_node as *const u8).offset(OS_MEM_NODE_HEAD_SIZE as isize)) as usize;
                    (*mem_exc_info).errorLen = OS_MEM_NODE_GET_SIZE((*tmp_node).sizeAndFlag) - OS_MEM_NODE_HEAD_SIZE;
                    (*mem_exc_info).errorOwner = task_id;
                    MEM_UNLOCK(pool, int_save);
                    return;
                }
            } else {
                let free_node = tmp_node as *mut OsMemFreeNodeHead;
                if OsMemAddrValidCheckPrint(pool, free_node) != 0 {
                    (*mem_exc_info).errorAddr = ((*tmp_node as *const u8).offset(OS_MEM_NODE_HEAD_SIZE as isize)) as usize;
                    (*mem_exc_info).errorLen = OS_MEM_NODE_GET_SIZE((*tmp_node).sizeAndFlag) - OS_MEM_NODE_HEAD_SIZE;
                    (*mem_exc_info).errorOwner = task_id;
                    MEM_UNLOCK(pool, int_save);
                    return;
                }
            }
            tmp_node = OS_MEM_NEXT_NODE(tmp_node);
        }
        
        MEM_UNLOCK(pool, int_save);
        return;
    }

    pub fn OsMemExcInfoGet(memNumMax: u32, memExcInfo: &MemInfoCB){
        let mut buffer: &mut u8 = memExcInfo as &mut u8;
        let mut count: u32 = 0;
    #[cfg(LOSCFG_MEM_MUL_POOL == 1)]
    {
        let mut memPool: &OsMemPoolHead = g_poolHead;
        while !memPool.is_null() {
            OsMemExcInfoGetSub(memPool, buffer as &mut MemInfoCB);
            count ++;
            buffer += std::mem:size_of::<MemInfoCB>;
            if count >= memNumMax {
                break;
            }
            memPool = (*memPool).nextPool;
        }
    }
    #[cfg(not(LOSCFG_MEM_MUL_POOL == 1))]
    {
        OsMemExcInfoGetSub(m_aucSysMem0, buffer);
        count++;
    }
        return count;
    }
}
