/*#if (LOSCFG_BASE_CORE_CPUP == 1)
STATIC UINT32 GetAllTskCpupInfo(CPUP_INFO_S **cpuLessOneSec,
                                CPUP_INFO_S **cpuTenSec,
                                CPUP_INFO_S **cpuOneSec)
{
    if ((cpuLessOneSec == NULL) || (cpuTenSec == NULL) || (cpuOneSec == NULL)) {
        return OS_ERROR;
    }
    *cpuLessOneSec = (CPUP_INFO_S *)LOS_MemAlloc((VOID *)OS_SYS_MEM_ADDR, sizeof(CPUP_INFO_S) * g_taskMaxNum);
    if (*cpuLessOneSec == NULL) {
        PRINT_ERR("%s[%d] malloc failure!\n", __FUNCTION__, __LINE__);
        return OS_ERROR;
    }
    // Ignore the return code when matching CSEC rule 6.6(3).
    (VOID)memset_s((VOID *)(*cpuLessOneSec), sizeof(CPUP_INFO_S) * g_taskMaxNum,
                   (INT32)0, sizeof(CPUP_INFO_S) * g_taskMaxNum);

    *cpuTenSec = (CPUP_INFO_S *)LOS_MemAlloc((VOID *)OS_SYS_MEM_ADDR, sizeof(CPUP_INFO_S) * g_taskMaxNum);
    if (*cpuTenSec == NULL) {
        PRINT_ERR("%s[%d] malloc failure!\n", __FUNCTION__, __LINE__);
        (VOID)LOS_MemFree((VOID *)OS_SYS_MEM_ADDR, *cpuLessOneSec);
        *cpuLessOneSec = NULL;
        return OS_ERROR;
    }*/
    // Ignore the return code when matching CSEC rule 6.6(3).
    /*(VOID)memset_s((VOID *)(*cpuTenSec), sizeof(CPUP_INFO_S) * g_taskMaxNum,
                   (INT32)0, sizeof(CPUP_INFO_S) * g_taskMaxNum);

    *cpuOneSec = (CPUP_INFO_S *)LOS_MemAlloc((VOID *)OS_SYS_MEM_ADDR, sizeof(CPUP_INFO_S) * g_taskMaxNum);
    if (*cpuOneSec == NULL) {
        PRINT_ERR("%s[%d] malloc failure!\n", __FUNCTION__, __LINE__);
        (VOID)LOS_MemFree((VOID *)OS_SYS_MEM_ADDR, *cpuLessOneSec);
        (VOID)LOS_MemFree((VOID *)OS_SYS_MEM_ADDR, *cpuTenSec);
        return OS_ERROR;
    }
    // Ignore the return code when matching CSEC rule 6.6(3).
    (VOID)memset_s((VOID *)(*cpuOneSec), sizeof(CPUP_INFO_S) * g_taskMaxNum,
                   (INT32)0, sizeof(CPUP_INFO_S) * g_taskMaxNum);

    LOS_TaskLock();
    (VOID)LOS_AllTaskCpuUsage(*cpuLessOneSec, CPUP_LESS_THAN_1S);
    (VOID)LOS_AllTaskCpuUsage(*cpuTenSec, CPUP_IN_10S);
    (VOID)LOS_AllTaskCpuUsage(*cpuOneSec, CPUP_IN_1S);
    LOS_TaskUnlock();

    return LOS_OK;
}*/


#[cfg(LOSCFG_BASE_CORE_CPUP == 1)]{

