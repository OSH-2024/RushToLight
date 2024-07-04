mod los_memory_h.rs;
mod securec.rs;
mod los_arch.rs;
mod los_config.rs;
mod los_debug.rs;
mod los_hook.rs;
mod los_interrupt.rs;
mod los_task.rs;

#[cfg(LOSCFG_KERNEL_LMS)]
mod los_lms_pri.rs;

#[cfg(LOSCFG_KERNEL_LMK)]   //LOSCFG_KERNEL_LMK找不到在哪里定义的，只能认为其为0、1变量
mod los_lmk.rs;

//定义用于在编译时控制是否启用某些非必要功能。
const OS_MEM_EXPAND_ENABLE: UINT32 = 0;    //使用常量替换原 #define OS_MEM_EXPAND_ENABLE    0 

//系统内存的起始地址
let mut m_aucSysMem0: *mut UINT8 = std::ptr::null_mut();  

//g_memStart是一个数组，作为系统堆的存储空间
#[cfg(not(LOSCFG_SYS_EXTERNAL_HEAP))]
static mut G_MEM_START: [UINT8; LOSCFG_SYS_HEAP_SIZE] = [0; LOSCFG_SYS_HEAP_SIZE]; //LOSCFG_SYS_HEAP_SIZE在los_config.rs里定义好

//g_poolHead用于存储多个内存池的头部，如果配置为支持多内存池
#[cfg(LOSCFG_MEM_MUL_POOL)]
static mut G_POOL_HEAD: *mut () = std::ptr::null_mut(); //void不能被bindgen正确生成，用单元值进行对void的替换

//TLSF相关宏定义和内联函数,TLSF（Two-Level Segregated Fit
const OS_MEM_BITMAP_MASK: UINT32 = 0x1F;

// 找不到CLZ函数在哪定义，自己写了个定义
// 计算bitmap中最高位1的位置，CLZ 表示统计前导零的位数
fn clz(bitmap: UINT32) -> UINT32 {
    if bitmap == 0 {
        32
    } else {
        bitmap.leading_zeros()
    }
}
//内联函数，用于找到位图中第一个设置为1的位
pub fn os_mem_ffs(bitmap: UINT32) -> UINT16 {
    let new_bitmap = bitmap & !(bitmap + 1);
    OS_MEM_BITMAP_MASK - clz(new_bitmap) as UINT16
}

//内联函数，用于找到位图中最后一个设置为1的位
pub fn os_mem_fls(bitmap: UINT32) -> UINT16 {
    OS_MEM_BITMAP_MASK - clz(bitmap) as UINT16
}

//计算给定大小的对数
pub fn os_mem_log2(size: UINT32) -> UINT32 {
    if size > 0 {
        os_mem_fls(size) as UINT32
    } else {
        0
    }
}

pub fn os_mem_fl_get(size: UINT32) -> UINT32 {
    if size < OS_MEM_SMALL_BUCKET_MAX_SIZE {
        (size >> 2) - 1 /* 2: The small bucket setup is 4. */
    } else {
        os_mem_log2(size) - OS_MEM_LARGE_START_BUCKET + OS_MEM_SMALL_BUCKET_COUNT
    }
}

// 根据给定的大小和桶级别计算内存请求大小所在的桶的级别
pub fn os_mem_sl_get(size: UINT32, fl: UINT32) -> UINT32 {
    if fl < OS_MEM_SMALL_BUCKET_COUNT || size < OS_MEM_SMALL_BUCKET_MAX_SIZE {
        println!("fl or size is too small, fl = {}, size = {}", fl, size);
        return 0;
    }

    let sl = (size << OS_MEM_SLI) >> (fl - OS_MEM_SMALL_BUCKET_COUNT + OS_MEM_LARGE_START_BUCKET);
    sl - (1 << OS_MEM_SLI)
}
//在config.h无法被bindgen转换
#[cfg(not(LOSCFG_TASK_MEM_USED))]
pub const LOSCFG_TASK_MEM_USED: UINT32 = 0;

#[cfg(not(LOSCFG_MEM_FREE_BY_TASKID))]
pub const LOSCFG_MEM_FREE_BY_TASKID: UINT32 = 0;

#[cfg(not(LOSCFG_BASE_CORE_TSK_LIMIT))]
pub const LOSCFG_BASE_CORE_TSK_LIMIT: UINT32 = 5;

