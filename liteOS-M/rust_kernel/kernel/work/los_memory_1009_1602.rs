//1009
#[cfg(LOSCFG_MEM_MUL_POOL == 1)]
pub fn os_mem_pool_delete(pool: &mut VOID) -> UINT32 {
    let mut ret: UINT32 = LOS_NOK;
    let mut next_pool: Option<&mut VOID> = None;
    let mut cur_pool: Option<&mut VOID> = None;
    loop {
        if pool == &mut g_pool_head as &const VOID {
            g_pool_head = (*(g_pool_head as &const OsMemPoolHead)).next_pool as &const VOID;
            ret = LOS_OK;
            break;
        }

        cur_pool = g_pool_head;
        next_pool = g_pool_head;

        while let Some(next) = next_pool {
            if pool as &const VOID == next as &const VOID {
                if let Some(pool_head) = g_pool_head as_mut()
                {
                    *(cur_pool as &mut OsMemPoolHead).nextPool = *(pool_head as &const OsMemPoolHead).nextPool; 
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

// u8替换原来VOID类型指针
fn los_mem_init(pool: *mut VOID, size: UINT32) -> UINT32 {

    if pool.is_null() || size <= OS_MEM_MIN_POOL_SIZE {
        return LOS_NOK;
    }

    if (pool as UINTPTR) & (OS_MEM_ALIGN_SIZE - 1) != 0 || size & (OS_MEM_ALIGN_SIZE - 1) != 0 {
        println!("LiteOS heap memory address or size configured not aligned: address: 0x{:x}, size: 0x{:x}, alignsize: {}", pool as usize, size, OS_MEM_ALIGN_SIZE);
        return LOS_NOK;
    }

    if os_mem_pool_init(pool, size) != 0 {
        return LOS_NOK;
    }

    #[cfg(LOSCFG_MEM_MUL_POOL)]
    {
        if os_mem_pool_add(pool, size) != 0 {
            os_mem_pool_deinit(pool, size);
            return LOS_NOK;
        }
    }

    os_hook_call(LOS_HOOK_TYPE_MEM_INIT, pool, size);
    //找不到LOS_HOOK_TYPE_MEM_INIT TO BE DONE
    //hook_call函数找不到在哪
    LOS_OK
}

#[cfg(LOSCFG_MEM_MUL_POOL)]
fn los_mem_deinit(pool: *mut OsMemPoolHead) -> UINT32 {
    if pool.info.pool.is_null() {
        return LOS_NOK;
    }

    let tmp_pool_info = &pool.info;

    if tmp_pool_info.pool != pool as *mut _ as *mut u8 || tmp_pool_info.total_size <= OS_MEM_MIN_POOL_SIZE {
        return LOS_NOK;
    }

    if os_mem_pool_delete(pool) != 0 {
        return LOS_NOK;
    }

    os_mem_pool_deinit(pool, tmp_pool_info.total_size);

    os_hook_call(LOS_HOOK_TYPE_MEM_DEINIT, pool as *mut _ as *mut c_void);

    LOS_OK
}

#[cfg(LOSCFG_MEM_MUL_POOL)]
fn los_mem_pool_list() -> UINT32 {
    let mut next_pool = g_pool_head;
    let mut index = 0;

    while !next_pool.is_null() {
        println!("pool{} :", index);
        index += 1;
        os_mem_info_print(next_pool);
        next_pool = (*(next_pool as &mut OsMemPoolHead)).next_pool ;
        //能否实现一个trait来实现类型转换功能？
        // 定义一个特性来提供类型转换功能
        // trait AsMemPoolHead {
        //     fn as_mem_pool_head(&self) -> *mut OsMemPoolHead;
        // }

        // // 实现特性 for *mut c_void
        // impl AsMemPoolHead for *mut c_void {
        //     fn as_mem_pool_head(&self) -> *mut OsMemPoolHead {
        //         *self as *mut OsMemPoolHead
        //     }
        // }

        // // 实现特性 for Option<*mut c_void>
        // impl AsMemPoolHead for Option<*mut c_void> {
        //     fn as_mem_pool_head(&self) -> *mut OsMemPoolHead {
        //         match self {
        //             Some(ptr) => *ptr as *mut OsMemPoolHead,
        //             None => std::ptr::null_mut(),
        //         }
        //     }
        // }
    }
    index
}

fn os_mem_alloc(pool: *mut OsMemPoolHead, size: UINT32, int_save: UINT32) -> *mut c_void {
    // 计算对齐后的分配大小
    let alloc_size = os_mem_align(size + OS_MEM_NODE_HEAD_SIZE, OS_MEM_ALIGN_SIZE);
    #![allow(unused_variables)]
    let alloc_node: *mut OsMemNodeHead = null_mut();

    // 内存节点完整性检查
    #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
    if os_mem_alloc_check(pool, int_save) == LOS_NOK {
        return null_mut();
    }

    loop {
        // 尝试获取空闲内存节点
        let alloc_node = os_mem_free_node_get(pool, alloc_size);
        if alloc_node.is_null() {
            // 内存池扩展功能
            #[cfg(OS_MEM_EXPAND_ENABLE)]
            {
                if (*pool).info.attr & OS_MEM_POOL_EXPAND_ENABLE != 0 {
                    let ret = os_mem_pool_expand(pool, alloc_size, int_save);
                    if ret == 0 {
                        continue;
                    }
                }
            }

            // 低内存杀手功能
            #[cfg(LOSCFG_KERNEL_LMK)]
            {
                let kill_ret = los_lmk_tasks_kill();
                if kill_ret == LOS_OK {
                    continue;
                }
            }

            // 打印错误信息并返回空指针
            println!("---------------------------------------------------\
                      --------------------------------------------------------");
            mem_unlock(pool, int_save);
            os_mem_info_print(pool);
            mem_lock(pool, int_save);
            println!("[{}] No suitable free block, require free node size: 0x{:x}",
                     "OsMemAlloc", alloc_size);
            println!("----------------------------------------------------\
                      -------------------------------------------------------");
            return null_mut();
        }  
    }
    
    // 如果分配的节点大小足够，进行分割
    if alloc_size + OS_MEM_MIN_LEFT_SIZE <= (*alloc_node).size_and_flag {
        os_mem_split_node(pool, alloc_node, alloc_size);
    }

    // 设置节点为已使用状态
    OS_MEM_NODE_SET_USED_FLAG(&mut (*alloc_node).size_and_flag);
    os_mem_water_used_record(pool, OS_MEM_NODE_GET_SIZE((*alloc_node).size_and_flag));

    // 内存泄漏检查
    #[cfg(LOSCFG_MEM_LEAKCHECK)]
    os_mem_link_register_record(alloc_node);

    // 返回可用的内存节点
    return os_mem_create_used_node(alloc_node as *mut c_void);
}

fn los_mem_alloc(pool: *mut c_void, size: UINT32) -> *mut c_void {
    if pool.is_null() || size == 0 {
        return null_mut();
    }

    let size1 = if size < OS_MEM_MIN_ALLOC_SIZE {
        OS_MEM_MIN_ALLOC_SIZE
    } else {
        size
    };

    let pool_head = pool as *mut OsMemPoolHead;
    let mut ptr: *mut c_void = null_mut();
    let mut int_save = 0;

    mem_lock(pool_head, &mut int_save);
    if !(OS_MEM_NODE_GET_USED_FLAG(size1) || OS_MEM_NODE_GET_ALIGNED_FLAG(size1)) {
        ptr = os_mem_alloc(pool_head, size1, int_save);
    }
    mem_unlock(pool_head, int_save);

    os_hook_call(LOS_HOOK_TYPE_MEM_ALLOC, pool, ptr, size1);

    ptr
}

fn los_mem_alloc_align(pool: *mut c_void, size: UINT32, boundary: UINT32) -> *mut c_void {
    let mut gap_size: UINT32;

    if pool.is_null() || size == 0 || boundary == 0 || !os_mem_is_pow_two(boundary) ||
        !os_mem_is_aligned(boundary, std::mem::size_of::<*mut c_void>()) {
        return null_mut();
    }

    let mut adjusted_size = size;
    if adjusted_size < OS_MEM_MIN_ALLOC_SIZE {
        adjusted_size = OS_MEM_MIN_ALLOC_SIZE;
    }

    if boundary.checked_sub(std::mem::size_of::<UINT32>()).unwrap_or(0) > UINT32::MAX - adjusted_size {
        return null_mut();
    }
    //  std::mem::size_of::<UINT32>() 返回 UINT32 类型的大小（以字节为单位）。
    //  boundary.checked_sub(std::mem::size_of::<UINT32>()) 用于计算 boundary 减去 UINT32 类型的大小，如果结果超出了 UINT32 类型的范围，则返回 None。否则返回差值。
    //  unwrap_or(0) 如果结果是 Some(value)，则返回 value；如果结果是 None，则返回 0

    let use_size = (adjusted_size + boundary) - std::mem::size_of::<UINT32>();
    if OS_MEM_NODE_GET_USED_FLAG(use_size) || OS_MEM_NODE_GET_ALIGNED_FLAG(use_size) {
        return null_mut();
    }

    let pool_head = pool as *mut OsMemPoolHead;
    let mut int_save = 0;
    let mut ptr: *mut c_void = null_mut();
    let mut aligned_ptr: *mut c_void = null_mut();

    mem_lock(pool_head, &mut int_save);
    loop {
        ptr = os_mem_alloc(pool_head, use_size, int_save);
        aligned_ptr = os_mem_align(ptr, boundary) as *mut c_void;
        if ptr == aligned_ptr {
        #[cfg(LOSCFG_KERNEL_LMS)]
        {
            os_lms_alloc_align_mark(ptr, aligned_ptr, size);
        }
            break;
        }

        gap_size = aligned_ptr as UINT32 - ptr as UINT32;
        let alloc_node = (ptr as *mut OsMemUsedNodeHead).offset(-1);
        OS_MEM_NODE_SET_ALIGNED_FLAG((*alloc_node).header.sizeAndFlag);
        OS_MEM_SET_GAPSIZE_ALIGNED_FLAG(gap_size);
        unsafe {
            *((aligned_ptr as *mut UINT32).offset(-1)) = gap_size;
        }
        #[cfg(LOSCFG_KERNEL_LMS)]
        {
            os_lms_alloc_align_mark(ptr, aligned_ptr, size);
        }
        ptr = aligned_ptr;
        break;
    }
    mem_unlock(pool_head, int_save);
    os_hook_call(LOS_HOOK_TYPE_MEM_ALLOCALIGN, pool, ptr, size, boundary);
    ptr
}

// Static类型函数需要全部改成crate限制函数作用域为该文件内吗？
pub(crate) fn os_mem_addr_valid_check(pool: *const OsMemPoolHead, addr: *const c_void) -> bool {
    let mut size = unsafe { (*pool).info.totalSize };
    if os_mem_middle_addr_open_end((pool as usize + 1) as *const _, addr as usize, (pool as UINTPTR + size) as usize) {
        return true;
    }
    #[cfg(OS_MEM_EXPAND_ENABLE)]
    {
        let mut node: *mut OsMemNodeHead = null_mut();
        let mut sentinel = os_mem_end_node(pool, size);
        while !os_mem_is_last_sentinel_node(sentinel).is_true() {
            size = unsafe { os_mem_node_get_size((*sentinel).size_and_flag) };
            node = os_mem_sentinel_node_get(sentinel);
            sentinel = os_mem_end_node(node, size);
            if os_mem_middle_addr_open_end(node, addr as usize, (node as usize + size) as usize) {
                return true;
            }
        }
    }
    false
}

pub(crate) fn os_mem_is_node_valid(node: *const OsMemNodeHead, start_node: *const OsMemNodeHead, 
                                    end_node: *const OsMemNodeHead, pool_info: *const OsMemPoolHead) -> bool {
    if !os_mem_middle_addr(start_node, node, end_node) {
        return false;
    }

    if OS_MEM_NODE_GET_USED_FLAG(unsafe { (*node).size_and_flag }) {
        if !os_mem_magic_valid(node) {
            return false;
        }
        return true;
    }

    if !os_mem_addr_valid_check(pool_info, unsafe { (*node).ptr.prev }) {
        return false;
    }

    true
}

fn os_mem_check_used_node(pool: *const OsMemPoolHead, node: *const OsMemNodeHead) -> UINT32 {
    let mut start_node = unsafe { OS_MEM_FIRST_NODE(pool) as *const OsMemNodeHead };
    let mut end_node = unsafe { OS_MEM_END_NODE(pool, (*pool).info.totalSize) as *const OsMemNodeHead };
    let mut next_node: *const OsMemNodeHead = null_mut();
    let mut done_flag = false;

    loop {
        loop {
            if OS_MEM_IS_GAP_NODE(node) != 0 {
                break;
            }

            if !os_mem_is_node_valid(node, start_node, end_node, pool) {
                break;
            }

            if OS_MEM_NODE_GET_USED_FLAG(unsafe { (*node).size_and_flag }) == 0 {
                break;
            }

            next_node = unsafe { OS_MEM_NEXT_NODE(node) };
            if !os_mem_is_node_valid(next_node, start_node, end_node, pool) {
                break;
            }

            if OS_MEM_NODE_GET_LAST_FLAG(unsafe { (*next_node).size_and_flag }) == 0 &&
                OS_MEM_IS_GAP_NODE(next_node) == 0 {
                if unsafe { (*next_node).ptr.prev } != node {
                    break;
                }
            }

            if node != start_node &&
                (!os_mem_is_node_valid(unsafe { (*node).ptr.prev }, start_node, end_node, pool) ||
                unsafe { OS_MEM_NEXT_NODE((*node).ptr.prev) } != node) {
                break;
            }
            done_flag = true;
        }

        if !done_flag {
            #[cfg(OS_MEM_EXPAND_ENABLE)]
            {
                if os_mem_is_last_sentinel_node(end_node) == 0 {
                    start_node = unsafe { os_mem_sentinel_node_get(end_node) };
                    end_node = unsafe { OS_MEM_END_NODE(start_node, OS_MEM_NODE_GET_SIZE((*end_node).size_and_flag)) };
                    continue;
                }
            }
            return LOS_NOK;
        }
        break;
    }

    LOS_OK
}

fn os_mem_free(pool: *mut OsMemPoolHead, node: *mut OsMemNodeHead) -> UINT32 {
    let ret = os_mem_check_used_node(pool, node);
    if ret != LOS_OK {
        PRINT_ERR("OsMemFree check error!\n");
        return ret;
    }

    #[cfg(LOSCFG_MEM_WATERLINE)]
    {
        unsafe {
            (*pool).info.curUsedSize -= OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag);
        }
    }

    unsafe {
        (*node).sizeAndFlag = OS_MEM_NODE_GET_SIZE((*node).sizeAndFlag);
    }

    #[cfg(LOSCFG_MEM_LEAKCHECK)]
    {
        unsafe {
            os_mem_link_register_record(node);
        }
    }

    #[cfg(LOSCFG_KERNEL_LMS)]
    {
        let next_node_backup = unsafe { OS_MEM_NEXT_NODE(node) };
        let cur_node_backup = node;
        if !g_lms.is_null() {
            unsafe {
                (*g_lms).check((node as UINTPTR + OS_MEM_NODE_HEAD_SIZE) as usize, true);
            }
        }
    }

    let pre_node = unsafe { (*node).ptr.prev };
    if !pre_node.is_null() && (unsafe { !OS_MEM_NODE_GET_USED_FLAG((*pre_node).sizeAndFlag) }) {
        unsafe {
            os_mem_free_node_delete(pool, pre_node as *mut OsMemFreeNodeHead);
            os_mem_merge_node(node);
            node = pre_node;
        }
    }

    let next_node = unsafe { OS_MEM_NEXT_NODE(node) };
    if !next_node.is_null() && (unsafe { !OS_MEM_NODE_GET_USED_FLAG((*next_node).size_and_flag) }) {
        unsafe {
            os_mem_free_node_delete(pool, next_node as *mut OsMemFreeNodeHead);
            os_mem_merge_node(next_node);
        }
    }

    #[cfg(OS_MEM_EXPAND_ENABLE)]
    {
        if unsafe { (*pool).info.attr & OS_MEM_POOL_EXPAND_ENABLE } != 0 {
            let first_node = unsafe { OS_MEM_FIRST_NODE(pool) };
            if (unsafe { (*node).ptr.prev } > node) && (node != first_node) {
                if try_shrink_pool(pool, node) {
                    return LOS_OK;
                }
            }
        }
    }

    unsafe {
        os_mem_free_node_add(pool, node as *mut OsMemFreeNodeHead);
    }

    #[cfg(LOSCFG_KERNEL_LMS)]
    {
        if !g_lms.is_null() {
            unsafe {
                (*g_lms).free_mark(cur_node_backup, next_node_backup, OS_MEM_NODE_HEAD_SIZE);
            }
        }
    }

    ret
}

fn os_get_real_ptr(pool: *const VOID, ptr: *mut VOID) -> *mut VOID {
    let mut real_ptr = ptr;
    unsafe {
        let gap_size = *((ptr as *mut UINT32).offset(-1));
        if os_mem_gapsize_check(gap_size) != 0 {
            eprintln!("[os_get_real_ptr:{}] gapSize:0x{:x} error",line!() gap_size);
            return null_mut();
        }

        if OS_MEM_GET_GAPSIZE_ALIGNED_FLAG(gap_size) != 0 {
            let gap_size_aligned = OS_MEM_GET_ALIGNED_GAPSIZE(gap_size);
            if gap_size_aligned & (OS_MEM_ALIGN_SIZE - 1) != 0 ||
                gap_size_aligned > (ptr as usize - OS_MEM_NODE_HEAD_SIZE - pool as usize) {
                eprintln!("[os_get_real_ptr] gapSize:0x{:x} error", gap_size);
                return null_mut();
            }
            real_ptr = (ptr as usize - gap_size_aligned) as *mut VOID;
        }
    }
    real_ptr
}

pub fn los_mem_free(pool: *mut VOID, ptr: *mut VOID) -> UINT32 {
    if os_mem_is_null(pool) || os_mem_is_null(ptr) || !os_mem_is_aligned(pool, std::mem::size_of::<*mut VOID>()) || !os_mem_is_aligned(ptr, std::mem::size_of::<*mut VOID>()) {
        return LOS_NOK;
    }

    os_hook_call(LOS_HOOK_TYPE_MEM_FREE, pool, ptr);

    let mut ret = LOS_NOK;
    let pool_head: *mut os_mem_pool_head = pool as *mut _;
    let mut node: *mut os_mem_node_head = std::ptr::null_mut();
    let mut int_save: UINT32 = 0;

    os_mem_lock(pool_head, &mut int_save);
    loop {
        let real_ptr = os_get_real_ptr(pool, ptr);
        if os_mem_is_null(real_ptr) {
            break;
        }
        node = ((real_ptr as usize) - OS_MEM_NODE_HEAD_SIZE) as *mut _;
        ret = os_mem_free(pool_head, node);
        break;
    }
    os_mem_unlock(pool_head, int_save);

    ret
}


pub fn os_mem_realloc_smaller(pool: *mut VOID, alloc_size: UINT32, node: *mut os_mem_node_head, node_size: UINT32) {

    node.size_and_flag = node_size;
    if (alloc_size + OS_MEM_MIN_LEFT_SIZE) <= node_size {
        os_mem_split_node(pool, node, alloc_size);
        #[cfg(LOSCFG_MEM_WATERLINE)]
        {
            let pool_info: *mut os_mem_pool_head = pool as *mut _;
            unsafe {
                (*pool_info).info.cur_used_size -= node_size - alloc_size;
            }
        }
        #[cfg(LOSCFG_KERNEL_LMS)]
        os_lms_realloc_split_node_mark(node);
    } else {
        #[cfg(LOSCFG_KERNEL_LMS)]
        os_lms_realloc_resize_mark(node, alloc_size);
    }
    os_mem_node_set_used_flag(&mut node.size_and_flag);
    #[cfg(LOSCFG_MEM_LEAKCHECK)]
    os_mem_link_register_record(node);
}

pub fn os_mem_merge_node_for_realloc_bigger(pool: *mut VOID, alloc_size: UINT32, node: *mut os_mem_node_head, node_size: UINT32, next_node: *mut os_mem_node_head) {
    node.size_and_flag = node_size;
    os_mem_free_node_delete(pool, next_node as *mut _);
    os_mem_merge_node(next_node);
    #[cfg(LOSCFG_KERNEL_LMS)]
    os_lms_realloc_merge_node_mark(node);
    if (alloc_size + OS_MEM_MIN_LEFT_SIZE) <= node.size_and_flag {
        os_mem_split_node(pool, node, alloc_size);
        #[cfg(LOSCFG_KERNEL_LMS)]
        os_lms_realloc_split_node_mark(node);
    } else {
        #[cfg(LOSCFG_KERNEL_LMS)]
        os_lms_realloc_resize_mark(node, alloc_size);
    }
    os_mem_node_set_used_flag(&mut node.size_and_flag);
    #[cfg(LOSCFG_MEM_LEAKCHECK)]
    os_mem_link_register_record(node);
    #[cfg(LOSCFG_MEM_LEAKCHECK)]
    {
        let pool_head: *mut os_mem_pool_head = pool as *mut _;
        unsafe {
            os_mem_water_used_record(pool_head, os_mem_node_get_size(node.size_and_flag) - node_size);
        }
    }
}

pub fn os_mem_realloc(pool: *mut os_mem_pool_head, ptr: *const VOID, node: *mut os_mem_node_head, size: UINT32, int_save: UINT32) -> *mut VOID {
    use crate::os_mem_align;
    use crate::os_mem_node_get_size;
    use crate::os_mem_realloc_smaller;
    use crate::os_mem_merge_node_for_realloc_bigger;
    use crate::os_mem_alloc;

    let mut next_node: *mut os_mem_node_head = std::ptr::null_mut();
    let alloc_size: UINT32 = os_mem_align(size + OS_MEM_NODE_HEAD_SIZE, OS_MEM_ALIGN_SIZE);
    let node_size: UINT32 = os_mem_node_get_size((*node).size_and_flag);
    let mut tmp_ptr: *mut VOID = std::ptr::null_mut();

    if node_size >= alloc_size {
        os_mem_realloc_smaller(pool, alloc_size, node, node_size);
        return ptr as *mut _;
    }

    next_node = OS_MEM_NEXT_NODE(node);
    if !OS_MEM_NODE_GET_USED_FLAG((*next_node).size_and_flag) && ((*next_node).size_and_flag + node_size) >= alloc_size {
        os_mem_merge_node_for_realloc_bigger(pool, alloc_size, node, node_size, next_node);
        return ptr as *mut _;
    }

    tmp_ptr = os_mem_alloc(pool, size, int_save);
    if !tmp_ptr.is_null() {
        unsafe {
            if libc::memcpy(tmp_ptr, ptr, (node_size - OS_MEM_NODE_HEAD_SIZE) as usize) != EOK {
                crate::mem_unlock(pool, int_save);
                (VOID)LOS_MemFree(pool as *mut VOID, tmp_ptr);
                crate::mem_lock(pool, int_save);
                return std::ptr::null_mut();
            }
            (VOID)OsMemFree(pool, node);
        }
    }
    tmp_ptr
}

#[cfg(LOSCFG_MEM_FREE_BY_TASKID)]
fn mem_node_free_by_task_id_handle(cur_node: *mut OsMemNodeHead, arg: *mut VOID) {
    let args = arg as *mut UINT32;
    let task_id = unsafe { *args };
    let pool_head = unsafe { *(args.offset(1) as *mut *mut os_mem_pool_head) };
    let node = cur_node as *mut os_mem_used_node_head;
    if !os_mem_node_get_used_flag((*cur_node).size_and_flag) {
        return;
    }

    if unsafe { (*node).header.task_id == task_id } {
        os_mem_free(pool_head, &mut (*node).header);
    }
}

#[cfg(LOSCFG_MEM_FREE_BY_TASKID)]
pub fn los_mem_free_by_task_id(pool: *mut VOID, task_id: UINT32) -> UINT32 {
    let args: [UINT32; 2] = [task_id, pool as UINT32];

    if pool.is_null() || task_id >= LOSCFG_BASE_CORE_TSK_LIMIT {
        return LOS_NOK;
    }

    os_all_mem_node_do_handle(pool, mem_node_free_by_task_id_handle, args.as_mut_ptr() as *mut VOID);
    LOS_OK
}