unsafe fn GetAllTskCpupInfo->u32(cpuLessOneSec:*const *mut CPUP_INFO_S,cpuTenSec:*const *mut CPUP_INFO_S,cpuOneSec:*const *mut CPUP_INFO_S){
    ///////  Option<&mut Vec<CPUP_INFO_S>>表示一个可能为空的可变引用，该引用指向一个 Vec（动态数组），其中包含类型为 CPUP_INFO_S 的元素。
    if cpuLessOneSec.is_null() || cpuTenSec.is_null() || cpuOneSec.is_null() {
        return OS_ERROR;
    }
    *cpuLessOneSec =(LOS_MemAlloc(OS_SYS_MEM_ADDR as *mut VOID, (std::mem::size_of::<CPUP_INFO_S>())* g_taskMaxNum)) as *mut CPUP_INFO_S;
    if (*cpuLessOneSec).is_null() {
        eprintln!("{}[{}] malloc failure!", module_path!(), line!());//println输出到标准输出，eprintln输出到标准错误
        //当前函数名，当前行号
        return OS_ERROR;
    }
    // Ignore the return code when matching CSEC rule 6.6(3).
    let ptr1 = (*cpuLessOneSec).as_mut_ptr() as *mut u8;
    let len = (std::mem::size_of::<CPUP_INFO_S>())* g_taskMaxNum;
    std::ptr::write_bytes(ptr1, 0, len);//用这个代替C中的memset_s
    *cpuTenSec = (LOS_MemAlloc(OS_SYS_MEM_ADDR as *mut VOID, (std::mem::size_of::<CPUP_INFO_S>()) * g_taskMaxNum) )as *mut CPUP_INFO_S;
    if (*cpuTenSec).is_null() {
        eprintln!("{}[{}] malloc failure!", module_path!(), line!());
        LOS_MemFree(OS_SYS_MEM_ADDR as *mut VOID, (*cpuLessOneSec));//////////// *cpuLessOneSec
        *cpuLessOneSec =std::ptr::null_mut();
        return OS_ERROR;///////////////OS_ERROR???
    }
    
    // Ignore the return code when matching CSEC rule 6.6(3).
    let ptr2 = (*cpuTenSec).as_mut_ptr() as *mut u8;
    std::ptr::write_bytes(ptr2, 0, len);
    *cpuOneSec = (LOS_MemAlloc(OS_SYS_MEM_ADDR as *mut VOID, (std::mem::size_of::<CPUP_INFO_S>()) * g_taskMaxNum)) as *mut CPUP_INFO_S;
    if (*cpuOneSec).is_null() {
        eprintln!("{}[{}] malloc failure!", module_path!(), line!());
        LOS_MemFree(OS_SYS_MEM_ADDR as *mut VOID, *cpuLessOneSec);//////////// *cpuLessOneSec
        LOS_MemFree(OS_SYS_MEM_ADDR as *mut VOID, *cpuTenSec);//////////// *cpuLessOneSe
        return OS_ERROR;///////////////OS_ERROR???
    }
     // Ignore the return code when matching CSEC rule 6.6(3).
    let ptr3 = (*cpuOneSec).as_mut_ptr() as *mut u8;
    std::ptr::write_bytes(ptr3, 0, len);

    LOS_TaskLock();
    LOS_AllTaskCpuUsage(*cpuLessOneSec, CPUP_LESS_THAN_1S);/////  *cpuLessOneSec 是*mut CPUP_INFO_S型的
    LOS_AllTaskCpuUsage(*cpuTenSec, CPUP_IN_10S);
    LOS_AllTaskCpuUsage(*cpuOneSec, CPUP_IN_1S);
    LOS_TaskUnlock();   

    return LOS_OK;   
}
}//  #if (LOSCFG_BASE_CORE_CPUP == 1)  endif

/*STATIC VOID PrintTskInfo(const LosTaskCB *taskCB)
{
    UINT32 semID;

    if (taskCB->taskStatus & OS_TASK_STATUS_EXIT) {
        PRINTK("%4u%9u%10s%#10x%#10x%#11x%#11x%#10x%#7x",
               taskCB->taskID, taskCB->priority, OsConvertTskStatus(taskCB->taskStatus),
               taskCB->stackSize, 0, 0, 0, 0, 0);
        return;
    }

    semID = (taskCB->taskSem == NULL) ? OS_NULL_SHORT : (((LosSemCB *)taskCB->taskSem)->semID);
    PRINTK("%4u%9u%10s%#10x%#10x%#11x%#11x%#10x%#7x",
           taskCB->taskID, taskCB->priority, OsConvertTskStatus(taskCB->taskStatus),
           taskCB->stackSize, OsGetTaskWaterLine(taskCB->taskID),
           (UINT32)(UINTPTR)taskCB->stackPointer, taskCB->topOfStack, taskCB->eventMask, semID);
    return;
}*/

