fn LOS_MemIntegrityCheck(pool: *const std::ffi::c_void) -> u32 {
    // 检查输入指针是否为空
    if pool.is_null() {
        return LOS_NOK;
    }

    if pool.is_null() {
        return LOS_NOK;
    }

    let pool_head = pool as *mut OsMemPoolHead;
    let mut tmp_node: *mut OsMemNodeHead = ptr::null_mut();
    let mut pre_node: *mut OsMemNodeHead = ptr::null_mut();
    let mut int_save = 0;

    // 使用 Option::as_mut 方法将裸指针转换为可变引用
    unsafe {
        mem_lock(&mut *pool_head, &mut int_save);
        if os_mem_integrity_check(&mut *pool_head, &mut tmp_node, &mut pre_node) {
            os_mem_integrity_check_error(&mut *pool_head, tmp_node, pre_node, int_save);
            return LOS_NOK;
        } else {
            mem_unlock(&mut *pool_head, int_save);
            return LOS_OK;
        }
    }
}

fn os_mem_info_get(node: &OsMemNodeHead, pool_status: &mut LOS_MEM_POOL_STATUS) {
    let mut total_used_size = 0;
    let mut total_free_size = 0;
    let mut used_node_num = 0;
    let mut free_node_num = 0;
    let mut max_free_size = 0;
    let mut size: u32;

    if !os_mem_node_get_used_flag(node.size_and_flag) {
        size = os_mem_node_get_size(node.size_and_flag);
        free_node_num += 1;
        total_free_size += size;
        if max_free_size < size {
            max_free_size = size;
        }
    } else {
        if os_mem_is_gap_node(node) {
            size = OS_MEM_NODE_HEAD_SIZE;
        } else {
            size = os_mem_node_get_size(node.size_and_flag);
        }
        used_node_num += 1;
        total_used_size += size;
    }

    pool_status.total_used_size += total_used_size;
    pool_status.total_free_size += total_free_size;
    pool_status.max_free_node_size = pool_status.max_free_node_size.max(max_free_size);
    pool_status.used_node_num += used_node_num;
    pool_status.free_node_num += free_node_num;
}

fn os_mem_node_info_get_handle(cur_node: &OsMemNodeHead, arg: *mut std::ffi::c_void) {
    let pool_status = unsafe { &mut *(arg as *mut LOS_MEM_POOL_STATUS) };
    os_mem_info_get(cur_node, pool_status);
}

fn los_mem_info_get(pool: *mut std::ffi::c_void, pool_status: *mut LOS_MEM_POOL_STATUS) -> u32 {
    let pool_info = pool as *mut OsMemPoolHead;
    let mut int_save = 0;

    if pool_status.is_null() {
        println!("can't use NULL addr to save info");
        return LOS_NOK;
    }

    if pool.is_null() || unsafe { (*pool_info).info.pool != pool } {
        println!("wrong mem pool addr: {:p}, line:{}", pool_info as usize, line!());
        return LOS_NOK;
    }

    unsafe {
        std::ptr::write_bytes(pool_status as *mut u8, 0, std::mem::size_of::<LOS_MEM_POOL_STATUS>());
    }

    os_all_mem_node_do_handle(pool, os_mem_node_info_get_handle, pool_status as *mut std::ffi::c_void);

    unsafe {
        MEM_LOCK(pool_info, &mut int_save);
        #![cfg(LOSCFG_MEM_WATERLINE)]
        (*pool_status).usage_water_line = (*pool_info).info.water_line;
        MEM_UNLOCK(pool_info, int_save);
    }

    LOS_OK
}

fn os_mem_info_print(pool: *mut std::ffi::c_void) {
    #[cfg(LOSCFG_KERNEL_PRINTF != 0)]
    {
        let pool_info = pool as *mut OsMemPoolHead;
        let mut status: LOS_MEM_POOL_STATUS = Default::default();

        if los_mem_info_get(pool, &mut status) == LOS_NOK {
            return;
        }

        #[cfg(LOSCFG_MEM_WATERLINE == 1)]
        {
            println!("pool addr          pool size    used size     free size    max free node size   used node num     free node num      UsageWaterLine");
            println!("---------------    --------     -------       --------     --------------       -------------      ------------      ------------");
            println!("{:-16p}   0x{:08x}   0x{:08x}    0x{:08x}   0x{:016x}   0x{:013x}    0x{:013x}    0x{:013x}",
                     pool_info as *mut u8, los_mem_pool_size_get(pool), status.total_used_size,
                     status.total_free_size, status.max_free_node_size, status.used_node_num,
                     status.free_node_num, status.usage_water_line);
        }
        #[cfg(not(LOSCFG_MEM_WATERLINE == 1))]
        {
            println!("pool addr          pool size    used size     free size    max free node size   used node num     free node num");
            println!("---------------    --------     -------       --------     --------------       -------------      ------------");
            println!("{:-16p}   0x{:08x}   0x{:08x}    0x{:08x}   0x{:016x}   0x{:013x}    0x{:013x}",
                     pool_info as *mut u8, los_mem_pool_size_get(pool), status.total_used_size,
                     status.total_free_size, status.max_free_node_size, status.used_node_num,
                     status.free_node_num);
        }
    }
}

