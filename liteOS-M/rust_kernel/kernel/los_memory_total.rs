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
static mut g_memStart: [u8; LOSCFG_SYS_HEAP_SIZE] = [0; LOSCFG_SYS_HEAP_SIZE]; // 初始化数组元素为0

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

fn mem_unlock(pool: &OsMemPoolHead, state: u32) {
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

macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
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
            if os_mem_is_last_sentinel_node(sentinel_node) {
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

        endNode = OS_MEM_END_NODE(newNode as *mut c_void, size1);
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
        UINT32 (*init)(const VOID *pool, UINT32 size);
        VOID (*deInit)(const VOID *pool);
        VOID (*mallocMark)(const VOID *curNodeStart, const VOID *nextNodeStart, UINT32 nodeHeadSize);
        VOID (*freeMark)(const VOID *curNodeStart, const VOID *nextNodeStart, UINT32 nodeHeadSize);
        VOID (*simpleMark)(UINTPTR startAddr, UINTPTR endAddr, UINT32 value);
        VOID (*check)(UINTPTR checkAddr, BOOL isFreeCheck);
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
        //VOID OsBackTraceHookCall(UINTPTR *LR, UINT32 LRSize, UINT32 jumpCount, UINTPTR SP)
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

    return OS_MEM_FREE_LIST_COUNT;
}

#[inline]
fn OsMemFindNextSuitableBlock(pool: &mut c_void, size: u32, outIndex: &mut u32) -> Option<&OsMemFreeNodeHead> {
    let poolHead: &OsMemPoolHead  = pool as &mut OsMemPoolHead;
    let mut fl: u32 = OsMemFlGet(size);
    let mut index: u32 = 0;
    let mut curIndex = OS_MEM_FREE_LIST_COUNT;
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
        if (tmp != OS_MEM_FREE_LIST_COUNT) {
            index = tmp;
            *outIndex = index;//change GOTO
            return poolHead->freeList[index] as *const c_void;
        }

        for (index = LOS_Align(index + 1, 32); index < OS_MEM_FREE_LIST_COUNT; index += 32) {
            /* 5: Divide by 32 to calculate the index of the bitmap array. */
            let mask = poolHead->freeListBitmap[index >> 5];
            if (mask != 0) {
                index = OsMemFFS(mask) + index;
                *outIndex = index;
                return poolHead->freeList[index] as &OsMemFreeNodeHead;
            }
        }
    } while(0);
    if (curIndex == OS_MEM_FREE_LIST_COUNT) {
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
    if(index>=OS_MEM_FREE_LIST_COUNT){
        LOS_Panic("The index of free lists is error, index = %u\n", index);
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
        OsMemFreeNodeDelete(pool, nextNode as &mut OsMemFreeNodeHead);
        OsMemMergeNode(nextNode);
    }
    OsMemFreeNodeAdd(pool, newFreeNode);
}