fn PrintTskInfo(taskCB:&LosTaskCB){
    let semID:u32 = if taskCB.taskSem.is_null() {
        OS_NULL_SHORT 
    } else {
        unsafe {
            (*(taskCB.taskSem as *const LosSemCB)).semID // Assuming taskSem points to a struct containing semID
        }//((LosSemCB *)taskCB->taskSem)中被强制类型转换为LosSemCB *类型的是taskCB->taskSem
    };

    // Print task information
    if LosTaskCB.taskStatus & OS_TASK_STATUS_EXIT != 0 {
        println!(
            "{:>4} {:>9} {:>10} {:>#10x} {:>#10x} {:>#11x} {:>#11x} {:>#10x} {:>#7x}",
            LosTaskCB.taskID,
            LosTaskCB.priority,
            OsConvertTskStatus(LosTaskCB.taskStatus), 
            LosTaskCB.stackSize,
            0,
            0,
            0,
            0,
            0
        );
    } else {
        println!(
            "{:>4} {:>9} {:>10} {:>#10x} {:>#10x} {:>#11x} {:>#11x} {:>#10x} {:>#7x}",
            LosTaskCB.taskID,
            LosTaskCB.priority,
            OsConvertTskStatus(LosTaskCB.taskStatus), // Assume os_convert_tsk_status is a function
            LosTaskCB.stackSize,
            OsGetTaskWaterLine(LosTaskCB.taskID), // Assume os_get_task_water_line is a function
            LosTaskCB.stackPointer as usize,
            LosTaskCB.topOfStack,
            LosTaskCB.eventMask,
            semID
        );
    }
}

/*STATIC VOID PrintTskInfoHeader(VOID)
{
    PRINTK("\r\n TID  Priority   Status StackSize WaterLine StackPoint TopOfStack EventMask  SemID");
#if (LOSCFG_TASK_MEM_USED == 1)
    PRINTK(" AllocSize");
#endif
#if (LOSCFG_BASE_CORE_CPUP == 1)
    PRINTK("  CPUUSE CPUUSE10s CPUUSE1s ");
#endif /* LOSCFG_BASE_CORE_CPUP */
    PRINTK("  TaskEntry name\n");
    PRINTK(" ---  -------- -------- ");
    PRINTK("--------- --------- ---------- ---------- --------- ------ ");
#if (LOSCFG_TASK_MEM_USED == 1)
    PRINTK("--------- ");
#endif
#if (LOSCFG_BASE_CORE_CPUP == 1)
    PRINTK("------- --------- --------  ");
#endif /* LOSCFG_BASE_CORE_CPUP */
    PRINTK("---------- ----\n");
}*/
fn PrintTskInfoHeader() {
    println!("\r\n TID  Priority   Status StackSize WaterLine StackPoint TopOfStack EventMask  SemID");
    
#[cfg(feature="LOSCFG_TASK_MEM_USED")]{
    println!(" AllocSize");
}
#[cfg(feature="LOSCFG_BASE_CORE_CPUP")]{
    println!("  CPUUSE CPUUSE10s CPUUSE1s ");
}

    println!("  TaskEntry name\n");
    println!(" ---  -------- -------- ");
    println!("--------- --------- ---------- ---------- --------- ------ ");
#[cfg(feature="LOSCFG_TASK_MEM_USED")]{
    println!("--------- ");
}
#[cfg(feature="LOSCFG_BASE_CORE_CPUP")]{
    println!("------- --------- --------  ");
}
    println!("---------- ----\n");
}

/*#if (LOSCFG_TASK_MEM_USED == 1)
STATIC UINT32                              g_taskMemUsed[LOSCFG_BASE_CORE_TSK_LIMIT + 1];
//声明了一个静态全局变量 有LOSCFG_BASE_CORE_TSK_LIMIT + 1个元素的u32数组，并把所有元素初始化为0
#endif*/
#[cfg(feature="LOSCFG_TASK_MEM_USED")]{
static mut g_taskMemUsed: [u32; LOSCFG_BASE_CORE_TSK_LIMIT + 1] = [0; LOSCFG_BASE_CORE_TSK_LIMIT + 1];
//rust中的static表明是全局变量（初始化为0） 
}