// 在满足指定条件时会触发错误
#[cfg(not(LOSCFG_TASK_MEM_USED))]
#[cfg(LOSCFG_MEM_FREE_BY_TASKID)]
#[cfg((LOSCFG_BASE_CORE_TSK_LIMIT + 1) > 64)]
compile_error!("When enter here, LOSCFG_BASE_CORE_TSK_LIMIT larger than 63 is not support");

pub struct OsMemUsedNodeHead {
    pub header: OsMemNodeHead,
}

pub const OS_MEM_POOL_EXPAND_ENABLE: UINT32 = 0x01; // 内存池支持扩展
pub const OS_MEM_POOL_UNLOCK_ENABLE: UINT32 = 0x02; // 内存池支持无锁操作

//MEM_LOCK部分在使用的地方进行展开,这里暂时不对复杂宏展开
//MEM_UNLOCK部分在使用的地方进行展开,这里暂时不对复杂宏展开

/* 内存节点魔术数字，用于检测内存节点的完整性 */
pub const OS_MEM_NODE_MAGIC: UINT32 = 0xABCDDCBA;

#[cfg(not(LOSCFG_TASK_MEM_USED != 1 && LOSCFG_MEM_FREE_BY_TASKID == 1))]
const OS_MEM_NODE_USED_FLAG: UINT32 = 1 << 31;
const OS_MEM_NODE_ALIGNED_FLAG: UINT32 = 1 << 30;
#[cfg(LOSCFG_MEM_LEAKCHECK == 1)]
const OS_MEM_NODE_LEAK_FLAG: UINT32 = 1 << 29;
#[cfg(not(LOSCFG_MEM_LEAKCHECK == 1))]
const OS_MEM_NODE_LEAK_FLAG: UINT32 = 0;
#[cfg(OS_MEM_EXPAND_ENABLE == 1)]
const OS_MEM_NODE_LAST_FLAG: UINT32 = 1 << 28; // Sentinel Node
#[cfg(not(OS_MEM_EXPAND_ENABLE == 1))]
const OS_MEM_NODE_LAST_FLAG: UINT32 = 0;

#[cfg(LOSCFG_TASK_MEM_USED != 1 && LOSCFG_MEM_FREE_BY_TASKID == 1)]
const OS_MEM_NODE_USED_FLAG: UINT32 = 1 << 25;
const OS_MEM_NODE_ALIGNED_FLAG: UINT32 = 1 << 24;
#[cfg(LOSCFG_MEM_LEAKCHECK == 1)]
const OS_MEM_NODE_LEAK_FLAG: UINT32 = 1 << 23;
#[cfg(not(LOSCFG_MEM_LEAKCHECK == 1))]
const OS_MEM_NODE_LEAK_FLAG: UINT32 = 0;
#[cfg(OS_MEM_EXPAND_ENABLE == 1)]
const OS_MEM_NODE_LAST_FLAG: UINT32 = 1 << 22; // Sentinel Node
#[cfg(not(OS_MEM_EXPAND_ENABLE == 1))]
const OS_MEM_NODE_LAST_FLAG: UINT32 = 0;

// 定义一个用于表示内存节点已用、对齐、泄漏和最后一个节点的标志的组合宏
const OS_MEM_NODE_ALIGNED_AND_USED_FLAG: usize = OS_MEM_NODE_USED_FLAG
    | OS_MEM_NODE_ALIGNED_FLAG
    | OS_MEM_NODE_LEAK_FLAG
    | OS_MEM_NODE_LAST_FLAG;

// 设置节点的对齐标记
fn os_mem_node_set_aligned_flag(size_and_flag: &mut UINT32) {
    *size_and_flag |= OS_MEM_NODE_ALIGNED_FLAG;
}

// 从节点大小和标记信息中获取已用标记
fn os_mem_node_get_used_flag(size_and_flag: UINT32) -> UINT32 {
    size_and_flag & OS_MEM_NODE_USED_FLAG
}

// 设置节点的已用标记
fn os_mem_node_set_used_flag(size_and_flag: &mut UINT32) {
    *size_and_flag |= OS_MEM_NODE_USED_FLAG;
}

// 获取节点的大小（去除标记位）
fn os_mem_node_get_size(size_and_flag: UINT32) -> UINT32 {
    size_and_flag & !OS_MEM_NODE_ALIGNED_AND_USED_FLAG
}

// 间隙大小的已用标记
const OS_MEM_GAPSIZE_USED_FLAG: UINT32 = 0x80000000;
// 间隙大小的对齐标记
const OS_MEM_GAPSIZE_ALIGNED_FLAG: UINT32 = 0x40000000;

