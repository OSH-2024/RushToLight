/*****************************************************************************
 Function    : OsIdleTaskCreate
 Description : Create idle task.
 Input       : None
 Output      : None
 Return      : LOS_OK on success or error code on failure
 *****************************************************************************/

/*
 LITE_OS_SEC_TEXT_INIT UINT32 OsIdleTaskCreate(VOID)
 {//
     UINT32 retVal;
     TSK_INIT_PARAM_S taskInitParam;
     // Ignore the return code when matching CSEC rule 6.6(4).
     (VOID)memset_s((VOID *)(&taskInitParam), sizeof(TSK_INIT_PARAM_S), 0, sizeof(TSK_INIT_PARAM_S));
     taskInitParam.pfnTaskEntry = (TSK_ENTRY_FUNC)OsIdleTask;
     taskInitParam.uwStackSize = LOSCFG_BASE_CORE_TSK_IDLE_STACK_SIZE;
     taskInitParam.pcName = "IdleCore000";
     taskInitParam.usTaskPrio = OS_TASK_PRIORITY_LOWEST;
     retVal = LOS_TaskCreateOnly(&g_idleTaskID, &taskInitParam);
     if (retVal != LOS_OK) {
         return retVal;
     }
 
     OsSchedSetIdleTaskSchedParam(OS_TCB_FROM_TID(g_idleTaskID));
     return LOS_OK;
 }
*/