/*LITE_OS_SEC_TEXT_MINOR UINT32 OsGetAllTskInfo(VOID)
{
#if (LOSCFG_KERNEL_PRINTF != 0)
    LosTaskCB    *taskCB = (LosTaskCB *)NULL;
    UINT32       loopNum;
#if (LOSCFG_BASE_CORE_CPUP == 1)
    CPUP_INFO_S *cpuLessOneSec = (CPUP_INFO_S *)NULL;
    CPUP_INFO_S *cpuTenSec = (CPUP_INFO_S *)NULL;
    CPUP_INFO_S *cpuOneSec = (CPUP_INFO_S *)NULL;
#endif

#if (LOSCFG_TASK_MEM_USED == 1)
    (VOID)memset_s(g_taskMemUsed, sizeof(UINT32) * g_taskMaxNum, 0, sizeof(UINT32) * g_taskMaxNum);
    OsTaskMemUsed((VOID *)OS_SYS_MEM_ADDR, g_taskMemUsed, g_taskMaxNum);
#endif

#if (LOSCFG_BASE_CORE_CPUP == 1)
    if (GetAllTskCpupInfo(&cpuLessOneSec, &cpuTenSec, &cpuOneSec) != LOS_OK) {
        return OS_ERROR;
    }
#endif /* LOSCFG_BASE_CORE_CPUP */

    PrintTskInfoHeader();

    for (loopNum = 0; loopNum < g_taskMaxNum; loopNum++) {
        taskCB = (((LosTaskCB *)g_taskCBArray) + loopNum);
        if (taskCB->taskStatus & OS_TASK_STATUS_UNUSED) {
            continue;
        }

        PrintTskInfo(taskCB);
#if (LOSCFG_TASK_MEM_USED == 1)
        PRINTK("%#10x", g_taskMemUsed[loopNum]);
#endif

#if (LOSCFG_BASE_CORE_CPUP == 1)
        PRINTK("%6u.%-2u%7u.%-2u%6u.%-2u ",
               cpuLessOneSec[taskCB->taskID].uwUsage / LOS_CPUP_PRECISION_MULT,
               cpuLessOneSec[taskCB->taskID].uwUsage % LOS_CPUP_PRECISION_MULT,
               cpuTenSec[taskCB->taskID].uwUsage / LOS_CPUP_PRECISION_MULT,
               cpuTenSec[taskCB->taskID].uwUsage % LOS_CPUP_PRECISION_MULT,
               cpuOneSec[taskCB->taskID].uwUsage / LOS_CPUP_PRECISION_MULT,
               cpuOneSec[taskCB->taskID].uwUsage % LOS_CPUP_PRECISION_MULT);
#endif /* LOSCFG_BASE_CORE_CPUP */
        PRINTK("%#10x %-32s\n", (UINT32)(UINTPTR)taskCB->taskEntry, taskCB->taskName);
    }

#if (LOSCFG_BASE_CORE_CPUP == 1)
    (VOID)LOS_MemFree((VOID *)OS_SYS_MEM_ADDR, cpuLessOneSec);
    (VOID)LOS_MemFree((VOID *)OS_SYS_MEM_ADDR, cpuTenSec);
    (VOID)LOS_MemFree((VOID *)OS_SYS_MEM_ADDR, cpuOneSec);
