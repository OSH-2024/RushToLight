fn os_mem_integrity_check(pool: &OsMemPoolHead, tmp_node: &mut *mut OsMemNodeHead, pre_node: &mut *mut OsMemNodeHead) -> u32 {
    let mut end_node = os_mem_end_node(pool, pool.info.totalSize);

    os_mem_pool_head_check(pool);

    *pre_node = os_mem_first_node(pool);
    loop {
        for *tmp_node = *pre_node; (*tmp_node as usize) < (end_node as usize); *tmp_node = os_mem_next_node(*tmp_node) {
            if os_mem_is_gap_node(*tmp_node) {
                continue;
            }
            if os_mem_integrity_check_sub(tmp_node, pool) == LOS_NOK {
                return LOS_NOK;
            }
            *pre_node = *tmp_node;
        }
        #[cfg(feature = "os_mem_expand_enable")]
        if !os_mem_is_last_sentinel_node(*tmp_node) {
            *pre_node = os_mem_sentinel_node_get(*tmp_node);
            end_node = os_mem_end_node(*pre_node, os_mem_node_get_size((*tmp_node).sizeAndFlag));
        } else {
            break;
        }

        #[cfg(not(feature = "os_mem_expand_enable"))]
        {
            break;
        }
    }
    LOS_OK
}

#[cfg(LOSCFG_KERNEL_PRINTF)]
fn os_mem_node_info(tmp_node: &OsMemNodeHead, pre_node: &OsMemNodeHead) {
    let mut used_node: Option<&OsMemUsedNodeHead> = None;
    let mut free_node: Option<&OsMemFreeNodeHead> = None;

    if ptr::eq(tmp_node, pre_node) {
        println!("\n the broken node is the first node\n");
    }

    if os_mem_node_get_used_flag(tmp_node.size_and_flag) {
        used_node = Some(unsafe { &*(tmp_node as *const OsMemNodeHead as *const OsMemUsedNodeHead) });
        println!("\n broken node head: {:p}  "
                 #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
                 "0x{:x}  "
                 "0x{:x}, ",
                 used_node.unwrap().header.ptr.prev,
                 #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
                 used_node.unwrap().header.magic,
                 used_node.unwrap().header.size_and_flag);
    } else {
        free_node = Some(unsafe { &*(tmp_node as *const OsMemNodeHead as *const OsMemFreeNodeHead) });
        println!("\n broken node head: {:p}  {:p}  {:p}  "
                 #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
                 "0x{:x}  "
                 "0x{:x}, ",
                 free_node.unwrap().header.ptr.prev, free_node.unwrap().next, free_node.unwrap().prev,
                 #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
                 free_node.unwrap().header.magic,
                 free_node.unwrap().header.size_and_flag);
    }

    if os_mem_node_get_used_flag(pre_node.size_and_flag) {
        used_node = Some(unsafe { &*(pre_node as *const OsMemNodeHead as *const OsMemUsedNodeHead) });
        println!("prev node head: {:p}  "
                 #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
                 "0x{:x}  "
                 "0x{:x}\n",
                 used_node.unwrap().header.ptr.prev,
                 #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
                 used_node.unwrap().header.magic,
                 used_node.unwrap().header.size_and_flag);
    } else {
        free_node = Some(unsafe { &*(pre_node as *const OsMemNodeHead as *const OsMemFreeNodeHead) });
        println!("prev node head: {:p}  {:p}  {:p}  "
                 #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
                 "0x{:x}  "
                 "0x{:x}, ",
                 free_node.unwrap().header.ptr.prev, free_node.unwrap().next, free_node.unwrap().prev,
                 #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
                 free_node.unwrap().header.magic,
                 free_node.unwrap().header.size_and_flag);
    }

    #[cfg(LOSCFG_MEM_LEAKCHECK)]
    os_mem_node_backtrace_info(tmp_node, pre_node);
}

#[derive(Default)]
struct OsMemIntegrityCheckInfo {
    pre_node: OsMemNodeHead,
    err_node: OsMemNodeHead,
}   //加到.h文件里

let g_integrity_check_record: OsMemIntegrityCheckInfo = Default::default();

fn os_mem_check_info_record(err_node: &OsMemNodeHead, pre_node: &OsMemNodeHead) {
    unsafe {
        G_INTEGRITY_CHECK_RECORD.pre_node = pre_node.clone();
        G_INTEGRITY_CHECK_RECORD.err_node = err_node.clone();
    }
}

fn os_mem_integrity_check_error(
    pool: &mut OsMemPoolHead,
    tmp_node: &OsMemNodeHead,
    pre_node: &OsMemNodeHead,
    int_save: u32,
) {
    #[cfg(LOSCFG_KERNEL_PRINTF)]
    os_mem_node_info(tmp_node, pre_node);

    os_mem_check_info_record(tmp_node, pre_node);

    #[cfg(any(LOSCFG_MEM_FREE_BY_TASKID, LOSCFG_TASK_MEM_USED))]
    {
        let mut task_cb: Option<&mut LosTaskCB> = None;
        if os_mem_node_get_used_flag(pre_node.size_and_flag) {
            let used_node = unsafe { &*(pre_node as *const OsMemNodeHead as *const OsMemUsedNodeHead) };
            let task_id = used_node.header.task_id;
            if task_id >= LOSCFG_BASE_CORE_TSK_LIMIT {
                mem_unlock(pool, int_save);
                los_panic(&format!("Task ID {} in pre node is invalid!\n", task_id));
            }

            task_cb = Some(os_tcb_from_tid(task_id));
            if let Some(task_cb) = task_cb {
                if task_cb.task_status & OS_TASK_STATUS_UNUSED != 0 || task_cb.task_entry.is_none() {
                    mem_unlock(pool, int_save);
                    los_panic(&format!("\r\nTask ID {} in pre node is not created!\n", task_id));
                }
            }
        } else {
            println!("The prev node is free");
        }
        mem_unlock(pool, int_save);
        if let Some(task_cb) = task_cb {
            println!(
                "cur node: 0x{:x}, pre node: 0x{:x}, pre node was allocated by task: {}, {}",
                tmp_node as *const _ as usize,
                pre_node as *const _ as usize,
                task_cb.task_id,
                task_cb.task_name
            );
        }
        los_panic("Memory integrity check error!\n");
    }

    #[cfg(not(any(LOSCFG_MEM_FREE_BY_TASKID, LOSCFG_TASK_MEM_USED)))]
    {
        mem_unlock(pool, int_save);
        los_panic(&format!(
            "Memory integrity check error, cur node: 0x{:x}, pre node: 0x{:x}\n",
            tmp_node as *const _ as usize,
            pre_node as *const _ as usize
        ));
    }
}

#[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
fn os_mem_alloc_check(pool: &mut OsMemPoolHead, int_save: u32) -> u32 {
    let mut tmp_node: Option<OsMemNodeHead> = None;
    let mut pre_node: Option<OsMemNodeHead> = None;

    if os_mem_integrity_check(pool, &mut tmp_node, &mut pre_node) {
        os_mem_integrity_check_error(pool, &tmp_node.unwrap(), &pre_node.unwrap(), int_save);
        return LOS_NOK;
    }
    LOS_OK
}