pub fn os_idle_task_create() -> u32 { //
    let mut task_init_param = TSK_INIT_PARAM_S {//这段结构体是在task.h里面有个定义的，这里应该是截取了几个有用的进行赋值？
        pfn_task_entry: Some(os_idle_task),
        uw_stack_size: LOSCFG_BASE_CORE_TSK_IDLE_STACK_SIZE,
        pc_name: "IdleCore000".as_ptr(),
        us_task_prio: OS_TASK_PRIORITY_LOWEST,
    };

    let ret_val = unsafe { los_task_create_only(&mut G_IDLE_TASK_ID, &mut task_init_param) };//这部分应该没问题
    if ret_val != LOS_OK {
        return ret_val;
    }

    unsafe {
        let os_tcb = os_tcb_from_tid(G_IDLE_TASK_ID);
        os_sched_set_idle_task_sched_param(os_tcb);
    }

    LOS_OK
}

 /*****************************************************************************
  Function    : LOS_CurTaskIDGet
  Description : get id of current running task.
  Input       : None
  Output      : None
  Return      : task id
  *****************************************************************************/
 LITE_OS_SEC_TEXT UINT32 LOS_CurTaskIDGet(VOID)
 {
     if (g_losTask.runTask == NULL) {
         return LOS_ERRNO_TSK_ID_INVALID;
     }
     return g_losTask.runTask->taskID;
 }
 
 pub fn los_cur_task_id_get() -> u32 {
    unsafe {
        if let Some(run_task) = &LOS_TASK.run_task {
            run_task.task_id
        } else {
            LOS_ERRNO_TSK_ID_INVALID
        }
    }
}


 /*****************************************************************************
  Function    : LOS_NextTaskIDGet
  Description : get id of next running task.
  Input       : None
  Output      : None
  Return      : task id
  *****************************************************************************/
 LITE_OS_SEC_TEXT UINT32 LOS_NextTaskIDGet(VOID)
 {
     UINT32 intSave = LOS_IntLock();
     UINT32 taskID = OsGetTopTask()->taskID;
     LOS_IntRestore(intSave);
 
     return taskID;
 }
 
 /*****************************************************************************
  Function    : LOS_CurTaskNameGet
  Description : get name of current running task.
  Input       : None
  Output      : None
  Return      : task name
  *****************************************************************************/
 LITE_OS_SEC_TEXT CHAR *LOS_CurTaskNameGet(VOID)
 {
     CHAR *taskName = NULL;
 
     if (g_losTask.runTask != NULL) {
         taskName = g_losTask.runTask->taskName;
     }
 
     return taskName;
 }
 
 #if (LOSCFG_BASE_CORE_TSK_MONITOR == 1)
 #if (LOSCFG_EXC_HARDWARE_STACK_PROTECTION == 0)
 /*****************************************************************************
  Function    : OsHandleRunTaskStackOverflow
  Description : handle stack overflow exception of the run task.
  Input       : None
  Output      : None
  Return      : None
  *****************************************************************************/
 LITE_OS_SEC_TEXT STATIC VOID OsHandleRunTaskStackOverflow(VOID)
 {
     PRINT_ERR("CURRENT task ID: %s:%d stack overflow!\n",
               g_losTask.runTask->taskName, g_losTask.runTask->taskID);
     OsDoExcHook(EXC_STACKOVERFLOW);
 }
 
 /*****************************************************************************
  Function    : OsHandleNewTaskStackOverflow
  Description : handle stack overflow exception of the new task.
  Input       : None
  Output      : None
  Return      : None
  *****************************************************************************/
 LITE_OS_SEC_TEXT STATIC VOID OsHandleNewTaskStackOverflow(VOID)
 {
     LosTaskCB *tmp = NULL;
 
     PRINT_ERR("HIGHEST task ID: %s:%d SP error!\n",
               g_losTask.newTask->taskName, g_losTask.newTask->taskID);
     PRINT_ERR("HIGHEST task StackPointer: 0x%x TopOfStack: 0x%x\n",
               (UINT32)(UINTPTR)(g_losTask.newTask->stackPointer), g_losTask.newTask->topOfStack);
 
     /*
      * make sure LOS_CurTaskIDGet and LOS_CurTaskNameGet returns the ID and name of which task
      * that occurred stack overflow exception in OsDoExcHook temporary.
      */
     tmp = g_losTask.runTask;
     g_losTask.runTask = g_losTask.newTask;
     OsDoExcHook(EXC_STACKOVERFLOW);
     g_losTask.runTask = tmp;
 }
 #else
 LITE_OS_SEC_TEXT STATIC VOID OsTaskStackProtect(VOID)
 {
     MPU_CFG_PARA mpuAttr = {0};
     STATIC INT32 id = -1;
 
     if (id == -1) {
         id = ArchMpuUnusedRegionGet();
         if (id < 0) {
             PRINT_ERR("%s %d, get unused id failed!\n", __FUNCTION__, __LINE__);
             return;
         }
     }
 
     mpuAttr.baseAddr = g_losTask.newTask->topOfStack - OS_TASK_STACK_PROTECT_SIZE;
     mpuAttr.size = OS_TASK_STACK_PROTECT_SIZE;
     mpuAttr.memType = MPU_MEM_ON_CHIP_RAM;
     mpuAttr.executable = MPU_NON_EXECUTABLE;
     mpuAttr.shareability = MPU_NO_SHARE;
     mpuAttr.permission = MPU_RO_BY_PRIVILEGED_ONLY;
 
     ArchMpuDisable();
     (VOID)ArchMpuDisableRegion(id);
     (VOID)ArchMpuSetRegion(id, &mpuAttr);
     ArchMpuEnable(1);
 }
 #endif
 #endif
 
 /*****************************************************************************
  Function    : OsTaskSwitchCheck
  Description : Check task switch
  Input       : Node
  Output      : None
  Return      : None
  *****************************************************************************/
 #if (LOSCFG_BASE_CORE_TSK_MONITOR == 1)
 LITE_OS_SEC_TEXT VOID OsTaskSwitchCheck(VOID)
 {
     UINT32 intSave = LOS_IntLock();
 #if (LOSCFG_EXC_HARDWARE_STACK_PROTECTION == 0)
     UINT32 endOfStack = g_losTask.newTask->topOfStack + g_losTask.newTask->stackSize;
 
     if ((*(UINT32 *)(UINTPTR)(g_losTask.runTask->topOfStack)) != OS_TASK_MAGIC_WORD) {
         OsHandleRunTaskStackOverflow();
     }
     if (((UINT32)(UINTPTR)(g_losTask.newTask->stackPointer) <= (g_losTask.newTask->topOfStack)) ||
         ((UINT32)(UINTPTR)(g_losTask.newTask->stackPointer) > endOfStack)) {
         OsHandleNewTaskStackOverflow();
     }
 #else
     OsTaskStackProtect();
 #endif
 
 #if (LOSCFG_BASE_CORE_EXC_TSK_SWITCH == 1)
     /* record task switch info */
     g_taskSwitchInfo.pid[g_taskSwitchInfo.idx] = (UINT16)(g_losTask.newTask->taskID);
 
     errno_t ret = memcpy_s(g_taskSwitchInfo.name[g_taskSwitchInfo.idx], LOS_TASK_NAMELEN,
                            g_losTask.newTask->taskName, LOS_TASK_NAMELEN);
     if (ret != EOK) {
         PRINT_ERR("exc task switch copy file name failed!\n");
     }
     g_taskSwitchInfo.name[g_taskSwitchInfo.idx][LOS_TASK_NAMELEN - 1] = '\0';
 
     if (++g_taskSwitchInfo.idx == OS_TASK_SWITCH_INFO_COUNT) {
         g_taskSwitchInfo.idx = 0;
         g_taskSwitchInfo.cntInfo.isFull = TRUE;
     }
 #endif
 
     LOSCFG_BASE_CORE_TSK_SWITCH_HOOK();
 
 #if (LOSCFG_BASE_CORE_CPUP == 1)
     OsTskCycleEndStart();
 #endif /* LOSCFG_BASE_CORE_CPUP */
     LOS_IntRestore(intSave);
 }
 
 LITE_OS_SEC_TEXT_MINOR VOID OsTaskMonInit(VOID)
 {
 #if (LOSCFG_BASE_CORE_EXC_TSK_SWITCH == 1)
     // Ignore the return code when matching CSEC rule 6.6(4).
     (VOID)memset_s(&g_taskSwitchInfo, sizeof(TaskSwitchInfo), 0, sizeof(TaskSwitchInfo));
     g_taskSwitchInfo.cntInfo.maxCnt = OS_TASK_SWITCH_INFO_COUNT;
 #endif
     return;
 }
 #endif
 