#endif
#endif
    return LOS_OK;
}*/
//!!!!!!!!!!!!!!!!LITE_OS_SEC_TEXT_MINOR的问题
pub fn OsGetAllTskInfo-> u32 {
    // Ensure LOSCFG_KERNEL_PRINTF is enabled
#[cfg(feature = "LOSCFG_KERNEL_PRINTF")]{
    let mut taskCB: *mut LosTaskCB = std::ptr::null_mut();
    let mut loop_num: u32=0;

    #[cfg(feature = "LOSCFG_BASE_CORE_CPUP")]{
        let mut cpuLessOneSec:*mut CPUP_INFO_S=std::ptr::null_mut();
        let mut cpuTenSec:*mut CPUP_INFO_S=std::ptr::null_mut();
        let mut cpuOneSec:*mut CPUP_INFO_S=std::ptr::null_mut();
    }

    #[cfg(feature = "LOSCFG_TASK_MEM_USED")]{
    // g_taskMemUsed是全局变量，u32数组
        unsafe {
            let ptr = g_taskMemUsed.as_mut_ptr() as *mut u8;
            let len = std::mem::size_of::<u32>() * g_taskMaxNum;
            std::ptr::write_bytes(ptr, 0, len);
            OsTaskMemUsed(OS_SYS_MEM_ADDR as *mut std::ffi::c_void, g_taskMemUsed.as_mut_ptr(), g_taskMaxNum);
            //as_mut_ptr() 是 Vec 类型的一个方法，用于获取指向 Vec 内部数据的可变裸指针（raw pointer）,也可以用于数组
            ///g_taskMemUsed是静态全局数组 
        }
    }


    #[cfg(feature = "LOSCFG_BASE_CORE_CPUP")]
    {
        if unsafe { GetAllTskCpupInfo(&mut cpuLessOneSec, &mut cpuTenSec, &mut cpuOneSec) } != LOS_OK {
            return OS_ERROR;
        }
    }

    PrintTskInfoHeader();

    for loop_num in 0..g_taskMaxNum {
        unsafe {
            taskCB = (g_taskCBArray as *mut LosTaskCB).offset(loop_num as isize);//g_taskCBArray是全局变量
        }//offset指针偏移（以数组中元素大小为单位）
        if unsafe { (*taskCB).taskStatus & OS_TASK_STATUS_UNUSED } != 0 {
            continue;
        }

        PrintTskInfo(taskCB);

        #[cfg(feature = "LOSCFG_TASK_MEM_USED")]
        {
            println!("{:#10x}", g_taskMemUsed[loop_num as usize]);
        }

        #[cfg(feature = "LOSCFG_BASE_CORE_CPUP")]
        {
            unsafe {
                println!("{:>6}.{:02}<{:>7}.{:02}<{:>6}.{:02} ", 
                    (*cpuLessOneSec.offset((*taskCB).taskID as isize)).uwUsage / LOS_CPUP_PRECISION_MULT,
                    (*cpuLessOneSec.offset((*taskCB).taskID as isize)).uwUsage % LOS_CPUP_PRECISION_MULT,
                    (*cpuTenSec.offset((*taskCB).taskID as isize)).uwUsage / LOS_CPUP_PRECISION_MULT,
                    (*cpuTenSec.offset((*taskCB).taskID as isize)).uwUsage % LOS_CPUP_PRECISION_MULT,
                    (*cpuOneSec.offset((*taskCB).taskID as isize)).uwUsage / LOS_CPUP_PRECISION_MULT,
                    (*cpuOneSec.offset((*taskCB).taskID as isize)).uwUsage % LOS_CPUP_PRECISION_MULT);
            }
        }

        unsafe {
            println!("{:#10x} {:<32}", (*taskCB).taskEntry as u32 as usize, CStr::from_ptr((*taskCB).taskName).to_string_lossy());
        }
    }

    #[cfg(feature = "base_core_cpup")]
    {
        unsafe {
            LOS_mem_free(OS_SYS_MALLOC, cpuLessOneSec as *mut std::ffi::c_void);
            LOS_mem_free(OS_SYS_MALLOC, cpuTenSec as *mut std::ffi::c_void);
            LOS_mem_free(OS_SYS_MALLOC, cpuOneSec as *mut std::ffi::c_void);
        }
    }
}//LOSCFG_KERNEL_PRINTF的endif
    LOS_OK
}

