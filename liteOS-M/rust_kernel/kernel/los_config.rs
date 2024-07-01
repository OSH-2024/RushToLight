pub const LOSCFG_SYS_HEAP_SIZE: u32 = 65536;
/*
    los_config.h
    #ifndef LOSCFG_SYS_HEAP_SIZE
    #define LOSCFG_SYS_HEAP_SIZE                                0x10000UL
    #endif
*/
pub const LOSCFG_TASK_MEM_USED: u32 = 0;
/*
    los_config.h
    #ifndef LOSCFG_TASK_MEM_USED
    #define LOSCFG_TASK_MEM_USED                                 0
    #endif
*/
pub const LOSCFG_MEM_FREE_BY_TASKID: u32 = 0;
/*
    los_config.h
    #ifndef LOSCFG_MEM_FREE_BY_TASKID
    #define LOSCFG_MEM_FREE_BY_TASKID                           0
    #endif
*/
pub const LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK: u32 = 0;
/*
    los_config.h
    #ifndef LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK
    #define LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK                0
    #endif
*/
pub const LOSCFG_MEM_MUL_REGIONS:u32 = 0;
/*
    los_config.h
    #ifndef LOSCFG_MEM_MUL_REGIONS
    #define LOSCFG_MEM_MUL_REGIONS                              0
    #endif
*/
pub const LOS_OK: u32 = 0;
/*
    los_compiler.h
    #ifndef LOS_OK
    #define LOS_OK        0U
    #endif
*/
pub const OS_FAIL: u32 = 1;
/*
    los_compiler.h
    #define OS_FAIL       1
*/
pub const OS_64BIT_MAX: i32 = -1;
/*
    los_compiler.h
    #define OS_64BIT_MAX  (0xFFFFFFFFFFFFFFFFULL)
*/
pub const LOSCFG_MEM_MUL_POOL: u32 = 1;
/*
    los_config.h
    #ifndef LOSCFG_MEM_MUL_POOL
    #define LOSCFG_MEM_MUL_POOL                                 1
    #endif
*/
pub const LOSCFG_BASE_CORE_TSK_LIMIT: u32 = 5;
/*
    los_config.h
    #ifndef LOSCFG_BASE_CORE_TSK_LIMIT
    #define LOSCFG_BASE_CORE_TSK_LIMIT                          5
    #endif
*/
pub const OS_MEM_EXPAND_ENABLE: u32 = 0;
/*
    los_memory.c
    #define OS_MEM_EXPAND_ENABLE    0
*/
pub const LOSCFG_MEM_RECORD_LR_CNT: u32 = 3;
/*
    los_config.h
    #ifndef LOSCFG_MEM_RECORD_LR_CNT
    #define LOSCFG_MEM_RECORD_LR_CNT                            3
    #endif
*/

pub const LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM: u32 = 1024
/*
    los_config.h
    #ifndef LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM
    #define LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM                 1024
    #endif
*/