// 获取对齐后的间隙大小
fn os_mem_get_aligned_gapsize(gapsize: UINT32) -> UINT32 {
    gapsize & !OS_MEM_GAPSIZE_ALIGNED_FLAG
}

// 获取间隙大小的对齐标记
fn os_mem_get_gapsize_aligned_flag(gapsize: UINT32) -> UINT32 {
    gapsize & OS_MEM_GAPSIZE_ALIGNED_FLAG
}

// 设置间隙大小的对齐标记
fn os_mem_set_gapsize_aligned_flag(gapsize: &mut UINT32) {
    *gapsize |= OS_MEM_GAPSIZE_ALIGNED_FLAG;
}

// 获取间隙大小的已用标记
fn os_mem_get_gapsize_used_flag(gapsize: UINT32) -> UINT32 {
    gapsize & OS_MEM_GAPSIZE_USED_FLAG
}

// 检查间隙大小的对齐和已用标记
fn os_mem_gapsize_check(gapsize: UINT32) -> bool {
    os_mem_get_gapsize_aligned_flag(gapsize) != 0 && os_mem_get_gapsize_used_flag(gapsize) != 0
}

// 设置节点为最后一个节点的标记
fn os_mem_node_set_last_flag(size_and_flag: &mut UINT32) {
    *size_and_flag |= OS_MEM_NODE_LAST_FLAG;
}

// 获取节点是否为最后一个节点的标记
fn os_mem_node_get_last_flag(size_and_flag: UINT32) -> UINT32 {
    size_and_flag & OS_MEM_NODE_LAST_FLAG
}

// 获取节点的泄漏标记
fn os_mem_node_get_leak_flag(size_and_flag: UINT32) -> UINT32 {
    size_and_flag & OS_MEM_NODE_LEAK_FLAG
}

// 基本内存对齐大小，通常是指针的大小
const OS_MEM_ALIGN_SIZE: usize = std::mem::size_of::<UINTPTR>();

// 检查一个值是否是2的幂
fn os_mem_is_pow_two(value: UINT32) -> bool {
    let value1 = value as UINTPTR;
    (value1 & (value1 - 1)) == 0
}

// 将指针p按照align_size大小对齐
fn os_mem_align(p: UINT32, align_size: usize) -> UINT32 { //使用usize进行相关指针操作，防止无定义行为
    let p1 = p as UINTPTR;
    let align_size1 = align_size as UINTPTR;
    (p1 + align_size1 - 1) & !(align_size1 - 1)
}

/* 检查地址a是否按照b对齐 */
fn os_mem_is_aligned(a: UINT32, b: usize) -> bool {
    let b1 = b as UINT32;
    !((a & (b1 - 1)) != 0)
}

// 内存节点头部的大小
const OS_MEM_NODE_HEAD_SIZE: usize = std::mem::size_of::<OsMemUsedNodeHead>();

// 内存池的最小大小，至少要容纳一个节点头部和一个内存池头部
const OS_MEM_MIN_POOL_SIZE: usize = OS_MEM_NODE_HEAD_SIZE + std::mem::size_of::<OsMemPoolHead>();

// 最小的剩余内存块大小，应能至少容纳一个空闲内存节点头部
const OS_MEM_MIN_LEFT_SIZE: usize = std::mem::size_of::<OsMemFreeNodeHead>();

// 最小的可分配内存大小
const OS_MEM_MIN_ALLOC_SIZE: UINT32 = 8;

/* 获取下一个内存节点的宏，通过当前节点的sizeAndFlag字段 */    //TO BE CHECKED
fn os_mem_next_node(node: &OsMemNodeHead) -> *const OsMemNodeHead {
    let next_node_offset = OS_MEM_NODE_GET_SIZE(node.size_and_flag) as isize;
    let next_node_ptr = (node as *const OsMemNodeHead).offset(next_node_offset) as *const OsMemNodeHead;   //wrapping_offset 是用于执行指针算术操作的安全方法
    next_node_ptr
}

// 定义获取内存池中第一个内存节点的函数
fn os_mem_first_node(pool: *const VOID) -> *const OsMemNodeHead {
    let pool_head_size = std::mem::size_of::<OsMemPoolHead>() as isize;
    let first_node_ptr = (pool as *const UINT8).offset(pool_head_size) as *const OsMemNodeHead;
    first_node_ptr
}