/*LITE_OS_SEC_TEXT_INIT UINT32 OsTaskInit(VOID)
{
    UINT32 size;
    UINT32 index;

    g_taskMaxNum = LOSCFG_BASE_CORE_TSK_LIMIT + 1; /* Reserved 1 for IDLE */
    size = (g_taskMaxNum + 1) * sizeof(LosTaskCB);
    g_taskCBArray = (LosTaskCB *)LOS_MemAlloc(m_aucSysMem0, size);
    if (g_taskCBArray == NULL) {
        return LOS_ERRNO_TSK_NO_MEMORY;
    }

    // Ignore the return code when matching CSEC rule 6.6(1).
    (VOID)memset_s(g_taskCBArray, size, 0, size);
    LOS_ListInit(&g_losFreeTask);
    LOS_ListInit(&g_taskRecycleList);
    for (index = 0; index <= LOSCFG_BASE_CORE_TSK_LIMIT; index++) {
        g_taskCBArray[index].taskStatus = OS_TASK_STATUS_UNUSED;
        g_taskCBArray[index].taskID = index;
        LOS_ListTailInsert(&g_losFreeTask, &g_taskCBArray[index].pendList);
    }

    // Ignore the return code when matching CSEC rule 6.6(4).
    (VOID)memset_s((VOID *)(&g_losTask), sizeof(g_losTask), 0, sizeof(g_losTask));
    g_losTask.runTask = &g_taskCBArray[g_taskMaxNum];
    g_losTask.runTask->taskID = index;
    g_losTask.runTask->taskStatus = (OS_TASK_STATUS_UNUSED | OS_TASK_STATUS_RUNNING);
    g_losTask.runTask->priority = OS_TASK_PRIORITY_LOWEST + 1;

    g_idleTaskID = OS_INVALID;
    return OsSchedInit();
}*/
//!!1!!!!LITE_OS_SEC_TEXT_INIT
fn OsTaskInit->u32{
    let mut size:u32=(LOSCFG_BASE_CORE_TSK_LIMIT + 1) * std::mem::size_of::<LosTaskCB>();
    let mut index:u32=0;
    g_taskMaxNum = LOSCFG_BASE_CORE_TSK_LIMIT + 1;//全局变量
    g_taskCBArray = (LOS_MemAlloc(m_aucSysMem0, size)) as *mut LosTaskCB;
    if g_taskCBArray.is_null() {
        return LOS_ERRNO_TSK_NO_MEMORY;
    }
    // Ignore the return code when matching CSEC rule 6.6(1).
    unsafe {
        let ptr1 = g_taskCBArray.as_mut_ptr() as *mut u8;
        std::ptr::write_bytes(ptr1, 0, size);
    }

    LOS_ListInit(&g_losFreeTask);//extern LOS_DL_LIST          g_losFreeTask;////????直接用&g_losFreeTask??
    LOS_ListInit(&g_taskRecycleList);
    for index in 0..=LOSCFG_BASE_CORE_TSK_LIMIT {
        g_taskCBArray[index].taskStatus = OS_TASK_STATUS_UNUSED; // OS_TASK_STATUS_UNUSED
        g_taskCBArray[index].taskID = index;
        LOS_ListTailInsert(&g_losFreeTask, &g_taskCBArray[index].pendList);///&???????
    }
    // Ignore the return code when matching CSEC rule 6.6(4).
    unsafe {//用std::ptr::write_bytes代替C中的memset_s
        let ptr2 = &mut g_losTask as *mut LosTask as *mut u8;//LITE_OS_SEC_BSS  LosTask                             g_losTask;
        let len = std::mem::size_of::<LosTask>();
        std::ptr::write_bytes(ptr2, 0, len);
    }
    unsafe{
        g_losTask.runTask = g_taskCBArray.as_mut_ptr().add(g_taskMaxNum);
        (*g_losTask.runTask).taskID = index;
        (*g_losTask.runTask).taskStatus = (OS_TASK_STATUS_UNUSED | OS_TASK_STATUS_RUNNING);
        (*g_losTask.runTask).priority = OS_TASK_PRIORITY_LOWEST + 1;
    }

    g_idleTaskID = OS_INVALID;
    return OsSchedInit();
}