fn los_mem_free_node_show(pool: *mut std::ffi::c_void) -> u32 {
    #[cfg(LOSCFG_KERNEL_PRINTF != 0)]
    {
        let pool_info = pool as *mut OsMemPoolHead;

        if pool_info.is_null() || (pool as usize) != (*pool_info).info.pool as usize {
            println!("wrong mem pool addr: {:p}, line: {}", pool_info as *mut u8, line!());
            return LOS_NOK;
        }

        let mut node: *mut OsMemFreeNodeHead = ptr::null_mut();
        let mut count_num: [u32; OS_MEM_FREE_LIST_COUNT] = [0; OS_MEM_FREE_LIST_COUNT];
        let mut index: u32;
        let mut int_save = 0;

        unsafe {
            MEM_LOCK(pool_info, &mut int_save);
            for index in 0..OS_MEM_FREE_LIST_COUNT {
                node = (*pool_info).free_list[index];
                while !node.is_null() {
                    node = (*node).next;
                    count_num[index] += 1;
                }
            }
            MEM_UNLOCK(pool_info, int_save);
        }

        println!("\n   ************************ left free node number**********************");
        for index in 0..OS_MEM_FREE_LIST_COUNT {
            if count_num[index] == 0 {
                continue;
            }

            print!("free index: {:03}, ", index);
            if index < OS_MEM_SMALL_BUCKET_COUNT {
                println!("size: [0x{:x}], num: {}", (index + 1) << 2, count_num[index]);
            } else {
                let val = 1 << (((index - OS_MEM_SMALL_BUCKET_COUNT) >> OS_MEM_SLI) + OS_MEM_LARGE_START_BUCKET);
                let offset = val >> OS_MEM_SLI;
                println!("size: [0x{:x}, 0x{:x}], num: {}",
                        (offset * ((index - OS_MEM_SMALL_BUCKET_COUNT) % (1 << OS_MEM_SLI))) + val,
                        ((offset * (((index - OS_MEM_SMALL_BUCKET_COUNT) % (1 << OS_MEM_SLI)) + 1)) + val - 1),
                        count_num[index]);
            }
        }
        println!("\n   ********************************************************************\n");

    }
    LOS_OK
}

fn los_mem_unlock_enable(pool: *mut std::ffi::c_void) {
    if pool.is_null() {
        return;
    }

    unsafe {
        (*(pool as *mut OsMemPoolHead)).info.attr |= OS_MEM_POOL_UNLOCK_ENABLE;
    }
}

#[cfg(LOSCFG_MEM_MUL_REGIONS == 1)]
fn os_mem_mul_regions_param_check(pool: *mut std::ffi::c_void, mem_regions: &[LosMemRegion], mem_region_count: u32) -> u32 {
    let mut last_start_address: *mut std::ffi::c_void = std::ptr::null_mut();
    let mut cur_start_address: *mut std::ffi::c_void;
    let mut last_length: u32 = 0;
    let mut cur_length: u32;
    let mut region_count = 0;

    if !pool.is_null() && unsafe { (*(pool as *mut OsMemPoolHead)).info.pool != pool } {
        println!("wrong mem pool addr: {:p}, func: {}, line: {}", pool, std::stringify!(OsMemMulRegionsParamCheck), line!());
        return LOS_NOK;
    }

    if !pool.is_null() {
        last_start_address = pool;
        last_length = unsafe { (*(pool as *mut OsMemPoolHead)).info.total_size };
    }

    for mem_region in mem_regions.iter().take(mem_region_count as usize) {
        cur_start_address = mem_region.start_address;
        cur_length = mem_region.length;

        if cur_start_address.is_null() || cur_length == 0 {
            println!("Memory address or length configured wrongly: address: 0x{:x}, the length: 0x{:x}", cur_start_address as usize, cur_length);
            return LOS_NOK;
        }

        if ((cur_start_address as usize) & (OS_MEM_ALIGN_SIZE - 1) != 0) || (cur_length & (OS_MEM_ALIGN_SIZE - 1) != 0) {
            println!("Memory address or length configured not aligned: address: 0x{:x}, the length: 0x{:x}, alignsize: {}", cur_start_address as usize, cur_length, OS_MEM_ALIGN_SIZE);
            return LOS_NOK;
        }

        if !last_start_address.is_null() && unsafe { ((last_start_address as *mut u8).add(last_length as usize)) >= (cur_start_address as *mut u8) } {
            println!("Memory regions overlapped, the last start address: 0x{:x}, the length: 0x{:x}, the current start address: 0x{:x}", last_start_address as usize, last_length, cur_start_address as usize);
            return LOS_NOK;
        }

        region_count += 1;
        last_start_address = cur_start_address;
        last_length = cur_length;
    }

    LOS_OK
}