// 定义获取内存池末尾节点的函数
fn os_mem_end_node(pool: *const VOID, size: usize) -> *const OsMemNodeHead {
    // 计算内存池末尾节点的指针
    let end_node_ptr = (pool as *const UINT8).offset(size as isize - OS_MEM_NODE_HEAD_SIZE as isize) as *const OsMemNodeHead;
    end_node_ptr
}

// 定义判断中间地址是否在开始和结束地址之间（不包含结束地址）的函数
fn os_mem_middle_addr_open_end(start_addr: *const OsMemPoolHead, middle_addr: *const VOID, end_addr: UINT32) -> bool {
    let start_addr_ptr = start_addr as *const UINT8;
    let middle_addr_ptr = middle_addr as *const UINT8;
    let end_addr = end_addr as *const UINT8;

    // 判断中间地址是否在开始和结束地址之间（不包含结束地址）
    (start_addr <= middle_addr) && (middle_addr < end_addr)
}

// 定义判断中间地址是否在开始和结束地址之间（不包含结束地址）的函数
fn os_mem_middle_addr(start_addr: *const OsMemPoolHead, middle_addr: *const VOID, end_addr: UINT32) -> bool {
    let start_addr_ptr = start_addr as *const UINT8;
    let middle_addr_ptr = middle_addr as *const UINT8;
    let end_addr = end_addr as *const UINT8;

    // 判断中间地址是否在开始和结束地址之间（包含结束地址）
    (start_addr <= middle_addr) && (middle_addr <= end_addr)
}

#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
fn os_mem_set_magic(node: &mut OsMemNodeHead) {
    node.magic = OS_MEM_NODE_MAGIC;
}

// 如果启用了内存节点完整性检查，则定义相应的函数
#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
fn os_mem_magic_valid(node: &OsMemNodeHead) -> bool {
    node.magic == OS_MEM_NODE_MAGIC
}

// 如果未启用内存节点完整性检查，则定义相应的函数
#[cfg(not(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK))]
fn os_mem_set_magic(_node: &mut OsMemNodeHead) {
    // 当内存节点完整性检查被禁用时不执行任何操作
}

// 如果未启用内存节点完整性检查，则定义相应的函数
#[cfg(not(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK))]
fn os_mem_magic_valid(_node: &OsMemNode) -> bool {
    true // 当内存节点完整性检查被禁用时始终返回 true
}

// 如果启用了内存节点完整性检查，则声明相应的函数
#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
fn os_mem_alloc_check(pool: &mut OsMemPoolHead, int_save: u32) -> u32;



// 如果支持多内存区域配置，则定义与间隙节点相关的宏
#[cfg(LOSCFG_MEM_MUL_REGIONS)]
const OS_MEM_GAP_NODE_MAGIC: usize = 0xDCBAABCD;

#[cfg(LOSCFG_MEM_MUL_REGIONS)]
fn os_mem_mark_gap_node(node: &mut OsMemNodeHead) {
    node.ptr.prev = OS_MEM_GAP_NODE_MAGIC as *mut OsMemNodeHead;
}

#[cfg(LOSCFG_MEM_MUL_REGIONS)]
fn os_mem_is_gap_node(node: &OsMemNodeHead) -> bool {
    node.ptr.prev == OS_MEM_GAP_NODE_MAGIC as *const OsMemNodeHead
}

#[cfg(not(LOSCFG_MEM_MUL_REGIONS))]
fn os_mem_mark_gap_node(_node: &mut OsMemNodeHead) {
    // 当不支持多内存区域配置时，标记间隙节点的函数为空操作
}

#[cfg(not(LOSCFG_MEM_MUL_REGIONS))]
fn os_mem_is_gap_node(_node: &OsMemNodeHead) -> bool {
    false // 当不支持多内存区域配置时，间隙节点判断函数始终返回 false
}

// 添加空闲内存节点到内存的内联函数
fn os_mem_free_node_add(pool: &mut c_void, node: &mut OsMemFreeNodeHead)

// 从内存池释放内存节点的内联函数
fn os_mem_free(pool: &mut OsMemPoolHead, node: &mut OsMemNodeHead) -> u32 ;

// 打印内存池信息的函数
fn os_mem_info_print(pool: &mut c_void);

#[cfg(any(LOSCFG_MEM_FREE_BY_TASKID, LOSCFG_TASK_MEM_USED))]
fn os_mem_node_set_task_id(node: &mut OsMemUsedNodeHead) {
    node.header.task_id = LOS_CurTaskIDGet();
}
