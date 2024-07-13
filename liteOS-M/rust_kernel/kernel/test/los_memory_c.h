#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

#define EOK 0

#define OS_MEM_NODE_MAGIC 2882395322

#define OS_LITTLE_ENDIAN 4660

#define OS_BIG_ENDIAN 17185

#define OS_BYTE_ORDER 4660

#define LOS_OK 0

#define OS_FAIL 1

#define OS_64BIT_MAX -1

#define __LIBATOMIC_N_LOCKS 16

#define OS_SYS_CLOCK 1000000

#define LOSCFG_BASE_CORE_TICK_PER_SECOND 100

#define LOSCFG_BASE_CORE_TICK_PER_SECOND_MINI 1000

#define LOSCFG_BASE_CORE_TICK_PERIOD_MS 10

#define LOSCFG_BASE_CORE_TICK_WTIMER 0

#define LOSCFG_BASE_CORE_TICK_RESPONSE_MAX 0

#define LOSCFG_PLATFORM_HWI 1

#define LOSCFG_USE_SYSTEM_DEFINED_INTERRUPT 1

#define LOSCFG_PLATFORM_HWI_LIMIT 32

#define LOSCFG_BASE_CORE_TSK_DEFAULT_PRIO 10

#define LOSCFG_BASE_CORE_TSK_LIMIT 5

#define LOSCFG_BASE_CORE_TSK_IDLE_STACK_SIZE 384

#define LOSCFG_BASE_CORE_TSK_DEFAULT_STACK_SIZE 1024

#define LOSCFG_BASE_CORE_TIMESLICE 1

#define LOSCFG_BASE_CORE_TIMESLICE_TIMEOUT 20000

#define LOSCFG_BASE_CORE_TSK_MONITOR 0

#define LOSCFG_BASE_CORE_EXC_TSK_SWITCH 0

#define LOS_TASK_PRIORITY_HIGHEST 0

#define LOS_TASK_PRIORITY_LOWEST 31

#define LOSCFG_BASE_CORE_TASKSTACK_INDEPENDENT 0

#define LOSCFG_STACK_POINT_ALIGN_SIZE 8

#define LOSCFG_BASE_IPC_SEM 1

#define LOSCFG_BASE_IPC_SEM_LIMIT 6

#define OS_SEM_COUNTING_MAX_COUNT 65535

#define LOSCFG_BASE_IPC_MUX 1

#define LOSCFG_BASE_IPC_MUX_LIMIT 6

#define LOSCFG_BASE_IPC_QUEUE 1

#define LOSCFG_BASE_IPC_QUEUE_LIMIT 6

#define LOSCFG_BASE_IPC_STATIC_QUEUE_LIMIT 3

#define LOSCFG_BASE_CORE_SWTMR 1

#define LOSCFG_BASE_CORE_SWTMR_LIMIT 5

#define LOSCFG_BASE_CORE_TSK_SWTMR_STACK_SIZE 1024

#define LOSCFG_BASE_CORE_SWTMR_ALIGN 0

#define OS_SWTMR_HANDLE_QUEUE_SIZE 5

#define LOS_COMMON_DIVISOR 10

#define LOSCFG_SYS_EXTERNAL_HEAP 0

#define LOSCFG_SYS_HEAP_SIZE 65536

#define LOSCFG_MEM_MUL_POOL 1

#define LOSCFG_MEM_FREE_BY_TASKID 0

#define LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK 0

#define LOSCFG_MEM_OMIT_LR_CNT 4

#define LOSCFG_MEM_RECORD_LR_CNT 3

#define LOSCFG_MEM_LEAKCHECK_RECORD_MAX_NUM 1024

#define LOSCFG_MEM_WATERLINE 1

#define OS_SYS_MEM_NUM 20

#define OS_SYS_NOCACHEMEM_SIZE 0

#define LOSCFG_MEM_MUL_REGIONS 0

#define LOSCFG_EXC_HARDWARE_STACK_PROTECTION 0

#define CMSIS_OS_VER 2

#define LOSCFG_KERNEL_PRINTF 1

#define LOSCFG_MPU_ENABLE 0

#define LOSCFG_TASK_MEM_USED 0

#define LOSCFG_PLATFORM_HWI_WITH_ARG 0

#define LOSCFG_ARCH_HWI_VECTOR_ALIGN 256

#define LOSCFG_KERNEL_SIGNAL 0

#define LOSCFG_KERNEL_PM 0

#define LOSCFG_KERNEL_PM_IDLE 0

#define LOSCFG_SHELL_STACK_SIZE 4096

#define LOG_EMG_LEVEL 0

#define LOG_COMMON_LEVEL 1

#define LOG_ERR_LEVEL 2

#define LOG_WARN_LEVEL 3

#define LOG_INFO_LEVEL 4

#define LOG_DEBUG_LEVEL 5

#define PRINT_LEVEL 2

#define OS_MEM_SLI 3

#define OS_MEM_SMALL_BUCKET_COUNT 31

#define OS_MEM_SMALL_BUCKET_MAX_SIZE 128

#define OS_MEM_LARGE_BUCKET_COUNT 24

#define OS_MEM_LARGE_START_BUCKET 7

#define OS_MEM_FREE_LIST_COUNT 223

#define OS_MEM_BITMAP_WORDS 7

#define TICK_CHECK 67108864

#define CYCLE_CHECK 4294967295

#define SHIFT_32_BIT 32

#define MAX_HOUR 24

#define MAX_MINUTES 60

#define MAX_SECONDS 60

#define MILSEC 1000

#define RTC_WAKEUPCLOCK_RTCCLK 32768

#define RTC_WAKEUPCLOCK_RTCCLK_DIV 16

#define RTC_CALIBRATE_SLEEP_TIME 8

#define MACHINE_CYCLE_DEALAY_TIMES 400

#define MAX_SHRINK_PAGECACHE_TRY 10

#define PAGE_SHIFT 10

#define LOS_HOOK_TYPE_MEM_INIT 1

#define LOS_HOOK_TYPE_MEM_ALLOC 2

#define LOS_WAITMODE_AND 4

#define LOS_WAITMODE_OR 2

#define LOS_WAITMODE_CLR 1

#define OS_ERR_MAGIC_WORD 2712847352

#define OS_SYS_MS_PER_SECOND 1000

#define OS_SYS_US_PER_SECOND 1000000

#define OS_SYS_NS_PER_SECOND 1000000000

#define OS_SYS_NS_PER_MS 1000000

#define OS_SYS_NS_PER_US 1000

#define OS_MS_PER_TICK 10

#define OS_US_PER_TICK 10000

#define OS_NS_PER_TICK 10000000

#define OS_SYS_MV_32_BIT 32

#define OS_SYS_US_PER_MS 1000

#define OS_SYS_APPVER_NAME_MAX 64

#define OS_SYS_MAGIC_WORD 2863311530

#define OS_SYS_EMPTY_STACK 3402287818

#define LOS_TASK_ATTR_JOINABLE 2147483648

#define LOS_TASK_NAMELEN 32

#define LOS_NO_WAIT 0

#define LOS_WAIT_FOREVER 4294967295

#define OS_TASK_ERRORID 4294967295

#define OS_TASK_PRIORITY_HIGHEST 0

#define OS_TASK_PRIORITY_LOWEST 31

#define OS_TASK_STATUS_UNUSED 1

#define OS_TASK_STATUS_SUSPEND 2

#define OS_TASK_STATUS_READY 4

#define OS_TASK_STATUS_PEND 8

#define OS_TASK_STATUS_RUNNING 16

#define OS_TASK_STATUS_DELAY 32

#define OS_TASK_STATUS_TIMEOUT 64

#define OS_TASK_STATUS_PEND_TIME 128

#define OS_TASK_STATUS_EXIT 256

#define OS_TASK_FLAG_STACK_FREE 2048

#define OS_TASK_FLAG_SYSTEM_TASK 4096

#define OS_TASK_FLAG_SIGNAL 8192

#define OS_TASK_FLAG_FREEZE 16384

#define OS_TASK_FLAG_JOINABLE 32768

#define OS_TASK_STACK_SIZE_ALIGN 16

#define OS_TASK_STACK_ADDR_ALIGN 8

#define OS_TASK_MAGIC_WORD 3435973836

#define OS_TASK_STACK_INIT 3402287818

#define OS_TSK_PRINUM 32

#define OS_TSK_SORTLINK_LEN 32

#define OS_TSK_HIGH_BITS 5

#define OS_TSK_LOW_BITS 27

#define OS_TSK_MAX_ROLLNUM 4294967263

#define OS_TSK_SORTLINK_LOGLEN 5

#define OS_TSK_SORTLINK_MASK 31

#define OS_TSK_HIGH_BITS_MASK 4160749568

#define OS_TSK_LOW_BITS_MASK -4160749569

#define OS_TASK_SWITCH_INFO_COUNT 10

#define OS_MUX_UNUSED 0

#define OS_MUX_USED 1

#define OS_WAIT_TASK_ARRAY_LEN 1

#define OS_WAIT_TASK_ARRAY_ELEMENT_MASK 31

#define OS_READWRITE_LEN 2

#define OS_QUEUE_UNUSED 0

#define OS_QUEUE_INUSED 1

#define OS_QUEUE_WAIT_FOR_POOL 1

#define OS_QUEUE_NORMAL 0

#define OS_ALL_IPC_QUEUE_LIMIT 6

#define OS_SEM_BINARY_MAX_COUNT 1

#define OS_SEM_UNUSED 0

#define OS_SEM_USED 1

#define COMMON_ERRMODE 3

#define FREE_ERRORMODE 2

#define STORE_ERRMODE 1

#define LOAD_ERRMODE 0

#define LMS_SHADOW_BITS_PER_CELL 2

#define LMS_MEM_BYTES_PER_SHADOW_CELL 4

#define LMS_SHADOW_U8_CELL_NUM 4

#define LMS_SHADOW_U8_REFER_BYTES 16

#define LMS_SHADOW_ACCESSIBLE 0

#define LMS_SHADOW_AFTERFREE 3

#define LMS_SHADOW_REDZONE 2

#define LMS_SHADOW_PAINT 1

#define LMS_SHADOW_MASK 3

#define LMS_SHADOW_ACCESSIBLE_U8 0

#define LMS_SHADOW_AFTERFREE_U8 255

#define LMS_SHADOW_REDZONE_U8 170

#define LMS_SHADOW_MASK_U8 255

#define LMS_SHADOW_PAINT_U8 85

#define MEM_REGION_SIZE_1 1

#define MEM_REGION_SIZE_2 2

#define MEM_REGION_SIZE_4 4

#define MEM_REGION_SIZE_8 8

#define MEM_REGION_SIZE_16 16

typedef unsigned int EXC_TYPE;

typedef void (*ExcHookFn)(EXC_TYPE excType);

typedef int INT32;

typedef unsigned int UINTPTR;

typedef unsigned int UINT32;

typedef void (*BACK_TRACE_HOOK)(UINTPTR *LR, UINT32 LRSize, UINT32 jumpCount, UINTPTR SP);

/**
 * @ingroup los_memory
 * Memory pool extern information structure
 */
typedef struct LOS_MEM_POOL_STATUS {
  UINT32 totalUsedSize;
  UINT32 totalFreeSize;
  UINT32 maxFreeNodeSize;
  UINT32 usedNodeNum;
  UINT32 freeNodeNum;
  UINT32 usageWaterLine;
} LOS_MEM_POOL_STATUS;

typedef UINT32 HWI_HANDLE_T;

typedef char CHAR;

typedef struct tagIrqParam {
  /**
   *< The interrupt number
   */
  int swIrq;
  /**
   *< The pointer to the device ID that launches the interrupt
   */
  void *pDevId;
  /**
   *< The interrupt name
   */
  const CHAR *pName;
} tagIrqParam;

typedef struct tagIrqParam HwiIrqParam;

typedef unsigned short UINT16;

typedef UINT16 HWI_PRIOR_T;

typedef UINT16 HWI_MODE_T;

typedef void (*HWI_PROC_FUNC)(void);

typedef unsigned char UINT8;

typedef struct HwiControllerOps {
  UINT32 (*triggerIrq)(HWI_HANDLE_T hwiNum);
  UINT32 (*clearIrq)(HWI_HANDLE_T hwiNum);
  UINT32 (*enableIrq)(HWI_HANDLE_T hwiNum);
  UINT32 (*disableIrq)(HWI_HANDLE_T hwiNum);
  UINT32 (*setIrqPriority)(HWI_HANDLE_T hwiNum, UINT8 priority);
  UINT32 (*getCurIrqNum)(void);
  UINT32 (*createIrq)(HWI_HANDLE_T hwiNum, HWI_PRIOR_T hwiPrio);
} HwiControllerOps;

typedef unsigned long long UINT64;

typedef struct ArchTickTimer {
  UINT32 freq;
  INT32 irqNum;
  UINT64 periodMax;
  UINT32 (*init)(HWI_PROC_FUNC tickHandler);
  UINT64 (*getCycle)(UINT32 *period);
  UINT64 (*reload)(UINT64 time);
  void (*lock)(void);
  void (*unlock)(void);
  HWI_PROC_FUNC tickHandler;
} ArchTickTimer;

/**
 * @ingroup los_list
 * Structure of a node in a doubly linked list.
 */
typedef struct LOS_DL_LIST {
  /**
   *< Current node's pointer to the previous node
   */
  struct LOS_DL_LIST *pstPrev;
  /**
   *< Current node's pointer to the next node
   */
  struct LOS_DL_LIST *pstNext;
} LOS_DL_LIST;

/**
 * @ingroup los_event
 * Event control structure
 */
typedef struct tagEvent {
  /**
   *< Event mask in the event control block,
   *indicating the event that has been logically processed.
   */
  UINT32 uwEventID;
  /**
   *< Event control block linked list
   */
  struct LOS_DL_LIST stEventList;
} tagEvent;

/**
 * @ingroup los_event
 * Event control structure
 */
typedef struct tagEvent *PEVENT_CB_S;

/**
 * @ingroup los_tick
 * @brief Adjust the system tick timer clock frequency function hooks.
 *
 * @par Description:
 * This API is used to adjust the system tick timer clock frequency.
 * @attention
 * <ul>
 * <li>None</li>
 * </ul>
 *
 * @param  param  [IN] Function parameters.
 *
 * @retval              0: Adjust the system tick timer clock frequency failed.
 * @retval more than zero: Adjust after the system tick timer clock frequency.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
typedef UINT32 (*SYS_TICK_FREQ_ADJUST_FUNC)(UINTPTR param);

/**
 * @ingroup los_tick
 * Define the CPU Tick structure.
 */
typedef struct TagCpuTick {
  UINT32 cntHi;
  UINT32 cntLo;
} TagCpuTick;

/**
 * @ingroup los_tick
 * Define the CPU Tick structure.
 */
typedef struct TagCpuTick CpuTick;

typedef struct SortLinkAttribute {
  struct LOS_DL_LIST sortLink;
} SortLinkAttribute;

typedef unsigned int SortLinkType;

typedef struct SortLinkList {
  struct LOS_DL_LIST sortLinkNode;
  UINT64 responseTime;
} SortLinkList;

/**
 * @ingroup los_task
 * Define the type of the task entry function.
 *
 */
typedef void *(*TSK_ENTRY_FUNC)(UINT32 arg);

/**
 * @ingroup los_task
 * Define the structure of the parameters used for task creation.
 *
 * Information of specified parameters passed in during task creation.
 */
typedef struct tagTskInitParam {
  /**
   *< Task entrance function
   */
  TSK_ENTRY_FUNC pfnTaskEntry;
  /**
   *< Task priority
   */
  UINT16 usTaskPrio;
  /**
   *< Task parameters
   */
  UINT32 uwArg;
  /**
   *< Task stack memory
   */
  UINTPTR stackAddr;
  /**
   *< Task stack size
   */
  UINT32 uwStackSize;
  /**
   *< Task name
   */
  CHAR *pcName;
  /**
   *< Reserved
   */
  UINT32 uwResved;
} tagTskInitParam;

/**
 * @ingroup los_task
 * Define the structure of the parameters used for task creation.
 *
 * Information of specified parameters passed in during task creation.
 */
typedef struct tagTskInitParam TSK_INIT_PARAM_S;

/**
 * @ingroup los_event
 * Event control structure
 */
typedef struct tagEvent EVENT_CB_S;

typedef unsigned int BOOL;

/**
 * @ingroup los_task
 * Task information structure.
 */
typedef struct tagTskInfo {
  /**
   *< Task entrance function
   */
  CHAR acName[32];
  /**
   *< Task ID
   */
  UINT32 uwTaskID;
  /**
   *< Task status
   */
  UINT16 usTaskStatus;
  /**
   *< Task priority
   */
  UINT16 usTaskPrio;
  /**
   *< Semaphore pointer
   */
  void *pTaskSem;
  /**
   *< Mutex pointer
   */
  void *pTaskMux;
  /**
   *< Sem ID
   */
  UINT32 uwSemID;
  /**
   *< Mux ID
   */
  UINT32 uwMuxID;
  /**
   *< Event
   */
  EVENT_CB_S uwEvent;
  /**
   *< Event mask
   */
  UINT32 uwEventMask;
  /**
   *< Task stack size
   */
  UINT32 uwStackSize;
  /**
   *< Task stack top
   */
  UINT32 uwTopOfStack;
  /**
   *< Task stack bottom
   */
  UINT32 uwBottomOfStack;
  /**
   *< Task SP pointer
   */
  UINT32 uwSP;
  /**
   *< Current task stack usage
   */
  UINT32 uwCurrUsed;
  /**
   *< Task stack usage peak
   */
  UINT32 uwPeakUsed;
  /**
   *< Flag that indicates whether a task stack overflow occurs
   */
  BOOL bOvf;
} tagTskInfo;

/**
 * @ingroup los_task
 * Task information structure.
 */
typedef struct tagTskInfo TSK_INFO_S;

/**
 * @ingroup los_queue
 * Structure of the block for queue information query
 */
typedef struct tagQueueInfo {
  /**
   *< Queue ID
   */
  UINT32 queueID;
  /**
   *< Queue length
   */
  UINT16 queueLen;
  /**
   *< Node size
   */
  UINT16 queueSize;
  /**
   *< Node head
   */
  UINT16 queueHead;
  /**
   *< Node tail
   */
  UINT16 queueTail;
  /**
   *< Count of writable resources
   */
  UINT16 writableCnt;
  /**
   *< Count of readable resources
   */
  UINT16 readableCnt;
  /**
   *< Resource reading task
   */
  UINT32 waitReadTask[1];
  /**
   *< Resource writing task
   */
  UINT32 waitWriteTask[1];
  /**
   *< Memory task
   */
  UINT32 waitMemTask[1];
} tagQueueInfo;

/**
 * @ingroup los_queue
 * Structure of the block for queue information query
 */
typedef struct tagQueueInfo QUEUE_INFO_S;

/**
 * @ingroup los_queue
 * Queue information block structure
 */
typedef struct LosQueueCB {
  /**
   *< Pointer to a queue handle
   */
  UINT8 *queue;
  /**
   *< Queue name
   */
  UINT8 *queueName;
  /**
   *< Queue state
   */
  UINT16 queueState;
  /**
   *< Queue length
   */
  UINT16 queueLen;
  /**
   *< Node size
   */
  UINT16 queueSize;
  /**
   *< queueID
   */
  UINT16 queueID;
  /**
   *< Node head
   */
  UINT16 queueHead;
  /**
   *< Node tail
   */
  UINT16 queueTail;
  /**
   *< Count of readable or writable resources, 0:readable, 1:writable
   */
  UINT16 readWriteableCnt[2];
  /**
   *< Pointer to the linked list to be read or written,
   *0:readlist, 1:writelist
   */
  struct LOS_DL_LIST readWriteList[2];
  /**
   *< Pointer to the memory linked list
   */
  struct LOS_DL_LIST memList;
} LosQueueCB;

/**
 * @ingroup  los_swtmr
 * @brief Define the type of a callback function that handles software timer timeout.
 *
 * @par Description:
 * This API is used to define the type of a callback function that handles software timer timeout, so that it can be
 * called when software timer timeout.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  para     [IN] the parameter of the callback function that handles software timer timeout.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_swtmr.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
typedef void (*SWTMR_PROC_FUNC)(UINT32 para);

typedef struct LosLmkOpsNode {
  /**
   *< The priority in the LMK list, the higher priority with a smaller number.
   */
  UINT32 priority;
  /**
   *< Release the memory of tasks in the LMK list. Return LOS_OK for a successful release.
   */
  UINT32 (*freeMem)(void);
  /**
   *< Restore the tasks killed by freeMem(). Return LOS_OK for a successful restore.
   */
  UINT32 (*restoreTask)(void);
  /**
   *< LosLmkOpsNode node.
   */
  struct LOS_DL_LIST node;
} LosLmkOpsNode;

typedef unsigned int LogModuleType;

typedef unsigned int LOS_MODULE_ID;

typedef unsigned int QueueReadWrite;

typedef unsigned int QueueHeadTail;

typedef unsigned int QueuePointOrNot;

/**
 * @ingroup los_swtmr
 * Software timer mode
 */
typedef unsigned int EnSwTmrType;

/**
 * @ingroup los_swtmr
 * Software timer state
 */
typedef unsigned int SwtmrState;

#define EXC_TYPE_EXC_REBOOT 0

#define EXC_TYPE_EXC_ASSERT 1

#define EXC_TYPE_EXC_PANIC 2

#define EXC_TYPE_EXC_STACKOVERFLOW 3

#define EXC_TYPE_EXC_INTERRUPT 4

#define EXC_TYPE_EXC_TYPE_END 5

#define LogModuleType_LOG_MODULE_KERNEL 0

#define LogModuleType_LOG_MODULE_FS 1

#define LogModuleType_LOS_MODULE_OTHERS 2

#define LOS_MODULE_ID_LOS_MOD_SYS 0

#define LOS_MODULE_ID_LOS_MOD_MEM 1

#define LOS_MODULE_ID_LOS_MOD_TSK 2

#define LOS_MODULE_ID_LOS_MOD_SWTMR 3

#define LOS_MODULE_ID_LOS_MOD_TICK 4

#define LOS_MODULE_ID_LOS_MOD_MSG 5

#define LOS_MODULE_ID_LOS_MOD_QUE 6

#define LOS_MODULE_ID_LOS_MOD_SEM 7

#define LOS_MODULE_ID_LOS_MOD_MBOX 8

#define LOS_MODULE_ID_LOS_MOD_HWI 9

#define LOS_MODULE_ID_LOS_MOD_HWWDG 10

#define LOS_MODULE_ID_LOS_MOD_CACHE 11

#define LOS_MODULE_ID_LOS_MOD_HWTMR 12

#define LOS_MODULE_ID_LOS_MOD_MMU 13

#define LOS_MODULE_ID_LOS_MOD_LOG 14

#define LOS_MODULE_ID_LOS_MOD_ERR 15

#define LOS_MODULE_ID_LOS_MOD_EXC 16

#define LOS_MODULE_ID_LOS_MOD_CSTK 17

#define LOS_MODULE_ID_LOS_MOD_MPU 18

#define LOS_MODULE_ID_LOS_MOD_NMHWI 19

#define LOS_MODULE_ID_LOS_MOD_TRACE 20

#define LOS_MODULE_ID_LOS_MOD_IPC 24

#define LOS_MODULE_ID_LOS_MOD_TIMER 26

#define LOS_MODULE_ID_LOS_MOD_EVENT 28

#define LOS_MODULE_ID_LOS_MOD_MUX 29

#define LOS_MODULE_ID_LOS_MOD_CPUP 30

#define LOS_MODULE_ID_LOS_MOD_HOOK 31

#define LOS_MODULE_ID_LOS_MOD_PM 32

#define LOS_MODULE_ID_LOS_MOD_LMK 33

#define LOS_MODULE_ID_LOS_MOD_SHELL 49

#define LOS_MODULE_ID_LOS_MOD_SIGNAL 50

#define LOS_MODULE_ID_LOS_MOD_BUTT 51

#define SortLinkType_OS_SORT_LINK_TASK 1

#define SortLinkType_OS_SORT_LINK_SWTMR 2

#define QueueReadWrite_OS_QUEUE_READ 0

#define QueueReadWrite_OS_QUEUE_WRITE 1

#define QueueHeadTail_OS_QUEUE_HEAD 0

#define QueueHeadTail_OS_QUEUE_TAIL 1

#define QueuePointOrNot_OS_QUEUE_NOT_POINT 0

#define QueuePointOrNot_OS_QUEUE_POINT 1

#define EnSwTmrType_LOS_SWTMR_MODE_ONCE 0

#define EnSwTmrType_LOS_SWTMR_MODE_PERIOD 1

#define EnSwTmrType_LOS_SWTMR_MODE_NO_SELFDELETE 2

#define EnSwTmrType_LOS_SWTMR_MODE_OPP 3

/**
 *< The software timer is not used.
 */
#define SwtmrState_OS_SWTMR_STATUS_UNUSED 0

/**
 *< The software timer is created.
 */
#define SwtmrState_OS_SWTMR_STATUS_CREATED 1

/**
 *< The software timer is timing.
 */
#define SwtmrState_OS_SWTMR_STATUS_TICKING 2

extern void *memcpy(void *dest, const void *src, size_t n);

extern void OsExcHookRegister(ExcHookFn excHookFn);

extern void OsDoExcHook(EXC_TYPE excType);

extern int printf(const char *fmt);

extern INT32 OsLogLevelCheck(INT32 level);

extern void OsBackTraceHookSet(BACK_TRACE_HOOK hook);

extern void OsBackTraceHookCall(UINTPTR *LR, UINT32 LRSize, UINT32 jumpCount, UINTPTR SP);

/**
 * @ingroup los_memory
 * @brief Deinitialize dynamic memory.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to deinitialize the dynamic memory of a doubly linked list.</li>
 * </ul>
 *
 * @param pool          [IN] Starting address of memory.
 *
 * @retval #OS_ERROR   The dynamic memory fails to be deinitialized.
 * @retval #LOS_OK     The dynamic memory is successfully deinitialized.
 * @par Dependency:
 * <ul>
 * <li>los_memory.h: the header file that contains the API declaration.</li>
 * </ul>
 * @see None.
 */
extern UINT32 LOS_MemDeInit(void *pool);

/**
 * @ingroup los_memory
 * @brief Print information about all pools.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to print information about all pools.</li>
 * </ul>
 *
 * @retval #UINT32   The pool number.
 * @par Dependency:
 * <ul>
 * <li>los_memory.h: the header file that contains the API declaration.</li>
 * </ul>
 * @see None.
 */
extern UINT32 LOS_MemPoolList(void);

/**
 * @ingroup los_memory
 * @brief Initialize dynamic memory.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to initialize the dynamic memory of a doubly linked list.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The size parameter value should match the following two conditions :
 * 1) Be less than or equal to the Memory pool size;
 * 2) Be greater than the size of OS_MEM_MIN_POOL_SIZE.</li>
 * <li>Call this API when dynamic memory needs to be initialized during the startup of Huawei LiteOS.</li>
 * <li>The parameter input must be four byte-aligned.</li>
 * <li>The init area [pool, pool + size] should not conflict with other pools.</li>
 * </ul>
 *
 * @param pool         [IN] Starting address of memory.
 * @param size         [IN] Memory size.
 *
 * @retval #OS_ERROR   The dynamic memory fails to be initialized.
 * @retval #LOS_OK     The dynamic memory is successfully initialized.
 * @par Dependency:
 * <ul>
 * <li>los_memory.h: the header file that contains the API declaration.</li>
 * </ul>
 * @see None.
 */
extern UINT32 LOS_MemInit(void *pool,
                          UINT32 size);

/**
 * @ingroup los_memory
 * @brief Allocate dynamic memory.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to allocate a memory block of which the size is specified.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The input pool parameter must be initialized via func LOS_MemInit.</li>
 * <li>The size of the input parameter size can not be greater than the memory pool size that specified at the second
 * input parameter of LOS_MemInit.</li>
 * <li>The size of the input parameter size must be four byte-aligned.</li>
 * </ul>
 *
 * @param  pool    [IN] Pointer to the memory pool that contains the memory block to be allocated.
 * @param  size    [IN] Size of the memory block to be allocated (unit: byte).
 *
 * @retval #NULL          The memory fails to be allocated.
 * @retval #VOID*         The memory is successfully allocated with the starting address of the allocated memory block
 *                        returned.
 * @par Dependency:
 * <ul><li>los_memory.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MemRealloc | LOS_MemAllocAlign | LOS_MemFree
 */
extern void *LOS_MemAlloc(void *pool,
                          UINT32 size);

/**
 * @ingroup los_memory
 * @brief Free dynamic memory.
 *
 * @par Description:
 * <li>This API is used to free specified dynamic memory that has been allocated.</li>
 * @attention
 * <ul>
 * <li>The input pool parameter must be initialized via func LOS_MemInit.</li>
 * <li>The input ptr parameter must be allocated by LOS_MemAlloc or LOS_MemAllocAlign or LOS_MemRealloc.</li>
 * </ul>
 *
 * @param  pool  [IN] Pointer to the memory pool that contains the dynamic memory block to be freed.
 * @param  ptr   [IN] Starting address of the memory block to be freed.
 *
 * @retval #LOS_NOK          The memory block fails to be freed because the starting address of the memory block is
 *                           invalid, or the memory overwriting occurs.
 * @retval #LOS_OK           The memory block is successfully freed.
 * @par Dependency:
 * <ul><li>los_memory.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MemAlloc | LOS_MemRealloc | LOS_MemAllocAlign
 */
extern UINT32 LOS_MemFree(void *pool,
                          void *ptr);

/**
 * @ingroup los_memory
 * @brief Re-allocate a memory block.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to allocate a new memory block of which the size is specified by size if the original memory
 * block size is insufficient. The new memory block will copy the data in the original memory block of which the
 * address is specified by ptr. The size of the new memory block determines the maximum size of data to be copied.
 * After the new memory block is created, the original one is freed.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The input pool parameter must be initialized via func LOS_MemInit.</li>
 * <li>The input ptr parameter must be allocated by LOS_MemAlloc or LOS_MemAllocAlign.</li>
 * <li>The size of the input parameter size can not be greater than the memory pool size that specified at the second
 * input parameter of LOS_MemInit.</li>
 * <li>The size of the input parameter size must be aligned as follows: 1) if the ptr is allocated by LOS_MemAlloc,
 * it must be four byte-aligned; 2) if the ptr is allocated by LOS_MemAllocAlign, it must be aligned with the size of
 * the input parameter boundary of LOS_MemAllocAlign.</li>
 * </ul>
 *
 * @param  pool     [IN] Pointer to the memory pool that contains the original and new memory blocks.
 * @param  ptr      [IN] Address of the original memory block.
 * @param  size     [IN] Size of the new memory block.
 *
 * @retval #NULL    The memory fails to be re-allocated.
 * @retval #VOID*   The memory is successfully re-allocated with the starting address of the new memory block returned.
 * @par Dependency:
 * <ul><li>los_memory.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MemAlloc | LOS_MemAllocAlign | LOS_MemFree
 */
extern void *LOS_MemRealloc(void *pool,
                            void *ptr,
                            UINT32 size);

/**
 * @ingroup los_memory
 * @brief Allocate aligned memory.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to allocate memory blocks of specified size and of which the starting addresses are aligned on
 * a specified boundary.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The input pool parameter must be initialized via func LOS_MemInit.</li>
 * <li>The size of the input parameter size can not be greater than the memory pool size that specified at the second
 * input parameter of LOS_MemInit.</li>
 * <li>The alignment parameter value must be a power of 2 with the minimum value being 4.</li>
 * </ul>
 *
 * @param  pool      [IN] Pointer to the memory pool that contains the memory blocks to be allocated.
 * @param  size      [IN] Size of the memory to be allocated.
 * @param  boundary  [IN] Boundary on which the memory is aligned.
 *
 * @retval #NULL    The memory fails to be allocated.
 * @retval #VOID*   The memory is successfully allocated with the starting address of the allocated memory returned.
 * @par Dependency:
 * <ul><li>los_memory.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MemAlloc | LOS_MemRealloc | LOS_MemFree
 */
extern void *LOS_MemAllocAlign(void *pool,
                               UINT32 size,
                               UINT32 boundary);

/**
 * @ingroup los_memory
 * @brief Get the size of memory pool's size.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to get the size of memory pool' total size.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The input pool parameter must be initialized via func LOS_MemInit.</li>
 * </ul>
 *
 * @param  pool           [IN] A pointer pointed to the memory pool.
 *
 * @retval #LOS_NOK        The incoming parameter pool is NULL.
 * @retval #UINT32         The size of the memory pool.
 * @par Dependency:
 * <ul><li>los_memory.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 LOS_MemPoolSizeGet(const void *pool);

/**
 * @ingroup los_memory
 * @brief Get the size of memory totally used.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to get the size of memory totally used in memory pool.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The input pool parameter must be initialized via func LOS_MemInit.</li>
 * </ul>
 *
 * @param  pool           [IN] A pointer pointed to the memory pool.
 *
 * @retval #LOS_NOK        The incoming parameter pool is NULL.
 * @retval #UINT32         The size of the memory pool used.
 * @par Dependency:
 * <ul><li>los_memory.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 LOS_MemTotalUsedGet(void *pool);

/**
 * @ingroup los_memory
 * @brief Get the information of memory pool.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to get the information of memory pool.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The input pool parameter must be initialized via func LOS_MemInit.</li>
 * </ul>
 *
 * @param  pool                 [IN] A pointer pointed to the memory pool.
 * @param  poolStatus           [IN] A pointer for storage the pool status
 *
 * @retval #LOS_NOK           The incoming parameter pool is NULL or invalid.
 * @retval #LOS_OK            Success to get memory information.
 * @par Dependency:
 * <ul><li>los_memory.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 LOS_MemInfoGet(void *pool, struct LOS_MEM_POOL_STATUS *poolStatus);

/**
 * @ingroup los_memory
 * @brief Get the number of free node in every size.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to get the number of free node in every size.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The input pool parameter must be initialized via func LOS_MemInit.</li>
 * </ul>
 *
 * @param  pool               [IN] A pointer pointed to the memory pool.
 *
 * @retval #LOS_NOK           The incoming parameter pool is NULL.
 * @retval #UINT32            The address of the last used node that casts to UINT32.
 * @par Dependency:
 * <ul><li>los_memory.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 LOS_MemFreeNodeShow(void *pool);

/**
 * @ingroup los_memory
 * @brief Check the memory pool integrity.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to check the memory pool integrity.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The input pool parameter must be initialized via func LOS_MemInit.</li>
 * <li>LOS_MemIntegrityCheck will be called by malloc function when the macro of LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK
 * is defined in LiteOS.</li>
 * <li>LOS_MemIntegrityCheck function can be called by user anytime.</li>
 * </ul>
 *
 * @param  pool              [IN] A pointer pointed to the memory pool.
 *
 * @retval #LOS_NOK           The memory pool (pool) is impaired.
 * @retval #LOS_OK            The memory pool (pool) is integrated.
 * @par Dependency:
 * <ul><li>los_memory.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 LOS_MemIntegrityCheck(const void *pool);

extern void LOS_MemUnlockEnable(void *pool);

extern UINT32 OsMemSystemInit(void);

extern void OsTaskMemUsed(void *pool, UINT32 *tskMemInfoBuf, UINT32 tskMemInfoCnt);

extern void __stack_chk_fail(void);

/**
 * @ingroup  los_interrupt
 * @brief Delete hardware interrupt.
 *
 * @par Description:
 * This API is used to delete hardware interrupt.
 *
 * @attention
 * <ul>
 * <li>The hardware interrupt module is usable only when the configuration item for hardware
 * interrupt tailoring is enabled.</li>
 * <li>Hardware interrupt number value range: [OS_USER_HWI_MIN,OS_USER_HWI_MAX]. The value range
 * applicable for a Cortex-A7 platform is [32,95].</li>
 * <li>OS_HWI_MAX_NUM specifies the maximum number of interrupts that can be created.</li>
 * <li>Before executing an interrupt on a platform, refer to the chip manual of the platform.</li>
 * </ul>
 *
 * @param  hwiNum   [IN] Type#HWI_HANDLE_T: hardware interrupt number. The value range applicable
 *                       for a Cortex-A7 platform is [32,95].
 * @param  irqParam [IN] Type #HwiIrqParam *. ID of hardware interrupt which will base on
 *                                                when delete the hardware interrupt.
 * @retval #OS_ERRNO_HWI_NUM_INVALID              0x02000900: Invalid interrupt number.
 * @retval #LOS_OK                                0         : The interrupt is successfully delete.
 * @par Dependency:
 * <ul><li>los_interrupt.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 ArchHwiDelete(HWI_HANDLE_T hwiNum, HwiIrqParam *irqParam);

/**
 * @ingroup  los_interrupt
 * @brief Create a hardware interrupt.
 *
 * @par Description:
 * This API is used to configure a hardware interrupt and register a hardware interrupt handling function.
 *
 * @attention
 * <ul>
 * <li>The hardware interrupt module is usable only when the configuration item for hardware
 * interrupt tailoring is enabled.</li>
 * <li>Hardware interrupt number value range: [OS_USER_HWI_MIN,OS_USER_HWI_MAX]. The value range
 * applicable for a Cortex-A7 platform is [32,95].</li>
 * <li>OS_HWI_MAX_NUM specifies the maximum number of interrupts that can be created.</li>
 * <li>Before executing an interrupt on a platform, refer to the chip manual of the platform.</li>
 * </ul>
 *
 * @param  hwiNum   [IN] Type#HWI_HANDLE_T: hardware interrupt number. The value range applicable for a
 *                       Cortex-A7 platform is [32,95].
 * @param  hwiPrio  [IN] Type#HWI_PRIOR_T: hardware interrupt priority. Ignore this parameter temporarily.
 * @param  mode     [IN] Type#HWI_MODE_T: hardware interrupt mode. Ignore this parameter temporarily.
 * @param  handler  [IN] Type#HWI_PROC_FUNC: interrupt handler used when a hardware interrupt is triggered.
 * @param  irqParam [IN] Type#HwiIrqParam: input parameter of the interrupt
 *                                         handler used when a hardware interrupt is triggered.
 *
 * @retval #OS_ERRNO_HWI_PROC_FUNC_NULL               0x02000901: Null hardware interrupt handling function.
 * @retval #OS_ERRNO_HWI_NUM_INVALID                  0x02000900: Invalid interrupt number.
 * @retval #OS_ERRNO_HWI_NO_MEMORY                    0x02000903: Insufficient memory for hardware interrupt creation.
 * @retval #OS_ERRNO_HWI_ALREADY_CREATED              0x02000904: The interrupt handler being created has
 *                                                                already been created.
 * @retval #LOS_OK                                    0         : The interrupt is successfully created.
 * @par Dependency:
 * <ul><li>los_interrupt.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 ArchHwiCreate(HWI_HANDLE_T hwiNum,
                            HWI_PRIOR_T hwiPrio,
                            HWI_MODE_T mode,
                            HWI_PROC_FUNC handler,
                            HwiIrqParam *irqParam);

extern UINT32 ArchIsIntActive(void);

extern UINT32 ArchIntLock(void);

extern UINT32 ArchIntUnLock(void);

extern void ArchIntRestore(UINT32 intSave);

extern UINT32 ArchIntTrigger(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntEnable(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntDisable(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntClear(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntSetPriority(HWI_HANDLE_T hwiNum, HWI_PRIOR_T priority);

extern UINT32 ArchIntCurIrqNum(void);

extern struct HwiControllerOps *ArchIntOpsGet(void);

extern UINT32 ArchEnterSleep(void);

/**
 * @ingroup los_timer
 * @brief Get tick timer control block.
 *
 * @par Description:
 * This API is used to get tick timer control block.
 *
 * @param  None
 *
 * @retval #tick timer control block
 * @par Dependency:
 * <ul><li>los_timer.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern struct ArchTickTimer *ArchSysTickTimerGet(void);

extern void ArchInit(void);

extern void OsExcHookRegister(ExcHookFn excHookFn);

extern void OsDoExcHook(EXC_TYPE excType);

extern int printf(const char *fmt);

extern INT32 OsLogLevelCheck(INT32 level);

extern void OsBackTraceHookSet(BACK_TRACE_HOOK hook);

extern void OsBackTraceHookCall(UINTPTR *LR, UINT32 LRSize, UINT32 jumpCount, UINTPTR SP);

extern void __stack_chk_fail(void);

/**
 * @ingroup  los_interrupt
 * @brief Delete hardware interrupt.
 *
 * @par Description:
 * This API is used to delete hardware interrupt.
 *
 * @attention
 * <ul>
 * <li>The hardware interrupt module is usable only when the configuration item for hardware
 * interrupt tailoring is enabled.</li>
 * <li>Hardware interrupt number value range: [OS_USER_HWI_MIN,OS_USER_HWI_MAX]. The value range
 * applicable for a Cortex-A7 platform is [32,95].</li>
 * <li>OS_HWI_MAX_NUM specifies the maximum number of interrupts that can be created.</li>
 * <li>Before executing an interrupt on a platform, refer to the chip manual of the platform.</li>
 * </ul>
 *
 * @param  hwiNum   [IN] Type#HWI_HANDLE_T: hardware interrupt number. The value range applicable
 *                       for a Cortex-A7 platform is [32,95].
 * @param  irqParam [IN] Type #HwiIrqParam *. ID of hardware interrupt which will base on
 *                                                when delete the hardware interrupt.
 * @retval #OS_ERRNO_HWI_NUM_INVALID              0x02000900: Invalid interrupt number.
 * @retval #LOS_OK                                0         : The interrupt is successfully delete.
 * @par Dependency:
 * <ul><li>los_interrupt.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 ArchHwiDelete(HWI_HANDLE_T hwiNum, HwiIrqParam *irqParam);

/**
 * @ingroup  los_interrupt
 * @brief Create a hardware interrupt.
 *
 * @par Description:
 * This API is used to configure a hardware interrupt and register a hardware interrupt handling function.
 *
 * @attention
 * <ul>
 * <li>The hardware interrupt module is usable only when the configuration item for hardware
 * interrupt tailoring is enabled.</li>
 * <li>Hardware interrupt number value range: [OS_USER_HWI_MIN,OS_USER_HWI_MAX]. The value range
 * applicable for a Cortex-A7 platform is [32,95].</li>
 * <li>OS_HWI_MAX_NUM specifies the maximum number of interrupts that can be created.</li>
 * <li>Before executing an interrupt on a platform, refer to the chip manual of the platform.</li>
 * </ul>
 *
 * @param  hwiNum   [IN] Type#HWI_HANDLE_T: hardware interrupt number. The value range applicable for a
 *                       Cortex-A7 platform is [32,95].
 * @param  hwiPrio  [IN] Type#HWI_PRIOR_T: hardware interrupt priority. Ignore this parameter temporarily.
 * @param  mode     [IN] Type#HWI_MODE_T: hardware interrupt mode. Ignore this parameter temporarily.
 * @param  handler  [IN] Type#HWI_PROC_FUNC: interrupt handler used when a hardware interrupt is triggered.
 * @param  irqParam [IN] Type#HwiIrqParam: input parameter of the interrupt
 *                                         handler used when a hardware interrupt is triggered.
 *
 * @retval #OS_ERRNO_HWI_PROC_FUNC_NULL               0x02000901: Null hardware interrupt handling function.
 * @retval #OS_ERRNO_HWI_NUM_INVALID                  0x02000900: Invalid interrupt number.
 * @retval #OS_ERRNO_HWI_NO_MEMORY                    0x02000903: Insufficient memory for hardware interrupt creation.
 * @retval #OS_ERRNO_HWI_ALREADY_CREATED              0x02000904: The interrupt handler being created has
 *                                                                already been created.
 * @retval #LOS_OK                                    0         : The interrupt is successfully created.
 * @par Dependency:
 * <ul><li>los_interrupt.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 ArchHwiCreate(HWI_HANDLE_T hwiNum,
                            HWI_PRIOR_T hwiPrio,
                            HWI_MODE_T mode,
                            HWI_PROC_FUNC handler,
                            HwiIrqParam *irqParam);

extern UINT32 ArchIsIntActive(void);

extern UINT32 ArchIntLock(void);

extern UINT32 ArchIntUnLock(void);

extern void ArchIntRestore(UINT32 intSave);

extern UINT32 ArchIntTrigger(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntEnable(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntDisable(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntClear(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntSetPriority(HWI_HANDLE_T hwiNum, HWI_PRIOR_T priority);

extern UINT32 ArchIntCurIrqNum(void);

extern struct HwiControllerOps *ArchIntOpsGet(void);

/**
 * @ingroup  los_context
 * @brief: Task stack initialization.
 *
 * @par Description:
 * This API is used to initialize the task stack.
 *
 * @attention:
 * <ul><li>None.</li></ul>
 *
 * @param  taskID     [IN] Type#UINT32: TaskID.
 * @param  stackSize  [IN] Type#UINT32: Total size of the stack.
 * @param  topStack   [IN] Type#VOID *: Top of task's stack.
 *
 * @retval: context Type#TaskContext *.
 * @par Dependency:
 * <ul><li>los_context.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern void *ArchTskStackInit(UINT32 taskID, UINT32 stackSize, void *topStack);

/**
 * @ingroup  los_context
 * @brief: Function to sys exit.
 *
 * @par Description:
 * This API is used to sys exit.
 *
 * @attention:
 * <ul><li>None.</li></ul>
 *
 * @param  None.
 *
 * @retval: None.
 * @par Dependency:
 * <ul><li>los_context.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern void ArchSysExit(void);

/**
 * @ingroup  los_context
 * @brief: Task scheduling Function.
 *
 * @par Description:
 * This API is used to scheduling task.
 *
 * @attention:
 * <ul><li>None.</li></ul>
 *
 * @param  None.
 *
 * @retval: None.
 * @par Dependency:
 * <ul><li>los_context.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern void ArchTaskSchedule(void);

extern UINT32 ArchStartSchedule(void);

extern void *ArchSignalContextInit(void *stackPointer,
                                   void *stackTop,
                                   UINTPTR sigHandler,
                                   UINT32 param);

/**
 * @ingroup los_event
 * @brief Initialize an event control block.
 *
 * @par Description:
 * This API is used to initialize the event control block pointed to by eventCB.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param eventCB [IN/OUT] Pointer to the event control block to be initialized.
 *
 * @retval #LOS_ERRNO_EVENT_PTR_NULL  Null pointer.
 * @retval #LOS_OK                    The event control block is successfully initialized.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventClear
 */
extern UINT32 LOS_EventInit(PEVENT_CB_S eventCB);

/**
 * @ingroup los_event
 * @brief Obtain an event specified by the event ID.
 *
 * @par Description:
 * This API is used to check whether an event expected by the user occurs according to the event ID, event mask,
 * and event reading mode, and process the event based on the event reading mode. The event ID must point to
 * valid memory.
 * @attention
 * <ul>
 * <li>When the mode is LOS_WAITMODE_CLR, the eventID is passed-out.</li>
 * <li>Otherwise the eventID is passed-in.</li>
 * </ul>
 *
 * @param eventID      [IN/OUT] Pointer to the ID of the event to be checked.
 * @param eventMask    [IN] Mask of the event expected to occur by the user, indicating the event obtained after
 * it is logically processed that matches the ID pointed to by mode.
 * @param mode         [IN] Event reading mode. The modes include LOS_WAITMODE_AND, LOS_WAITMODE_OR, LOS_WAITMODE_CLR.
 *
 * @retval 0             The event expected by the user does not occur.
 * @retval #UINT32       The event expected by the user occurs.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventRead | LOS_EventWrite
 */
extern UINT32 LOS_EventPoll(UINT32 *eventID,
                            UINT32 eventMask,
                            UINT32 mode);

/**
 * @ingroup los_event
 * @brief Read an event.
 *
 * @par Description:
 * This API is used to block or schedule a task that reads an event of which the event control block, event mask,
 * reading mode,
 * and timeout information are specified.
 * </ul>
 * @attention
 * <ul>
 * <li>An error code and an event return value can be same. To differentiate the error code and return value, bit 25 of
 * the event mask is forbidden to be used.</li>
 * </ul>
 *
 * @param eventCB      [IN/OUT] Pointer to the event control block to be checked. This parameter must point to
 *                              valid memory.
 * @param eventMask    [IN]     Mask of the event expected to occur by the user, indicating the event obtained after
 *                              it is logically processed that matches the ID pointed to by eventID.
 * @param mode         [IN]     Event reading mode.
 * @param timeOut      [IN]     Timeout interval of event reading (unit: Tick).
 *
 * @retval #LOS_ERRNO_EVENT_SETBIT_INVALID     Bit 25 of the event mask cannot be set because
 *                                             it is set to an error number.
 * @retval #LOS_ERRNO_EVENT_EVENTMASK_INVALID  The passed-in event reading mode is incorrect.
 * @retval #LOS_ERRNO_EVENT_READ_IN_INTERRUPT  The event is being read during an interrupt.
 * @retval #LOS_ERRNO_EVENT_FLAGS_INVALID      The event mode is invalid.
 * @retval #LOS_ERRNO_EVENT_READ_IN_LOCK       The event reading task is locked.
 * @retval #LOS_ERRNO_EVENT_PTR_NULL           The passed-in pointer is null.
 * @retval 0                                   The event expected by the user does not occur.
 * @retval #UINT32                             The event expected by the user occurs.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventPoll | LOS_EventWrite
 */
extern UINT32 LOS_EventRead(PEVENT_CB_S eventCB,
                            UINT32 eventMask,
                            UINT32 mode,
                            UINT32 timeOut);

/**
 * @ingroup los_event
 * @brief Write an event.
 *
 * @par Description:
 * This API is used to write an event specified by the passed-in event mask into an event control block
 * pointed to by eventCB.
 * @attention
 * <ul>
 * <li>To determine whether the LOS_EventRead API returns an event or an error code, bit 25 of the event mask
 * is forbidden to be used.</li>
 * </ul>
 *
 * @param eventCB  [IN/OUT] Pointer to the event control block into which an event is to be written.
 * This parameter must point to valid memory.
 * @param events   [IN] Event mask to be written.
 *
 * @retval #LOS_ERRNO_EVENT_SETBIT_INVALID  Bit 25 of the event mask cannot be set to an event
 * because it is set to an error code.
 * @retval #LOS_ERRNO_EVENT_PTR_NULL        Null pointer.
 * @retval #LOS_OK                          The event is successfully written.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventPoll | LOS_EventRead
 */
extern UINT32 LOS_EventWrite(PEVENT_CB_S eventCB,
                             UINT32 events);

/**
 * @ingroup los_event
 * @brief Clear the event of the eventCB by a specified eventMask.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to set the ID of an event that has a specified mask and of which the information is stored in
 * an event control block pointed to by eventCB to 0. eventCB must point to valid memory.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The value of events needs to be reversed when it is passed-in.</li>
 * </ul>
 *
 * @param eventCB     [IN/OUT] Pointer to the event control block to be cleared.
 * @param eventMask   [IN]     Mask of the event to be cleared.
 *
 * @retval #LOS_ERRNO_EVENT_PTR_NULL  Null pointer.
 * @retval #LOS_OK                    The event is successfully cleared.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventPoll | LOS_EventRead | LOS_EventWrite
 */
extern UINT32 LOS_EventClear(PEVENT_CB_S eventCB,
                             UINT32 eventMask);

/**
 * @ingroup los_event
 * @brief Destroy an event.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to Destroy an event.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The specific event should be a valid one.</li>
 * </ul>
 *
 * @param eventCB     [IN/OUT] Pointer to the event control block to be Destroyed.
 *
 * @retval #LOS_ERRNO_EVENT_PTR_NULL Null pointer.
 * @retval #LOS_OK                   The event is successfully cleared.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventPoll | LOS_EventRead | LOS_EventWrite
 */
extern UINT32 LOS_EventDestroy(PEVENT_CB_S eventCB);

extern UINT32 OsEventReadOnce(PEVENT_CB_S eventCB, UINT32 eventMask, UINT32 mode, UINT32 timeOut);

extern UINT32 OsEventWriteOnce(PEVENT_CB_S eventCB, UINT32 events);

/**
 * @ingroup los_err
 * @brief Error handling function.
 *
 * @par Description:
 * This API is used to perform different operations according to error types.
 * @attention
 * <ul>
 * <li>None</li>
 * </ul>
 *
 * @param  fileName  [IN] Log file that stores error information.
 * @param  lineNo    [IN] Line number of the erroneous line which should not be OS_ERR_MAGIC_WORD.
 * @param  errorNo   [IN] Error code.
 * @param  paraLen   [IN] Length of the input parameter pPara.
 * @param  para      [IN] User label of the error.
 *
 * @retval LOS_OK The error is successfully processed.
 * @par Dependency:
 * <ul><li>los_err.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern UINT32 LOS_ErrHandle(CHAR *fileName,
                            UINT32 lineNo,
                            UINT32 errorNo,
                            UINT32 paraLen,
                            void *para);

extern UINT32 ArchEnterSleep(void);

/**
 * @ingroup los_timer
 * @brief Get tick timer control block.
 *
 * @par Description:
 * This API is used to get tick timer control block.
 *
 * @param  None
 *
 * @retval #tick timer control block
 * @par Dependency:
 * <ul><li>los_timer.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern struct ArchTickTimer *ArchSysTickTimerGet(void);

/**
 *  @ingroup  los_tick
 *  @brief: System timer cycles get function.
 *
 *  @par Description:
 *  This API is used to get system timer cycles.
 *
 * @attention:
 * <ul><li>None.</li></ul>
 *
 * @param: None.
 *
 * @retval: current system cycles.
 *
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 *
 */
extern UINT64 LOS_SysCycleGet(void);

extern UINT64 OsTickTimerReload(UINT64 period);

extern void OsTickTimerBaseReset(UINT64 currTime);

extern UINT32 OsTickTimerInit(void);

extern void OsTickSysTimerStartTimeSet(UINT64 currTime);

/**
 * @ingroup los_tick
 * @brief Adjust the system tick timer clock frequency.
 *
 * @par Description:
 * This API is used to adjust the system tick timer clock frequency.
 * @attention
 * <ul>
 * <li>This function needs to be invoked only when the clock frequency of the system tick timer adjust as a result of
 * changing the CPU frequency.</li>
 * </ul>
 *
 * @param  handler [IN] Adjust the system tick timer clock frequency function hooks.
 * @param param   [IN] Function parameters.
 *
 * @retval LOS_OK or Error code.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern UINT32 LOS_SysTickClockFreqAdjust(SYS_TICK_FREQ_ADJUST_FUNC handler,
                                         UINTPTR param);

/**
 * @ingroup los_tick
 * @brief Obtain the number of Ticks.
 *
 * @par Description:
 * This API is used to obtain the number of Ticks.
 * @attention
 * <ul>
 * <li>None</li>
 * </ul>
 *
 * @param  None
 *
 * @retval UINT64 The number of Ticks.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern UINT64 LOS_TickCountGet(void);

/**
 * @ingroup los_tick
 * @brief Obtain the number of cycles in one second.
 *
 * @par Description:
 * This API is used to obtain the number of cycles in one second.
 * @attention
 * <ul>
 * <li>None</li>
 * </ul>
 *
 * @param  None
 *
 * @retval UINT32 Number of cycles obtained in one second.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern UINT32 LOS_CyclePerTickGet(void);

/**
 * @ingroup los_tick
 * @brief Convert Ticks to milliseconds.
 *
 * @par Description:
 * This API is used to convert Ticks to milliseconds.
 * @attention
 * <ul>
 * <li>The number of milliseconds obtained through the conversion is 32-bit.</li>
 * </ul>
 *
 * @param  ticks  [IN] Number of Ticks. The value range is (0,OS_SYS_CLOCK).
 *
 * @retval UINT32 Number of milliseconds obtained through the conversion. Ticks are successfully converted to
 * milliseconds.
 * @par  Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MS2Tick
 */
extern UINT32 LOS_Tick2MS(UINT32 ticks);

/**
 * @ingroup los_tick
 * @brief Convert milliseconds to Ticks.
 *
 * @par Description:
 * This API is used to convert milliseconds to Ticks.
 * @attention
 * <ul>
 * <li>If the parameter passed in is equal to 0xFFFFFFFF, the retval is 0xFFFFFFFF. Pay attention to the value to be
 * converted because data possibly overflows.</li>
 * </ul>
 *
 * @param  millisec  [IN] Number of milliseconds.
 *
 * @retval UINT32 Number of Ticks obtained through the conversion.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_Tick2MS
 */
extern UINT32 LOS_MS2Tick(UINT32 millisec);

/**
 * @ingroup los_tick
 * @brief Re-initializes the system tick timer.
 *
 * @par Description:
 * This API is used to re-initialize the system Tick timer.
 * @attention
 *
 * @param timer        [IN] Specify the tick timer.
 * @param tickHandler  [IN] Tick Interrupts the execution of the hook function.
 *
 * @retval LOS_OK or Error code.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TickTimerRegister(const struct ArchTickTimer *timer, HWI_PROC_FUNC tickHandler);

extern void LOS_UDelay(UINT64 microseconds);

extern void LOS_MDelay(UINT32 millisec);

extern UINT64 LOS_CurrNanosec(void);

/**
 * @ingroup  los_tick
 * @brief Handle the system tick timeout.
 *
 * @par Description:
 * This API is called when the system tick timeout and triggers the interrupt.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param none.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern void OsTickHandler(void);

/**
 * @ingroup los_tick
 * @brief Convert cycles to milliseconds.
 *
 * @par Description:
 * This API is used to convert cycles to milliseconds.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  cpuTick  [IN]  Number of CPU cycles.
 * @param  msHi     [OUT] Upper 32 bits of the number of milliseconds.
 * @param  msLo     [OUT] Lower 32 bits of the number of milliseconds.
 *
 * @retval #LOS_ERRNO_SYS_PTR_NULL    0x02000011: Invalid parameter.
 * @retval #LOS_OK                   0:  Cycles are successfully converted to microseconds.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsCpuTick2MS(CpuTick *cpuTick, UINT32 *msHi, UINT32 *msLo);

/**
 * @ingroup los_tick
 * @brief Convert cycles to microseconds.
 *
 * @par Description:
 * This API is used to convert cycles to microseconds.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  cpuTick  [IN]  Number of CPU cycles.
 * @param  usHi     [OUT] Upper 32 bits of the number of microseconds.
 * @param  usLo     [OUT] Lower 32 bits of the number of microseconds.
 *
 * @retval #LOS_ERRNO_SYS_PTR_NULL    0x02000011: Invalid parameter.
 * @retval #LOS_OK                   0: Cycles are successfully converted to microseconds.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsCpuTick2US(CpuTick *cpuTick, UINT32 *usHi, UINT32 *usLo);

extern struct SortLinkAttribute *OsGetSortLinkAttribute(SortLinkType type_);

extern UINT32 OsSortLinkInit(struct SortLinkAttribute *sortLinkHead);

extern void OsAdd2SortLink(struct SortLinkList *node,
                           UINT64 startTime,
                           UINT32 waitTicks,
                           SortLinkType type_);

extern void OsDeleteSortLink(struct SortLinkList *node);

extern UINT64 OsSortLinkGetTargetExpireTime(UINT64 currTime,
                                            const struct SortLinkList *targetSortList);

extern UINT64 OsSortLinkGetNextExpireTime(const struct SortLinkAttribute *sortLinkHead);

extern void OsSortLinkResponseTimeConvertFreq(UINT32 oldFreq);

/**
 * @ingroup los_task
 * @brief Sleep the current task.
 *
 * @par Description:
 * This API is used to delay the execution of the current task. The task is able to be scheduled
 * after it is delayed for a specified number of Ticks.
 *
 * @attention
 * <ul>
 * <li>The task fails to be delayed if it is being delayed during interrupt processing or it is locked.</li>
 * <li>If 0 is passed in and the task scheduling is not locked,
 * execute the next task in the queue of tasks with the priority of the current task.
 * If no ready task with the priority of the current task is available,
 * the task scheduling will not occur, and the current task continues to be executed.</li>
 * <li>The parameter passed in can not be equal to LOS_WAIT_FOREVER(0xFFFFFFFF).
 * If that happens, the task will not sleep 0xFFFFFFFF milliseconds or sleep forever but sleep 0xFFFFFFFF Ticks.</li>
 * </ul>
 *
 * @param mSecs [IN] Type #UINT32 Number of MS for which the task is delayed.
 *
 * @retval None
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern void LOS_Msleep(UINT32 mSecs);

/**
 * @ingroup los_task
 * @brief System kernel initialization function.
 *
 * @par Description:
 * This API is used to start liteOS .
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param: None.
 *
 * @retval #LOS_OK                                  0:LiteOS start success.
 *
 * @par Dependency:
 * <ul><li>los_config.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_Start(void);

extern void LOS_Reboot(void);

extern void LOS_Panic(const CHAR *fmt);

/**
 * @ingroup los_task
 * @brief System kernel initialization function.
 *
 * @par Description:
 * This API is used to Initialize kernel ,configure all system modules.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param: None.
 *
 * @retval #LOS_OK                                  0:System kernel initialization success.
 *
 * @par Dependency:
 * <ul><li>los_config.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_KernelInit(void);

/**
 * @ingroup  los_task
 * @brief Create a task and suspend.
 *
 * @par Description:
 * This API is used to create a task and suspend it. This task will not be added to the queue of ready tasks before
 * resume it.
 *
 * @attention
 * <ul>
 * <li>During task creation, the task control block and task stack of the task that is previously automatically deleted
 * are deallocated.</li>
 * <li>The task name is a pointer and is not allocated memory.</li>
 * <li>If the size of the task stack of the task to be created is 0, configure #LOSCFG_BASE_CORE_TSK_DEFAULT_STACK_SIZE
 * to specify the default task stack size. The stack size should be a reasonable value, if the size is too large, may
 * cause memory exhaustion.</li>
 * <li>The task stack size must be aligned on the boundary of 8 bytes. The size is determined by whether it is big
 * enough to avoid task stack overflow.</li>
 * <li>Less parameter value indicates higher task priority.</li>
 * <li>The task name cannot be null.</li>
 * <li>The pointer to the task executing function cannot be null.</li>
 * <li>The two parameters of this interface is pointer, it should be a correct value, otherwise, the system may be
 * abnormal.</li>
 * <li>If user mode is enabled, user should input user stack pointer and size, the size must fit the stack pointer,
 * uwStackSize remain as the kernel stack size.</li>
 * </ul>
 *
 * @param  taskID        [OUT] Type  #UINT32 * Task ID.
 * @param  taskInitParam [IN]  Type  #TSK_INIT_PARAM_S * Parameter for task creation.
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID        Invalid Task ID, param puwTaskID is NULL.
 * @retval #LOS_ERRNO_TSK_PTR_NULL          Param pstInitParam is NULL.
 * @retval #LOS_ERRNO_TSK_NAME_EMPTY        The task name is NULL.
 * @retval #LOS_ERRNO_TSK_ENTRY_NULL        The task entrance is NULL.
 * @retval #LOS_ERRNO_TSK_PRIOR_ERROR       Incorrect task priority.
 * @retval #LOS_ERRNO_TSK_STKSZ_TOO_LARGE   The task stack size is too large.
 * @retval #LOS_ERRNO_TSK_STKSZ_TOO_SMALL   The task stack size is too small.
 * @retval #LOS_ERRNO_TSK_TCB_UNAVAILABLE   No free task control block is available.
 * @retval #LOS_ERRNO_TSK_NO_MEMORY         Insufficient memory for task creation.
 * @retval #LOS_OK                          The task is successfully created.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * <ul><li>los_config.h: the header file that contains system configuration items.</li></ul>
 * @see LOS_TaskDelete
 */
extern UINT32 LOS_TaskCreateOnly(UINT32 *taskID,
                                 TSK_INIT_PARAM_S *taskInitParam);

/**
 * @ingroup  los_task
 * @brief Create a task.
 *
 * @par Description:
 * This API is used to create a task. If the priority of the task created after system initialized is higher than
 * the current task and task scheduling is not locked, it is scheduled for running.
 * If not, the created task is added to the queue of ready tasks.
 *
 * @attention
 * <ul>
 * <li>During task creation, the task control block and task stack of the task that is previously automatically
 * deleted are deallocated.</li>
 * <li>The task name is a pointer and is not allocated memory.</li>
 * <li>If the size of the task stack of the task to be created is 0, configure #LOSCFG_BASE_CORE_TSK_DEFAULT_STACK_SIZE
 * to specify the default task stack size.</li>
 * <li>The task stack size must be aligned on the boundary of 8 bytes. The size is determined by whether it is big
 * enough to avoid task stack overflow.</li>
 * <li>Less parameter value indicates higher task priority.</li>
 * <li>The task name cannot be null.</li>
 * <li>The pointer to the task executing function cannot be null.</li>
 * <li>The two parameters of this interface is pointer, it should be a correct value, otherwise, the system may be
 * abnormal.</li>
 * <li>If user mode is enabled, user should input user stack pointer and size, the size must fit the stack pointer,
 * uwStackSize remain as the kernel stack size.</li>
 * </ul>
 *
 * @param  taskID        [OUT] Type  #UINT32 * Task ID.
 * @param  taskInitParam [IN]  Type  #TSK_INIT_PARAM_S * Parameter for task creation.
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID        Invalid Task ID, param puwTaskID is NULL.
 * @retval #LOS_ERRNO_TSK_PTR_NULL          Param pstInitParam is NULL.
 * @retval #LOS_ERRNO_TSK_NAME_EMPTY        The task name is NULL.
 * @retval #LOS_ERRNO_TSK_ENTRY_NULL        The task entrance is NULL.
 * @retval #LOS_ERRNO_TSK_PRIOR_ERROR       Incorrect task priority.
 * @retval #LOS_ERRNO_TSK_STKSZ_TOO_LARGE   The task stack size is too large.
 * @retval #LOS_ERRNO_TSK_STKSZ_TOO_SMALL   The task stack size is too small.
 * @retval #LOS_ERRNO_TSK_TCB_UNAVAILABLE   No free task control block is available.
 * @retval #LOS_ERRNO_TSK_NO_MEMORY         Insufficient memory for task creation.
 * @retval #LOS_OK                          The task is successfully created.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * <ul><li>los_config.h: the header file that contains system configuration items.</li></ul>
 * @see LOS_TaskDelete
 */
extern UINT32 LOS_TaskCreate(UINT32 *taskID,
                             TSK_INIT_PARAM_S *taskInitParam);

/**
 * @ingroup  los_task
 * @brief Resume a task.
 *
 * @par Description:
 * This API is used to resume a suspended task.
 *
 * @attention
 * <ul>
 * <li>If the task is delayed or blocked, resume the task without adding it to the queue of ready tasks.</li>
 * <li>If the priority of the task resumed after system initialized is higher than the current task and task scheduling
 * is not locked, it is scheduled for running.</li>
 * </ul>
 *
 * @param  taskID [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID        Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED       The task is not created.
 * @retval #LOS_ERRNO_TSK_NOT_SUSPENDED     The task is not suspended.
 * @retval #LOS_OK                          The task is successfully resumed.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskSuspend
 */
extern UINT32 LOS_TaskResume(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Suspend a task.
 *
 * @par Description:
 * This API is used to suspend a specified task, and the task will be removed from the queue of ready tasks.
 *
 * @attention
 * <ul>
 * <li>The task that is running and locked cannot be suspended.</li>
 * <li>The idle task and swtmr task cannot be suspended.</li>
 * </ul>
 *
 * @param  taskID [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 *
 * @retval #LOS_ERRNO_TSK_OPERATE_IDLE                  Check the task ID and do not operate on the idle task.
 * @retval #LOS_ERRNO_TSK_SUSPEND_SWTMR_NOT_ALLOWED     Check the task ID and do not operate on the swtmr task.
 * @retval #LOS_ERRNO_TSK_ID_INVALID                    Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED                   The task is not created.
 * @retval #LOS_ERRNO_TSK_ALREADY_SUSPENDED             The task is already suspended.
 * @retval #LOS_ERRNO_TSK_SUSPEND_LOCKED                The task being suspended is current task and task scheduling
 *                                                      is locked.
 * @retval #LOS_OK                                      The task is successfully suspended.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskResume
 */
extern UINT32 LOS_TaskSuspend(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Delete a task.
 *
 * @par Description:
 * This API is used to delete a specified task and release the resources for its task stack and task control block.
 *
 * @attention
 * <ul>
 * <li>The idle task and swtmr task cannot be deleted.</li>
 * <li>If delete current task maybe cause unexpected error.</li>
 * <li>If a task get a mutex is deleted or automatically deleted before release this mutex, other tasks pended
 * this mutex maybe never be scheduled.</li>
 * </ul>
 *
 * @param  taskID [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 *
 * @retval #LOS_ERRNO_TSK_OPERATE_IDLE                  Check the task ID and do not operate on the idle task.
 * @retval #LOS_ERRNO_TSK_SUSPEND_SWTMR_NOT_ALLOWED     Check the task ID and do not operate on the swtmr task.
 * @retval #LOS_ERRNO_TSK_ID_INVALID                    Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED                   The task is not created.
 * @retval #LOS_OK                                      The task is successfully deleted.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskCreate | LOS_TaskCreateOnly
 */
extern UINT32 LOS_TaskDelete(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Delay a task.
 *
 * @par Description:
 * This API is used to delay the execution of the current task. The task is able to be scheduled after it is delayed
 * for a specified number of Ticks.
 *
 * @attention
 * <ul>
 * <li>The task fails to be delayed if it is being delayed during interrupt processing or it is locked.</li>
 * <li>If 0 is passed in and the task scheduling is not locked, execute the next task in the queue of tasks with
 * the same priority of the current task.
 * If no ready task with the priority of the current task is available, the task scheduling will not occur, and the
 * current task continues to be executed.</li>
 * <li>Using the interface before system initialized is not allowed.</li>
 * </ul>
 *
 * @param  tick [IN] Type #UINT32 Number of Ticks for which the task is delayed.
 *
 * @retval #LOS_ERRNO_TSK_DELAY_IN_INT              The task delay occurs during an interrupt.
 * @retval #LOS_ERRNO_TSK_DELAY_IN_LOCK             The task delay occurs when the task scheduling is locked.
 * @retval #LOS_ERRNO_TSK_ID_INVALID                Invalid Task ID
 * @retval #LOS_ERRNO_TSK_YIELD_NOT_ENOUGH_TASK     No tasks with the same priority is available for scheduling.
 * @retval #LOS_OK                                  The task is successfully delayed.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskDelay(UINT32 tick);

/**
 * @ingroup  los_task
 * @brief Lock the task scheduling.
 *
 * @par Description:
 * This API is used to lock the task scheduling. Task switching will not occur if the task scheduling is locked.
 *
 * @attention
 * <ul>
 * <li>If the task scheduling is locked, but interrupts are not disabled, tasks are still able to be interrupted.</li>
 * <li>One is added to the number of task scheduling locks if this API is called. The number of locks is decreased by
 * one if the task scheduling is unlocked. Therefore, this API should be used together with LOS_TaskUnlock.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskUnlock
 */
extern void LOS_TaskLock(void);

/**
 * @ingroup  los_task
 * @brief Unlock the task scheduling.
 *
 * @par Description:
 * This API is used to unlock the task scheduling. Calling this API will decrease the number of task locks by one.
 * If a task is locked more than once, the task scheduling will be unlocked only when the number of locks becomes zero.
 *
 * @attention
 * <ul>
 * <li>The number of locks is decreased by one if this API is called. One is added to the number of task scheduling
 * locks if the task scheduling is locked. Therefore, this API should be used together with LOS_TaskLock.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskLock
 */
extern void LOS_TaskUnlock(void);

/**
 * @ingroup  los_task
 * @brief Set a task priority.
 *
 * @par Description:
 * This API is used to set the priority of a specified task.
 *
 * @attention
 * <ul>
 * <li>If the set priority is higher than the priority of the current running task, task scheduling probably occurs.
 * </li>
 * <li>Changing the priority of the current running task also probably causes task scheduling.</li>
 * <li>Using the interface to change the priority of software timer task and idle task is not allowed.</li>
 * <li>Using the interface in the interrupt is not allowed.</li>
 * </ul>
 *
 * @param  taskID   [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 * @param  taskPrio [IN] Type #UINT16 Task priority.
 *
 * @retval #LOS_ERRNO_TSK_PRIOR_ERROR    Incorrect task priority.Re-configure the task priority
 * @retval #LOS_ERRNO_TSK_OPERATE_IDLE   Check the task ID and do not operate on the idle task.
 * @retval #LOS_ERRNO_TSK_ID_INVALID     Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED    The task is not created.
 * @retval #LOS_OK                       The task priority is successfully set to a specified priority.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskPriSet
 */
extern UINT32 LOS_TaskPriSet(UINT32 taskID,
                             UINT16 taskPrio);

/**
 * @ingroup  los_task
 * @brief Set the priority of the current running task to a specified priority.
 *
 * @par Description:
 * This API is used to set the priority of the current running task to a specified priority.
 *
 * @attention
 * <ul>
 * <li>Changing the priority of the current running task probably causes task scheduling.</li>
 * <li>Using the interface to change the priority of software timer task and idle task is not allowed.</li>
 * <li>Using the interface in the interrupt is not allowed.</li>
 * </ul>
 *
 * @param  taskPrio [IN] Type #UINT16 Task priority.
 *
 * @retval #LOS_ERRNO_TSK_PRIOR_ERROR     Incorrect task priority.Re-configure the task priority
 * @retval #LOS_ERRNO_TSK_OPERATE_IDLE    Check the task ID and do not operate on the idle task.
 * @retval #LOS_ERRNO_TSK_ID_INVALID      Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED     The task is not created.
 * @retval #LOS_OK                        The priority of the current running task is successfully set to a specified
 *                                        priority.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskPriGet
 */
extern UINT32 LOS_CurTaskPriSet(UINT16 taskPrio);

/**
 * @ingroup  los_task
 * @brief Change the scheduling sequence of tasks with the same priority.
 *
 * @par Description:
 * This API is used to move current task in a queue of tasks with the same priority to the tail of the queue of ready
 * tasks.
 *
 * @attention
 * <ul>
 * <li>At least two ready tasks need to be included in the queue of ready tasks with the same priority. If the
 * less than two ready tasks are included in the queue, an error is reported.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID                    Invalid Task ID
 * @retval #LOS_ERRNO_TSK_YIELD_NOT_ENOUGH_TASK         No tasks with the same priority is available for scheduling.
 * @retval #LOS_OK                                      The scheduling sequence of tasks with same priority is
 *                                                      successfully changed.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskYield(void);

/**
 * @ingroup  los_task
 * @brief Obtain a task priority.
 *
 * @par Description:
 * This API is used to obtain the priority of a specified task.
 *
 * @attention None.
 *
 * @param  taskID [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 *
 * @retval #OS_INVALID      The task priority fails to be obtained.
 * @retval #UINT16          The task priority.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskPriSet
 */
extern UINT16 LOS_TaskPriGet(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Obtain current running task ID.
 *
 * @par Description:
 * This API is used to obtain the ID of current running task.
 *
 * @attention
 * <ul>
 * <li> This interface should not be called before system initialized.</li>
 * </ul>
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID    Invalid Task ID.
 * @retval #UINT32                      Task ID.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_CurTaskIDGet(void);

/**
 * @ingroup  los_task
 * @brief Obtain next running task ID.
 *
 * @par Description:
 * This API is used to obtain the ID of next running task.
 *
 * @attention None.
 *
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID    invalid Task ID.
 * @retval #UINT32                      task id.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_NextTaskIDGet(void);

/**
 * @ingroup  los_task
 * @brief Obtain next running task ID.
 *
 * @par Description:
 * This API is used to obtain the ID of next running task.
 *
 * @attention None.
 *
 *
 * @retval #NULL            invalid Task name.
 * @retval #CHAR*           task name.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern CHAR *LOS_CurTaskNameGet(void);

/**
 * @ingroup  los_task
 * @brief Obtain a task information structure.
 *
 * @par Description:
 * This API is used to obtain a task information structure.
 *
 * @attention
 * <ul>
 * <li>One parameter of this interface is a pointer, it should be a correct value, otherwise, the system may be
 * abnormal.</li>
 * </ul>
 *
 * @param  taskID    [IN]  Type  #UINT32 Task ID. The task id value is obtained from task creation.
 * @param  taskInfo  [OUT] Type  #TSK_INFO_S* Pointer to the task information structure to be obtained.
 *
 * @retval #LOS_ERRNO_TSK_PTR_NULL        Null parameter.
 * @retval #LOS_ERRNO_TSK_ID_INVALID      Invalid task ID.
 * @retval #LOS_ERRNO_TSK_NOT_CREATED     The task is not created.
 * @retval #LOS_OK                        The task information structure is successfully obtained.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskInfoGet(UINT32 taskID,
                              TSK_INFO_S *taskInfo);

/**
 * @ingroup  los_task
 * @brief Obtain the task status.
 *
 * @par Description:
 * This API is used to obtain the task status.
 *
 * @attention None.
 *
 * @param  taskID     [IN] Type  #TSK_HANDLE_T Task ID.
 * @param  taskStatus [OUT] Type  #UINT32 Pointer to the task status to be obtained.
 *
 * @retval #LOS_ERRNO_TSK_PTR_NULL                    0x02000201: Null parameter.
 * @retval #LOS_ERRNO_TSK_ID_INVALID                  0x02000207: Invalid task ID.
 * @retval #LOS_ERRNO_TSK_NOT_CREATED                 0x0200020a: The task is not created.
 * @retval #LOS_OK                                   0: The task information structure is successfully obtained.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskStatusGet(UINT32 taskID,
                                UINT32 *taskStatus);

/**
 * @ingroup los_monitor
 * @brief Obtain all tasks info.
 *
 * @par Description:
 * This API is used to obtain all tasks info.
 * @attention
 * <ul>
 * <li>This API can be called only after the CPU usage is initialized. Otherwise, -1 will be returned.</li>
 * </ul>
 *
 * @param None.
 *
 * @retval #OS_ERROR           -1:all tasks info obtain failed.
 * @retval #LOS_OK              0:all tasks info is successfully obtained.
 * @par Dependency:
 * <ul><li>los_monitor.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskInfoMonitor
 */
extern UINT32 LOS_TaskInfoMonitor(void);

/**
 * @ingroup  los_task
 * @brief Obtain tasks switch info.
 *
 * @par Description:
 * This API is used to obtain tasks switch info.
 *
 * @attention None.
 *
 * @param  index            [IN]  Type  #UINT32  Switch info array index.
 * @param  taskSwitchInfo   [OUT] Type  #UINT32* First 4 bytes is task id, and then is task name, name len is
 *                                OS_TSK_NAME_LEN.
 *
 * @retval #LOS_ERRNO_TSK_PTR_NULL           0x02000201: Null parameter.
 * @retval #LOS_OK                           0: The task switch information is successfully obtained.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskSwitchInfoGet(UINT32 index,
                                    UINT32 *taskSwitchInfo);

/**
 * @ingroup  los_task
 * @brief Obtain tasks schduling info.
 *
 * @par Description:
 * This API is used to obtain task is scheduled.
 *
 * @attention None.
 *
 * @param None.
 *
 * @retval #TRUE         Tasks is scheduled.
 * @retval #FALSE        Tasks not scheduling yet.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern BOOL LOS_TaskIsRunning(void);

/**
 * @ingroup  los_task
 * @brief Obtain current new task ID.
 *
 * @par Description:
 * This API is used to obtain the ID of new task.
 *
 * @attention None.
 *
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID    0x02000207: invalid Task ID.
 * @retval # Task ID.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_NewTaskIDGet(void);

/**
 * @ingroup  los_task
 * @brief Obtain the task name.
 *
 * @par Description:
 * This API is used to obtain the task name.
 *
 * @attention None.
 *
 * @param  taskID            [IN]  Task ID.
 *
 * @retval #NULL: invalid Task name.
 * @retval # Task name.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern CHAR *LOS_TaskNameGet(UINT32 taskID);

extern UINT32 LOS_TaskJoin(UINT32 taskID, UINTPTR *retval);

extern UINT32 LOS_TaskDetach(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Initialization a task.
 *
 * @par Description:
 * This API is used to initialization a task.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  UINT32    Initialization result.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 OsTaskInit(void);

/**
 * @ingroup  los_task
 * @brief Create idle task.
 *
 * @par Description:
 * This API is used to create idle task.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  UINT32   Create result.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 OsIdleTaskCreate(void);

/**
 * @ingroup  los_task
 * @brief Check task switch.
 *
 * @par Description:
 * This API is used to check task switch.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern void OsTaskSwitchCheck(void);

/**
 * @ingroup  los_task
 * @brief TaskMonInit.
 *
 * @par Description:
 * This API is used to taskMonInit.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern void OsTaskMonInit(void);

/**
 * @ingroup  los_task
 * @brief Task entry.
 *
 * @par Description:
 * This API is used to task entry.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  taskID  [IN] Type #UINT32   task id.
 *
 * @retval  None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern void OsTaskEntry(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Get the task water line.
 *
 * @par Description:
 * This API is used to get the task water line.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  taskID [IN] Type #UINT32 task id.
 *
 * @retval  UINT32  Task water line.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsGetTaskWaterLine(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Convert task status to string.
 *
 * @par Description:
 * This API is used to convert task status to string.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  taskStatus [IN] Type #UINT16 task status.
 *
 * @retval  UINT8 *  String.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT8 *OsConvertTskStatus(UINT16 taskStatus);

/**
 * @ingroup  los_task
 * @brief Get all task information.
 *
 * @par Description:
 * This API is used to get all task information.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  UINT32  All task information.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsGetAllTskInfo(void);

extern void *OsTskUserStackInit(void *stackPtr, void *userSP, UINT32 userStackSize);

extern UINT32 OsPmEnterHandlerSet(void (*func)(void));

extern void LOS_TaskResRecycle(void);

/**
 * @ingroup los_mux
 * @brief Create a mutex.
 *
 * @par Description:
 * This API is used to create a mutex. A mutex handle is assigned to muxHandle when the mutex is created successfully.
 * Return LOS_OK on creating successful, return specific error code otherwise.
 * @attention
 * <ul>
 * <li>The total number of mutexes is pre-configured. If there are no available mutexes, the mutex creation fails.</li>
 * </ul>
 *
 * @param muxHandle   [OUT] Handle pointer of the successfully created mutex. The value of handle should be in
 * [0, LOSCFG_BASE_IPC_MUX_LIMIT - 1].
 *
 * @retval #LOS_ERRNO_MUX_PTR_NULL           The muxHandle pointer is NULL.
 * @retval #LOS_ERRNO_MUX_ALL_BUSY           No available mutex.
 * @retval #LOS_OK                           The mutex is successfully created.
 * @par Dependency:
 * <ul><li>los_mux.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MuxDelete
 */
extern UINT32 LOS_MuxCreate(UINT32 *muxHandle);

/**
 * @ingroup los_mux
 * @brief Delete a mutex.
 *
 * @par Description:
 * This API is used to delete a specified mutex. Return LOS_OK on deleting successfully, return specific error code
 * otherwise.
 * @attention
 * <ul>
 * <li>The specific mutex should be created firstly.</li>
 * <li>The mutex can be deleted successfully only if no other tasks pend on it.</li>
 * </ul>
 *
 * @param muxHandle   [IN] Handle of the mutex to be deleted. The value of handle should be in
 * [0, LOSCFG_BASE_IPC_MUX_LIMIT - 1].
 *
 * @retval #LOS_ERRNO_MUX_INVALID            Invalid handle or mutex in use.
 * @retval #LOS_ERRNO_MUX_PENDED             Tasks pended on this mutex.
 * @retval #LOS_OK                           The mutex is successfully deleted.
 * @par Dependency:
 * <ul><li>los_mux.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MuxCreate
 */
extern UINT32 LOS_MuxDelete(UINT32 muxHandle);

/**
 * @ingroup los_mux
 * @brief Wait to lock a mutex.
 *
 * @par Description:
 * This API is used to wait for a specified period of time to lock a mutex.
 * @attention
 * <ul>
 * <li>The specific mutex should be created firstly.</li>
 * <li>The function fails if the mutex that is waited on is already locked by another thread when the task scheduling
 * is disabled.</li>
 * <li>Do not wait on a mutex during an interrupt.</li>
 * <li>The priority inheritance protocol is supported. If a higher-priority thread is waiting on a mutex, it changes
 * the priority of the thread that owns the mutex to avoid priority inversion.</li>
 * <li>A recursive mutex can be locked more than once by the same thread.</li>
 * </ul>
 *
 * @param muxHandle    [IN] Handle of the mutex to be waited on.  The value of handle should be
 * in [0, LOSCFG_BASE_IPC_MUX_LIMIT - 1].
 * @param timeout      [IN] Waiting time. The value range is [0, LOS_WAIT_FOREVER](unit: Tick).
 *
 * @retval #LOS_ERRNO_MUX_INVALID            The mutex state (for example, the mutex does not exist or is not in use)
 *                                           is not applicable for the current operation.
 * @retval #LOS_ERRNO_MUX_UNAVAILABLE        The mutex fails to be locked because it is locked by another thread and
 * a period of time is not set for waiting for the mutex to become available.
 * @retval #LOS_ERRNO_MUX_IN_INTERR        The mutex is being locked during an interrupt.
 * @retval #LOS_ERRNO_MUX_PEND_IN_LOCK       The mutex is waited on when the task scheduling is disabled.
 * @retval #LOS_ERRNO_MUX_TIMEOUT            The mutex waiting times out.
 * @retval #LOS_OK                           The mutex is successfully locked.
 * @par Dependency:
 * <ul><li>los_mux.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MuxCreate | LOS_MuxPost
 */
extern UINT32 LOS_MuxPend(UINT32 muxHandle,
                          UINT32 timeout);

/**
 * @ingroup los_mux
 * @brief Release a mutex.
 *
 * @par Description:
 * This API is used to release a specified mutex.
 * @attention
 * <ul>
 * <li>The specific mutex should be created firstly.</li>
 * <li>Do not release a mutex during an interrupt.</li>
 * <li>If a recursive mutex is locked for many times, it must be unlocked for the same times to be released.</li>
 * </ul>
 *
 * @param muxHandle    [IN] Handle of the mutex to be released. The value of handle should be in
 * [0, LOSCFG_BASE_IPC_MUX_LIMIT - 1].
 *
 * @retval #LOS_ERRNO_MUX_INVALID            The mutex state (for example, the mutex does not exist or is not in use
 * or owned by other thread) is not applicable for the current operation.
 * @retval #LOS_ERRNO_MUX_IN_INTERR        The mutex is being released during an interrupt.
 * @retval #LOS_OK                           The mutex is successfully released.
 * @par Dependency:
 * <ul><li>los_mux.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MuxCreate | LOS_MuxPend
 */
extern UINT32 LOS_MuxPost(UINT32 muxHandle);

/**
 * @ingroup los_mux
 * @brief Initializes the mutex.
 *
 * @par Description:
 * This API is used to initializes the mutex.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param None.
 *
 * @retval UINT32     Initialization result.
 * @par Dependency:
 * <ul><li>los_mux.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MuxDelete
 */
extern UINT32 OsMuxInit(void);

/**
 * @ingroup los_queue
 * @brief Create a message queue.
 *
 * @par Description:
 * This API is used to create a message queue.
 * @attention
 * <ul>
 * <li>There are LOSCFG_BASE_IPC_QUEUE_LIMIT queues available, change it's value when necessary.</li>
 * </ul>
 * @param queueName        [IN]    Message queue name.
 * @param len              [IN]    Queue length. The value range is [1,0xffff].
 * @param queueID          [OUT]   ID of the queue control structure that is successfully created.
 * @param flags            [IN]    Queue mode. Reserved parameter, not used for now.
 * @param maxMsgSize       [IN]    Node size. The value range is [1,0xffff-4].
 *
 * @retval   #LOS_OK                               The message queue is successfully created.
 * @retval   #LOS_ERRNO_QUEUE_CB_UNAVAILABLE       The upper limit of the number of created queues is exceeded.
 * @retval   #LOS_ERRNO_QUEUE_CREATE_NO_MEMORY     Insufficient memory for queue creation.
 * @retval   #LOS_ERRNO_QUEUE_CREAT_PTR_NULL       Null pointer, queueID is NULL.
 * @retval   #LOS_ERRNO_QUEUE_PARA_ISZERO          The queue length or message node size passed in during queue
 * creation is 0.
 * @retval   #LOS_ERRNO_QUEUE_SIZE_TOO_BIG         The parameter maxMsgSize is larger than 0xffff - 4.
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_QueueDelete
 */
extern UINT32 LOS_QueueCreate(const CHAR *queueName,
                              UINT16 len,
                              UINT32 *queueID,
                              UINT32 flags,
                              UINT16 maxMsgSize);

/**
 * @ingroup los_queue
 * @brief Create a static message queue.
 *
 * @par Description:
 * This API is used to create a message queue using static memory for data storage.
 * @attention
 * <ul>
 * <li>There are LOSCFG_BASE_IPC_QUEUE_LIMIT queues available, change it's value when necessary.</li>
 * </ul>
 * @param queueName        [IN]    Message queue name.
 * @param len              [IN]    Queue length. The value range is [1,0xffff].
 * @param queueID          [OUT]   ID of the queue control structure that is successfully created.
 * @param staticMem        [IN]    Pointer to a static memory for the message queue data.
 * @param flags            [IN]    Queue mode. Reserved parameter, not used for now.
 * @param maxMsgSize       [IN]    Node size. The value range is [1,0xffff-4].
 *
 * @retval   #LOS_OK                               The message queue is successfully created.
 * @retval   #LOS_ERRNO_QUEUE_CB_UNAVAILABLE       The upper limit of the number of created queues is exceeded.
 * @retval   #LOS_ERRNO_QUEUE_CREATE_NO_MEMORY     Insufficient memory for queue creation.
 * @retval   #LOS_ERRNO_QUEUE_CREAT_PTR_NULL       Null pointer, queueID is NULL.
 * @retval   #LOS_ERRNO_QUEUE_PARA_ISZERO          The queue length or message node size passed in during queue
 * creation is 0.
 * @retval   #LOS_ERRNO_QUEUE_SIZE_TOO_BIG         The parameter maxMsgSize is larger than 0xffff - 4.
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_QueueDelete
 */
extern UINT32 LOS_QueueCreateStatic(const CHAR *queueName,
                                    UINT16 len,
                                    UINT32 *queueID,
                                    UINT8 *staticMem,
                                    UINT32 flags,
                                    UINT16 maxMsgSize);

/**
 * @ingroup los_queue
 * @brief Read a queue.
 *
 * @par Description:
 * This API is used to read data in a specified queue, and store the obtained data to the address specified
 * by bufferAddr. The address and the size of the data to be read are defined by users.
 * @attention
 * <ul>
 * <li>The specific queue should be created firstly.</li>
 * <li>Queue reading adopts the fist in first out (FIFO) mode. The data that is first stored in the queue is read
 * first.</li>
 * <li>bufferAddr stores the obtained data.</li>
 * <li>Do not read or write a queue in unblocking modes such as an interrupt.</li>
 * <li>This API cannot be called before the kernel is initialized.</li>
 * <li>The argument timeOut is a relative time.</li>
 * </ul>
 *
 * @param queueID        [IN]     Queue ID created by LOS_QueueCreate. The value range is
 * [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 * @param bufferAddr     [OUT]    Starting address that stores the obtained data. The starting address must not be
 * null.
 * @param bufferSize     [IN/OUT] Where to maintain the buffer expected-size before read, and the real-size after read.
 * @param timeOut        [IN]     Expiry time. The value range is [0,LOS_WAIT_FOREVER](unit: Tick).
 *
 * @retval   #LOS_OK                              The queue is successfully read.
 * @retval   #LOS_ERRNO_QUEUE_INVALID             The handle of the queue that is being read is invalid.
 * @retval   #LOS_ERRNO_QUEUE_READ_PTR_NULL       The pointer passed in during queue reading is null.
 * @retval   #LOS_ERRNO_QUEUE_READSIZE_ISZERO     The buffer size passed in during queue reading is 0.
 * @retval   #LOS_ERRNO_QUEUE_READ_IN_INTERRUPT   The queue cannot be read during an interrupt when the time for
 * waiting to processing the queue expires.
 * @retval   #LOS_ERRNO_QUEUE_NOT_CREATE          The queue to be read is not created.
 * @retval   #LOS_ERRNO_QUEUE_ISEMPTY             No resource is in the queue that is being read when the time for
 * waiting to processing the queue expires.
 * @retval   #LOS_ERRNO_QUEUE_PEND_IN_LOCK        The task is forbidden to be blocked on a queue when the task is
 * locked.
 * @retval   #LOS_ERRNO_QUEUE_TIMEOUT             The time set for waiting to processing the queue expires.
 * @retval   #LOS_ERRNO_QUEUE_READ_SIZE_TOO_SMALL The buffer size passed in during queue reading is less than
 * the queue size.
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_QueueWriteCopy | LOS_QueueCreate
 */
extern UINT32 LOS_QueueReadCopy(UINT32 queueID,
                                void *bufferAddr,
                                UINT32 *bufferSize,
                                UINT32 timeOut);

/**
 * @ingroup los_queue
 * @brief Write data into a queue.
 *
 * @par Description:
 * This API is used to write the data of the size specified by bufferSize and stored at the address specified by
 * bufferAddr into a queue.
 * @attention
 * <ul>
 * <li>The specific queue should be created firstly.</li>
 * <li>Do not read or write a queue in unblocking modes such as interrupt.</li>
 * <li>This API cannot be called before the kernel is initialized.</li>
 * <li>The data to be written is of the size specified by bufferSize and is stored at the address specified by
 * BufferAddr.</li>
 * <li>The argument timeOut is a relative time.</li>
 * </ul>
 *
 * @param queueID        [IN]        Queue ID created by LOS_QueueCreate. The value range is
 * [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 * @param bufferAddr     [IN]        Starting address that stores the data to be written.The starting address must
 * not be null.
 * @param bufferSize     [IN]        Passed-in buffer size. The value range is [1,USHRT_MAX - sizeof(UINT32)].
 * @param timeOut        [IN]        Expiry time. The value range is [0,LOS_WAIT_FOREVER](unit: Tick).
 *
 * @retval   #LOS_OK                                 The data is successfully written into the queue.
 * @retval   #LOS_ERRNO_QUEUE_INVALID                The queue handle passed in during queue writing is invalid.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_PTR_NULL         The pointer passed in during queue writing is null.
 * @retval   #LOS_ERRNO_QUEUE_WRITESIZE_ISZERO       The buffer size passed in during queue writing is 0.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_IN_INTERRUPT     The queue cannot be written during an interrupt when the time
 * for waiting to processing the queue expires.
 * @retval   #LOS_ERRNO_QUEUE_NOT_CREATE             The queue into which the data is written is not created.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_SIZE_TOO_BIG     The buffer size passed in during queue writing is bigger than
 * the queue size.
 * @retval   #LOS_ERRNO_QUEUE_ISFULL                 No free node is available during queue writing.
 * @retval   #LOS_ERRNO_QUEUE_PEND_IN_LOCK           The task is forbidden to be blocked on a queue when
 * the task is locked.
 * @retval   #LOS_ERRNO_QUEUE_TIMEOUT                The time set for waiting to processing the queue expires.
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_QueueReadCopy | LOS_QueueCreate
 */
extern UINT32 LOS_QueueWriteCopy(UINT32 queueID,
                                 void *bufferAddr,
                                 UINT32 bufferSize,
                                 UINT32 timeOut);

/**
 * @ingroup los_queue
 * @brief Read a queue.
 *
 * @par Description:
 * This API is used to read the address of data in a specified queue, and store it to the address specified by
 * bufferAddr.
 * @attention
 * <ul>
 * <li>The specific queue should be created firstly.</li>
 * <li>Queue reading adopts the fist in first out (FIFO) mode. The data that is first stored in the queue is
 * read first.</li>
 * <li>bufferAddr stores the obtained data address.</li>
 * <li>Do not read or write a queue in unblocking modes such as an interrupt.</li>
 * <li>This API cannot be called before the kernel is initialized.</li>
 * <li>The argument timeOut is a relative time.</li>
 * <li>The bufferSize is not really used in LOS_QueueRead, because the interface is only used to
 * obtain the address of data.</li>
 * <li>The buffer which the bufferAddr pointing to must be greater than or equal to 4 bytes.</li>
 * </ul>
 *
 * @param queueID        [IN]        Queue ID created by LOS_QueueCreate. The value range is
 * [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 * @param bufferAddr     [OUT]       Starting address that stores the obtained data. The starting address must
 * not be null.
 * @param bufferSize     [IN]        Passed-in buffer size, which must not be 0. The value range is [1,0xffffffff].
 * @param timeOut        [IN]        Expiry time. The value range is [0,LOS_WAIT_FOREVER](unit: Tick).
 *
 * @retval   #LOS_OK                               The queue is successfully read.
 * @retval   #LOS_ERRNO_QUEUE_INVALID              The handle of the queue that is being read is invalid.
 * @retval   #LOS_ERRNO_QUEUE_READ_PTR_NULL        The pointer passed in during queue reading is null.
 * @retval   #LOS_ERRNO_QUEUE_READSIZE_ISZERO      The buffer size passed in during queue reading is 0.
 * @retval   #LOS_ERRNO_QUEUE_READ_IN_INTERRUPT    The queue cannot be read during an interrupt when the time for
 * waiting to processing the queue expires.
 * @retval   #LOS_ERRNO_QUEUE_NOT_CREATE           The queue to be read is not created.
 * @retval   #LOS_ERRNO_QUEUE_ISEMPTY              No resource is in the queue that is being read when the time for
 * waiting to processing the queue expires.
 * @retval   #LOS_ERRNO_QUEUE_PEND_IN_LOCK         The task is forbidden to be blocked on a queue when the task is
 * locked.
 * @retval   #LOS_ERRNO_QUEUE_TIMEOUT              The time set for waiting to processing the queue expires.
 * @par Dependency:
 * <ul><li>los_queue.h: The header file that contains the API declaration.</li></ul>
 * @see LOS_QueueWrite | LOS_QueueCreate
 */
extern UINT32 LOS_QueueRead(UINT32 queueID,
                            void *bufferAddr,
                            UINT32 bufferSize,
                            UINT32 timeOut);

/**
 * @ingroup los_queue
 * @brief Write data into a queue.
 *
 * @par Description:
 * This API is used to write the address of data specified by bufferAddr into a queue.
 * @attention
 * <ul>
 * <li>The specific queue should be created firstly.</li>
 * <li>Do not read or write a queue in unblocking modes such as an interrupt.</li>
 * <li>This API cannot be called before the kernel is initialized.</li>
 * <li>The address of the data of the size specified by bufferSize and stored at the address specified by
 * BufferAddr is to be written.</li>
 * <li>The argument timeOut is a relative time.</li>
 * <li>The bufferSize is not really used in LOS_QueueWrite, because the interface is only used to write the address
 * of data specified by bufferAddr into a queue.</li>
 * </ul>
 *
 * @param queueID        [IN]        Queue ID created by LOS_QueueCreate. The value range is
 * [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 * @param bufferAddr     [IN]        Starting address that stores the data to be written. The starting address
 * must not be null.
 * @param bufferSize     [IN]        Passed-in buffer size, which must not be 0. The value range is [1,0xffffffff].
 * @param timeOut        [IN]        Expiry time. The value range is [0,LOS_WAIT_FOREVER](unit: Tick).
 *
 * @retval   #LOS_OK                                The data is successfully written into the queue.
 * @retval   #LOS_ERRNO_QUEUE_INVALID               The queue handle passed in during queue writing is invalid.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_PTR_NULL        The pointer passed in during queue writing is null.
 * @retval   #LOS_ERRNO_QUEUE_WRITESIZE_ISZERO      The buffer size passed in during queue writing is 0.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_IN_INTERRUPT    The queue cannot be written during an interrupt when the time for
 * waiting to processing the queue expires.
 * @retval   #LOS_ERRNO_QUEUE_NOT_CREATE            The queue into which the data is written is not created.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_SIZE_TOO_BIG    The buffer size passed in during queue writing is bigger than
 * the queue size.
 * @retval   #LOS_ERRNO_QUEUE_ISFULL                No free node is available during queue writing.
 * @retval   #LOS_ERRNO_QUEUE_PEND_IN_LOCK          The task is forbidden to be blocked on a queue when the task is
 * locked.
 * @retval   #LOS_ERRNO_QUEUE_TIMEOUT               The time set for waiting to processing the queue expires.
 * @par Dependency:
 * <ul><li>los_queue.h: The header file that contains the API declaration.</li></ul>
 * @see LOS_QueueRead | LOS_QueueCreate
 */
extern UINT32 LOS_QueueWrite(UINT32 queueID,
                             void *bufferAddr,
                             UINT32 bufferSize,
                             UINT32 timeOut);

/**
 * @ingroup los_queue
 * @brief Write data into a queue header.
 *
 * @par Description:
 * This API is used to write the data of the size specified by bufferSize and stored at the address specified by
 * bufferAddr into a queue header.
 * @attention
 * <ul>
 * <li>Do not read or write a queue in unblocking modes such as an interrupt.</li>
 * <li>This API cannot be called before the kernel is initialized.</li>
 * <li>The address of the data of the size specified by bufferSize and stored at the address specified by
 * BufferAddr is to be written.</li>
 * <li>The argument timeOut is a relative time.</li>
 * <li>LOS_QueueRead and LOS_QueueWriteHead are a set of interfaces, and the two groups of interfaces need to be used.
 * <li>
 * </ul>
 *
 * @param queueID        [IN]        Queue ID created by LOS_QueueCreate. The value range is
 * [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 * @param bufferAddr     [OUT]       Starting address that stores the data to be written. The starting address
 * must not be null.
 * @param bufferSize     [IN]        Passed-in buffer size, which must not be 0. The value range is [1,0xffffffff].
 * @param timeOut        [IN]        Expiry time. The value range is [0,LOS_WAIT_FOREVER](unit: Tick).
 *
 * @retval   #LOS_OK                                 The data is successfully written into the queue.
 * @retval   #LOS_ERRNO_QUEUE_INVALID                The queue handle passed in during queue writing is invalid.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_PTR_NULL         The pointer passed in during queue writing is null.
 * @retval   #LOS_ERRNO_QUEUE_WRITESIZE_ISZERO       The buffer size passed in during queue writing is 0.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_IN_INTERRUPT     The queue cannot be written during an interrupt when the time for
 * waiting to processing the queue expires.
 * @retval   #LOS_ERRNO_QUEUE_NOT_CREATE             The queue into which the data is written is not created.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_SIZE_TOO_BIG     The buffer size passed in during queue writing is bigger than
 * the queue size.
 * @retval   #LOS_ERRNO_QUEUE_ISFULL                 No free node is available during queue writing.
 * @retval   #LOS_ERRNO_QUEUE_PEND_IN_LOCK           The task is forbidden to be blocked on a queue when the task is
 * locked.
 * @retval   #LOS_ERRNO_QUEUE_TIMEOUT                The time set for waiting to processing the queue expires.
 * @par Dependency:
 * <ul><li>los_queue.h: The header file that contains the API declaration.</li></ul>
 * @see LOS_QueueRead | LOS_QueueCreate
 */
extern UINT32 LOS_QueueWriteHead(UINT32 queueID,
                                 void *bufferAddr,
                                 UINT32 bufferSize,
                                 UINT32 timeOut);

/**
 * @ingroup los_queue
 * @brief Write data into a queue header.
 *
 * @par Description:
 * This API is used to write the data of the size specified by bufferSize and stored at the address specified by
 * bufferAddr into a queue header.
 * @attention
 * <ul>
 * <li>Do not read or write a queue in unblocking modes such as an interrupt.</li>
 * <li>This API cannot be called before the kernel is initialized.</li>
 * <li>The address of the data of the size specified by bufferSize and stored at the address specified by
 * BufferAddr is to be written.</li>
 * <li>The argument timeOut is a relative time.</li>
 * <li>LOS_QueueRead and LOS_QueueWriteHead are a set of interfaces, and the two groups of interfaces need to be
 * used.<li>
 * </ul>
 *
 * @param queueID        [IN]        Queue ID created by LOS_QueueCreate. The value range is
 * [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 * @param bufferAddr     [OUT]       Starting address that stores the data to be written.
 * The starting address must not be null.
 * @param bufferSize     [IN]        Passed-in buffer size, which must not be 0. The value range is [1,0xffffffff].
 * @param timeOut        [IN]        Expiry time. The value range is [0,LOS_WAIT_FOREVER](unit: Tick).
 *
 * @retval   #LOS_OK                                 The data is successfully written into the queue.
 * @retval   #LOS_ERRNO_QUEUE_INVALID                The queue handle passed in during queue writing is invalid.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_PTR_NULL         The pointer passed in during queue writing is null.
 * @retval   #LOS_ERRNO_QUEUE_WRITESIZE_ISZERO       The buffer size passed in during queue writing is 0.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_IN_INTERRUPT     The queue cannot be written during an interrupt when the time for
 * waiting to processing the queue expires.
 * @retval   #LOS_ERRNO_QUEUE_NOT_CREATE             The queue into which the data is written is not created.
 * @retval   #LOS_ERRNO_QUEUE_WRITE_SIZE_TOO_BIG     The buffer size passed in during queue writing is bigger than
 * the queue size.
 * @retval   #LOS_ERRNO_QUEUE_ISFULL                 No free node is available during queue writing.
 * @retval   #LOS_ERRNO_QUEUE_PEND_IN_LOCK           The task is forbidden to be blocked on a queue when the task is
 * locked.
 * @retval   #LOS_ERRNO_QUEUE_TIMEOUT                The time set for waiting to processing the queue expires.
 * @par Dependency:
 * <ul><li>los_queue.h: The header file that contains the API declaration.</li></ul>
 * @see LOS_QueueWrite | LOS_QueueWriteHead
 */
extern UINT32 LOS_QueueWriteHeadCopy(UINT32 queueID,
                                     void *bufferAddr,
                                     UINT32 bufferSize,
                                     UINT32 timeOut);

/**
 * @ingroup los_queue
 * @brief Delete a queue.
 *
 * @par Description:
 * This API is used to delete a queue.
 * @attention
 * <ul>
 * <li>This API cannot be used to delete a queue that is not created.</li>
 * <li>A synchronous queue fails to be deleted if any tasks are blocked on it, or some queues are being read or
 * written.</li>
 * </ul>
 *
 * @param queueID     [IN]      Queue ID created by LOS_QueueCreate. The value range is
 * [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 *
 * @retval   #LOS_OK                           The queue is successfully deleted.
 * @retval   #LOS_ERRNO_QUEUE_NOT_FOUND        The queue cannot be found.
 * @retval   #LOS_ERRNO_QUEUE_NOT_CREATE       The queue handle passed in when the queue is being deleted is
 * incorrect.
 * @retval   #LOS_ERRNO_QUEUE_IN_TSKUSE        The queue that blocks a task cannot be deleted.
 * @retval   #LOS_ERRNO_QUEUE_IN_TSKWRITE      Queue reading and writing are not synchronous.
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_QueueCreate | LOS_QueueCreate
 */
extern UINT32 LOS_QueueDelete(UINT32 queueID);

/**
 * @ingroup los_queue
 * @brief Obtain queue information.
 *
 * @par Description:
 * This API is used to obtain queue information.
 * @attention
 * <ul>
 * <li>The specific queue should be created firstly.</li>
 * </ul>
 * @param queueID       [IN]        Queue ID created by LOS_QueueCreate. The value range is
 * [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 * @param queueInfo     [OUT]       The queue information to be read must not be null.
 *
 * @retval   #LOS_OK                            The queue information is successfully obtained.
 * @retval   #LOS_ERRNO_QUEUE_PTR_NULL          The pointer to the queue information to be obtained is null.
 * @retval   #LOS_ERRNO_QUEUE_INVALID           The handle of the queue that is being read is invalid.
 * @retval   #LOS_ERRNO_QUEUE_NOT_CREATE        The queue in which the information to be obtained is stored is
 * not created.
 *
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_QueueCreate
 */
extern UINT32 LOS_QueueInfoGet(UINT32 queueID,
                               QUEUE_INFO_S *queueInfo);

extern struct LosQueueCB *OsGetQueueHandle(UINT32 queueID);

/**
 * @ingroup los_queue
 * @brief Alloc a stationary memory for a mail.
 *
 * @par Description:
 * This API is used to alloc a stationary memory for a mail according to queueID.
 * @attention
 * <ul>
 * <li>Do not alloc memory in unblocking modes such as interrupt.</li>
 * <li>This API cannot be called before the kernel is initialized.</li>
 * <li>The argument timeOut is a relative time.</li>
 * </ul>
 *
 * @param queueID        [IN]        Queue ID. The value range is [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 * @param mailPool       [IN]        The memory poll that stores the mail.
 * @param timeOut        [IN]        Expiry time. The value range is [0,LOS_WAIT_FOREVER].
 *
 * @retval   #NULL                    The memory allocation is failed.
 * @retval   #mem                     The address of alloc memory.
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see OsQueueMailFree
 */
extern void *OsQueueMailAlloc(UINT32 queueID, void *mailPool, UINT32 timeOut);

/**
 * @ingroup los_queue
 * @brief Free a stationary memory of a mail.
 *
 * @par Description:
 * This API is used to free a stationary memory for a mail according to queueID.
 * @attention
 * <ul>
 * <li>This API cannot be called before the kernel is initialized.</li>
 * </ul>
 *
 * @param queueID         [IN]        Queue ID. The value range is [1,LOSCFG_BASE_IPC_QUEUE_LIMIT].
 * @param mailPool        [IN]        The mail memory poll address.
 * @param mailMem         [IN]        The mail memory block address.
 *
 * @retval   #LOS_OK                                 0x00000000: The memory free successfully.
 * @retval   #OS_ERRNO_QUEUE_MAIL_HANDLE_INVALID     0x02000619: The handle of the queue passed-in when the memory for
 *the queue is being freed is invalid.
 * @retval   #OS_ERRNO_QUEUE_MAIL_PTR_INVALID        0x0200061a: The pointer to the memory to be freed is null.
 * @retval   #OS_ERRNO_QUEUE_MAIL_FREE_ERROR         0x0200061b: The memory for the queue fails to be freed.
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see OsQueueMailAlloc
 */
extern UINT32 OsQueueMailFree(UINT32 queueID,
                              void *mailPool,
                              void *mailMem);

/**
 * @ingroup los_queue
 * @brief Initialization queue.
 *
 * @par Description:
 * This API is used to initialization queue.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param None.
 *
 * @retval   UINT32  Initialization result.
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsQueueInit(void);

/**
 * @ingroup los_queue
 * @brief Handle when read or write queue.
 *
 * @par Description:
 * This API is used to handle when read or write queue.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param queueID        [IN]       Queue id.
 * @param operateType    [IN]       Operate type
 * @param bufferAddr     [IN]       Buffer address.
 * @param bufferSize     [IN]       Buffer size.
 * @param timeOut        [IN]       Timeout.
 *
 * @retval   UINT32  Handle result.
 * @par Dependency:
 * <ul><li>los_queue.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsQueueOperate(UINT32 queueID,
                             UINT32 operateType,
                             void *bufferAddr,
                             UINT32 *bufferSize,
                             UINT32 timeOut);

/**
 * @ingroup los_sem
 * @brief Create a Counting semaphore.
 *
 * @par Description:
 * This API is used to create a semaphore control structure according to the initial number of available semaphores
 * specified by count and return the ID of this semaphore control structure.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param count       [IN]  Initial number of available semaphores. The value range is [0, OS_SEM_COUNTING_MAX_COUNT).
 * @param semHandle   [OUT] ID of the semaphore control structure that is initialized.
 *
 * @retval #LOS_ERRNO_SEM_PTR_NULL     The passed-in semHandle value is NULL.
 * @retval #LOS_ERRNO_SEM_OVERFLOW     The passed-in count value is greater than the maximum number of available
 * semaphores.
 * @retval #LOS_ERRNO_SEM_ALL_BUSY     No semaphore control structure is available.
 * @retval #LOS_OK   The semaphore is successfully created.
 * @par Dependency:
 * <ul><li>los_sem.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_SemDelete
 */
extern UINT32 LOS_SemCreate(UINT16 count,
                            UINT32 *semHandle);

/**
 * @ingroup los_sem
 * @brief Create a binary semaphore.
 *
 * @par Description:
 * This API is used to create a binary semaphore control structure according to the initial number of available
 * semaphores specified by count and return the ID of this semaphore control structure.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param count        [IN]  Initial number of available semaphores. The value range is [0, 1].
 * @param semHandle    [OUT] ID of the semaphore control structure that is initialized.
 *
 * @retval #LOS_ERRNO_SEM_PTR_NULL     The passed-in semHandle value is NULL.
 * @retval #LOS_ERRNO_SEM_OVERFLOW     The passed-in count value is greater than the maximum number of available
 * semaphores.
 * @retval #LOS_ERRNO_SEM_ALL_BUSY     No semaphore control structure is available.
 * @retval #LOS_OK   The semaphore is successfully created.
 * @par Dependency:
 * <ul><li>los_sem.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_SemDelete
 */
extern UINT32 LOS_BinarySemCreate(UINT16 count,
                                  UINT32 *semHandle);

/**
 * @ingroup los_sem
 * @brief Delete a semaphore.
 *
 * @par Description:
 * This API is used to delete a semaphore control structure that has an ID specified by semHandle.
 * @attention
 * <ul>
 * <li>The specified sem id must be created first. </li>
 * </ul>
 *
 * @param semHandle   [IN] ID of the semaphore control structure to be deleted. The ID of the semaphore
 * control structure is obtained from semaphore creation.
 *
 * @retval #LOS_ERRNO_SEM_INVALID  The passed-in semHandle value is invalid.
 * @retval #LOS_ERRNO_SEM_PENDED   The queue of the tasks that are waiting on the semaphore control structure is
 * not null.
 * @retval #LOS_OK   The semaphore control structure is successfully deleted.
 * @par Dependency:
 * <ul><li>los_sem.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_SemCreate
 */
extern UINT32 LOS_SemDelete(UINT32 semHandle);

/**
 * @ingroup los_sem
 * @brief Request a semaphore.
 *
 * @par Description:
 * This API is used to request a semaphore based on the semaphore control structure ID specified by semHandle and the
 * parameter that specifies the timeout period.
 * @attention
 * <ul>
 * <li>The specified sem id must be created first. </li>
 * </ul>
 *
 * @param semHandle   [IN] ID of the semaphore control structure to be requested. The ID of the semaphore control
 * structure is obtained from semaphore creation.
 * @param timeout     [IN] Timeout interval for waiting on the semaphore. The value range is [0, 0xFFFFFFFF].
 * If the value is set to 0, the semaphore is not waited on. If the value is set to 0xFFFFFFFF,
 * the semaphore is waited on forever(unit: Tick).
 *
 * @retval #LOS_ERRNO_SEM_INVALID          The passed-in semHandle value is invalid.
 * @retval #LOS_ERRNO_SEM_UNAVAILABLE      There is no available semaphore resource.
 * @retval #LOS_ERRNO_SEM_PEND_INTERR      The API is called during an interrupt, which is forbidden.
 * @retval #LOS_ERRNO_SEM_PEND_IN_LOCK     The task is unable to request a semaphore because task scheduling is locked.
 * @retval #LOS_ERRNO_SEM_TIMEOUT	 The request for the semaphore times out.
 * @retval #LOS_OK   The semaphore request succeeds.
 * @par Dependency:
 * <ul><li>los_sem.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_SemPost | LOS_SemCreate
 */
extern UINT32 LOS_SemPend(UINT32 semHandle,
                          UINT32 timeout);

/**
 * @ingroup los_sem
 * @brief Release a semaphore.
 *
 * @par Description:
 * This API is used to release a semaphore that has a semaphore control structure ID specified by semHandle.
 * @attention
 * <ul>
 * <li>The specified sem id must be created first. </li>
 * </ul>
 *
 * @param semHandle   [IN] ID of the semaphore control structure to be released.The ID of the semaphore control
 * structure is obtained from semaphore creation.
 *
 * @retval #LOS_ERRNO_SEM_INVALID      The passed-in semHandle value is invalid.
 * @retval #LOS_ERRNO_SEM_OVERFLOW     The times of semaphore release exceed the maximum times permitted.
 * @retval #LOS_OK                     The semaphore is successfully released.
 * @par Dependency:
 * <ul><li>los_sem.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_SemPend | LOS_SemCreate
 */
extern UINT32 LOS_SemPost(UINT32 semHandle);

extern UINT32 LOS_SemGetValue(UINT32 semHandle, INT32 *currVal);

/**
 * @ingroup los_sem
 * @brief Initialize the  Semaphore doubly linked list.
 *
 * @par Description:
 * This API is used to initialize the  Semaphore doubly linked list.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param None.
 *
 * @retval UINT32   Initialization result.
 * @par Dependency:
 * <ul><li>los_sem.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsSemInit(void);

/**
 * @ingroup los_sem
 * @brief Create Semaphore.
 *
 * @par Description:
 * This API is used to create Semaphore.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  count      [IN]Type  #UINT16 Semaphore count.
 * @param  maxCount   [IN]Type  #UINT16 Max semaphore count.
 * @param  semHandle  [OUT]Type #UINT32 * Index of semaphore.
 *
 * @retval UINT32   Create result.
 * @par Dependency:
 * <ul><li>los_sem.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsSemCreate(UINT16 count, UINT16 maxCount, UINT32 *semHandle);

/**
 * @ingroup los_swtmr
 * @brief Start a software timer.
 *
 * @par Description:
 * This API is used to start a software timer that has a specified ID.
 * @attention
 * <ul>
 * <li>The specific timer must be created first</li>
 * </ul>
 *
 * @param  swtmrID  [IN] Software timer ID created by LOS_SwtmrCreate.
 *
 * @retval #LOS_ERRNO_SWTMR_ID_INVALID       Invalid software timer ID.
 * @retval #LOS_ERRNO_SWTMR_NOT_CREATED      The software timer is not created.
 * @retval #LOS_ERRNO_SWTMR_STATUS_INVALID   Invalid software timer state.
 * @retval #LOS_OK                           The software timer is successfully started.
 * @par Dependency:
 * <ul><li>los_swtmr.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_SwtmrStop | LOS_SwtmrCreate
 */
extern UINT32 LOS_SwtmrStart(UINT32 swtmrID);

/**
 * @ingroup los_swtmr
 * @brief Stop a software timer.
 *
 * @par Description:
 * This API is used to stop a software timer that has a specified ID.
 * @attention
 * <ul>
 * <li>The specific timer should be created and started firstly.</li>
 * </ul>
 *
 * @param  swtmrID  [IN] Software timer ID created by LOS_SwtmrCreate.
 *
 * @retval #LOS_ERRNO_SWTMR_ID_INVALID       Invalid software timer ID.
 * @retval #LOS_ERRNO_SWTMR_NOT_CREATED      The software timer is not created.
 * @retval #LOS_ERRNO_SWTMR_NOT_STARTED      The software timer is not started.
 * @retval #LOS_ERRNO_SWTMR_STATUS_INVALID   Invalid software timer state.
 * @retval #LOS_OK                           The software timer is successfully stopped.
 * @par Dependency:
 * <ul><li>los_swtmr.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_SwtmrStart | LOS_SwtmrCreate
 */
extern UINT32 LOS_SwtmrStop(UINT32 swtmrID);

/**
 * @ingroup los_swtmr
 * @brief Obtain the number of remaining Ticks configured on a software timer.
 *
 * @par Description:
 * This API is used to obtain the number of remaining Ticks configured on the software timer of which the ID is
 * specified by usSwTmrID.
 * @attention
 * <ul>
 * <li>The specific timer should be created and started successfully, error happens otherwise.</li>
 * </ul>
 *
 * @param  swtmrID  [IN]  Software timer ID created by LOS_SwtmrCreate.
 * @param  tick     [OUT] Number of remaining Ticks configured on the software timer.
 *
 * @retval #LOS_ERRNO_SWTMR_ID_INVALID      Invalid software timer ID.
 * @retval #LOS_ERRNO_SWTMR_NOT_CREATED     The software timer is not created.
 * @retval #LOS_ERRNO_SWTMR_NOT_STARTED     The software timer is not started.
 * @retval #LOS_ERRNO_SWTMR_STATUS_INVALID  Invalid software timer state.
 * @retval #LOS_OK                          The number of remaining Ticks is successfully obtained.
 * @par Dependency:
 * <ul><li>los_swtmr.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_SwtmrCreate
 */
extern UINT32 LOS_SwtmrTimeGet(UINT32 swtmrID,
                               UINT32 *tick);

extern UINT32 LOS_SwtmrCreate(UINT32 interval,
                              UINT8 mode,
                              SWTMR_PROC_FUNC handler,
                              UINT32 *swtmrID,
                              UINT32 arg);

/**
 * @ingroup los_swtmr
 * @brief Delete a software timer.
 *
 * @par Description:
 * This API is used to delete a software timer.
 * @attention
 * <ul>
 * <li>The specific timer should be created and then stopped firstly.</li>
 * </ul>
 *
 * @param  swtmrID     [IN] Software timer ID created by LOS_SwtmrCreate.
 *
 * @retval #LOS_ERRNO_SWTMR_ID_INVALID        Invalid software timer ID.
 * @retval #LOS_ERRNO_SWTMR_NOT_CREATED       The software timer is not created.
 * @retval #LOS_ERRNO_SWTMR_STATUS_INVALID    Invalid software timer state.
 * @retval #LOS_OK                            The software timer is successfully deleted.
 * @par Dependency:
 * <ul><li>los_swtmr.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_SwtmrCreate
 */
extern UINT32 LOS_SwtmrDelete(UINT32 swtmrID);

/**
 * @ingroup los_swtmr
 * @brief Initialization software timer.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to initialization software.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_swtmr.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsSwtmrInit(void);

/**
 * @ingroup los_swtmr
 * @brief Get next timeout.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to get next timeout.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_swtmr.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsSwtmrGetNextTimeout(void);

extern void OsSwtmrResponseTimeReset(UINT64 startTime);

extern void __stack_chk_fail(void);

/**
 * @ingroup  los_interrupt
 * @brief Delete hardware interrupt.
 *
 * @par Description:
 * This API is used to delete hardware interrupt.
 *
 * @attention
 * <ul>
 * <li>The hardware interrupt module is usable only when the configuration item for hardware
 * interrupt tailoring is enabled.</li>
 * <li>Hardware interrupt number value range: [OS_USER_HWI_MIN,OS_USER_HWI_MAX]. The value range
 * applicable for a Cortex-A7 platform is [32,95].</li>
 * <li>OS_HWI_MAX_NUM specifies the maximum number of interrupts that can be created.</li>
 * <li>Before executing an interrupt on a platform, refer to the chip manual of the platform.</li>
 * </ul>
 *
 * @param  hwiNum   [IN] Type#HWI_HANDLE_T: hardware interrupt number. The value range applicable
 *                       for a Cortex-A7 platform is [32,95].
 * @param  irqParam [IN] Type #HwiIrqParam *. ID of hardware interrupt which will base on
 *                                                when delete the hardware interrupt.
 * @retval #OS_ERRNO_HWI_NUM_INVALID              0x02000900: Invalid interrupt number.
 * @retval #LOS_OK                                0         : The interrupt is successfully delete.
 * @par Dependency:
 * <ul><li>los_interrupt.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 ArchHwiDelete(HWI_HANDLE_T hwiNum, HwiIrqParam *irqParam);

/**
 * @ingroup  los_interrupt
 * @brief Create a hardware interrupt.
 *
 * @par Description:
 * This API is used to configure a hardware interrupt and register a hardware interrupt handling function.
 *
 * @attention
 * <ul>
 * <li>The hardware interrupt module is usable only when the configuration item for hardware
 * interrupt tailoring is enabled.</li>
 * <li>Hardware interrupt number value range: [OS_USER_HWI_MIN,OS_USER_HWI_MAX]. The value range
 * applicable for a Cortex-A7 platform is [32,95].</li>
 * <li>OS_HWI_MAX_NUM specifies the maximum number of interrupts that can be created.</li>
 * <li>Before executing an interrupt on a platform, refer to the chip manual of the platform.</li>
 * </ul>
 *
 * @param  hwiNum   [IN] Type#HWI_HANDLE_T: hardware interrupt number. The value range applicable for a
 *                       Cortex-A7 platform is [32,95].
 * @param  hwiPrio  [IN] Type#HWI_PRIOR_T: hardware interrupt priority. Ignore this parameter temporarily.
 * @param  mode     [IN] Type#HWI_MODE_T: hardware interrupt mode. Ignore this parameter temporarily.
 * @param  handler  [IN] Type#HWI_PROC_FUNC: interrupt handler used when a hardware interrupt is triggered.
 * @param  irqParam [IN] Type#HwiIrqParam: input parameter of the interrupt
 *                                         handler used when a hardware interrupt is triggered.
 *
 * @retval #OS_ERRNO_HWI_PROC_FUNC_NULL               0x02000901: Null hardware interrupt handling function.
 * @retval #OS_ERRNO_HWI_NUM_INVALID                  0x02000900: Invalid interrupt number.
 * @retval #OS_ERRNO_HWI_NO_MEMORY                    0x02000903: Insufficient memory for hardware interrupt creation.
 * @retval #OS_ERRNO_HWI_ALREADY_CREATED              0x02000904: The interrupt handler being created has
 *                                                                already been created.
 * @retval #LOS_OK                                    0         : The interrupt is successfully created.
 * @par Dependency:
 * <ul><li>los_interrupt.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 ArchHwiCreate(HWI_HANDLE_T hwiNum,
                            HWI_PRIOR_T hwiPrio,
                            HWI_MODE_T mode,
                            HWI_PROC_FUNC handler,
                            HwiIrqParam *irqParam);

extern UINT32 ArchIsIntActive(void);

extern UINT32 ArchIntLock(void);

extern UINT32 ArchIntUnLock(void);

extern void ArchIntRestore(UINT32 intSave);

extern UINT32 ArchIntTrigger(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntEnable(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntDisable(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntClear(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntSetPriority(HWI_HANDLE_T hwiNum, HWI_PRIOR_T priority);

extern UINT32 ArchIntCurIrqNum(void);

extern struct HwiControllerOps *ArchIntOpsGet(void);

extern void __stack_chk_fail(void);

/**
 * @ingroup  los_interrupt
 * @brief Delete hardware interrupt.
 *
 * @par Description:
 * This API is used to delete hardware interrupt.
 *
 * @attention
 * <ul>
 * <li>The hardware interrupt module is usable only when the configuration item for hardware
 * interrupt tailoring is enabled.</li>
 * <li>Hardware interrupt number value range: [OS_USER_HWI_MIN,OS_USER_HWI_MAX]. The value range
 * applicable for a Cortex-A7 platform is [32,95].</li>
 * <li>OS_HWI_MAX_NUM specifies the maximum number of interrupts that can be created.</li>
 * <li>Before executing an interrupt on a platform, refer to the chip manual of the platform.</li>
 * </ul>
 *
 * @param  hwiNum   [IN] Type#HWI_HANDLE_T: hardware interrupt number. The value range applicable
 *                       for a Cortex-A7 platform is [32,95].
 * @param  irqParam [IN] Type #HwiIrqParam *. ID of hardware interrupt which will base on
 *                                                when delete the hardware interrupt.
 * @retval #OS_ERRNO_HWI_NUM_INVALID              0x02000900: Invalid interrupt number.
 * @retval #LOS_OK                                0         : The interrupt is successfully delete.
 * @par Dependency:
 * <ul><li>los_interrupt.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 ArchHwiDelete(HWI_HANDLE_T hwiNum, HwiIrqParam *irqParam);

/**
 * @ingroup  los_interrupt
 * @brief Create a hardware interrupt.
 *
 * @par Description:
 * This API is used to configure a hardware interrupt and register a hardware interrupt handling function.
 *
 * @attention
 * <ul>
 * <li>The hardware interrupt module is usable only when the configuration item for hardware
 * interrupt tailoring is enabled.</li>
 * <li>Hardware interrupt number value range: [OS_USER_HWI_MIN,OS_USER_HWI_MAX]. The value range
 * applicable for a Cortex-A7 platform is [32,95].</li>
 * <li>OS_HWI_MAX_NUM specifies the maximum number of interrupts that can be created.</li>
 * <li>Before executing an interrupt on a platform, refer to the chip manual of the platform.</li>
 * </ul>
 *
 * @param  hwiNum   [IN] Type#HWI_HANDLE_T: hardware interrupt number. The value range applicable for a
 *                       Cortex-A7 platform is [32,95].
 * @param  hwiPrio  [IN] Type#HWI_PRIOR_T: hardware interrupt priority. Ignore this parameter temporarily.
 * @param  mode     [IN] Type#HWI_MODE_T: hardware interrupt mode. Ignore this parameter temporarily.
 * @param  handler  [IN] Type#HWI_PROC_FUNC: interrupt handler used when a hardware interrupt is triggered.
 * @param  irqParam [IN] Type#HwiIrqParam: input parameter of the interrupt
 *                                         handler used when a hardware interrupt is triggered.
 *
 * @retval #OS_ERRNO_HWI_PROC_FUNC_NULL               0x02000901: Null hardware interrupt handling function.
 * @retval #OS_ERRNO_HWI_NUM_INVALID                  0x02000900: Invalid interrupt number.
 * @retval #OS_ERRNO_HWI_NO_MEMORY                    0x02000903: Insufficient memory for hardware interrupt creation.
 * @retval #OS_ERRNO_HWI_ALREADY_CREATED              0x02000904: The interrupt handler being created has
 *                                                                already been created.
 * @retval #LOS_OK                                    0         : The interrupt is successfully created.
 * @par Dependency:
 * <ul><li>los_interrupt.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 ArchHwiCreate(HWI_HANDLE_T hwiNum,
                            HWI_PRIOR_T hwiPrio,
                            HWI_MODE_T mode,
                            HWI_PROC_FUNC handler,
                            HwiIrqParam *irqParam);

extern UINT32 ArchIsIntActive(void);

extern UINT32 ArchIntLock(void);

extern UINT32 ArchIntUnLock(void);

extern void ArchIntRestore(UINT32 intSave);

extern UINT32 ArchIntTrigger(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntEnable(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntDisable(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntClear(HWI_HANDLE_T hwiNum);

extern UINT32 ArchIntSetPriority(HWI_HANDLE_T hwiNum, HWI_PRIOR_T priority);

extern UINT32 ArchIntCurIrqNum(void);

extern struct HwiControllerOps *ArchIntOpsGet(void);

/**
 * @ingroup  los_context
 * @brief: Task stack initialization.
 *
 * @par Description:
 * This API is used to initialize the task stack.
 *
 * @attention:
 * <ul><li>None.</li></ul>
 *
 * @param  taskID     [IN] Type#UINT32: TaskID.
 * @param  stackSize  [IN] Type#UINT32: Total size of the stack.
 * @param  topStack   [IN] Type#VOID *: Top of task's stack.
 *
 * @retval: context Type#TaskContext *.
 * @par Dependency:
 * <ul><li>los_context.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern void *ArchTskStackInit(UINT32 taskID, UINT32 stackSize, void *topStack);

/**
 * @ingroup  los_context
 * @brief: Function to sys exit.
 *
 * @par Description:
 * This API is used to sys exit.
 *
 * @attention:
 * <ul><li>None.</li></ul>
 *
 * @param  None.
 *
 * @retval: None.
 * @par Dependency:
 * <ul><li>los_context.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern void ArchSysExit(void);

/**
 * @ingroup  los_context
 * @brief: Task scheduling Function.
 *
 * @par Description:
 * This API is used to scheduling task.
 *
 * @attention:
 * <ul><li>None.</li></ul>
 *
 * @param  None.
 *
 * @retval: None.
 * @par Dependency:
 * <ul><li>los_context.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern void ArchTaskSchedule(void);

extern UINT32 ArchStartSchedule(void);

extern void *ArchSignalContextInit(void *stackPointer,
                                   void *stackTop,
                                   UINTPTR sigHandler,
                                   UINT32 param);

/**
 * @ingroup los_event
 * @brief Initialize an event control block.
 *
 * @par Description:
 * This API is used to initialize the event control block pointed to by eventCB.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param eventCB [IN/OUT] Pointer to the event control block to be initialized.
 *
 * @retval #LOS_ERRNO_EVENT_PTR_NULL  Null pointer.
 * @retval #LOS_OK                    The event control block is successfully initialized.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventClear
 */
extern UINT32 LOS_EventInit(PEVENT_CB_S eventCB);

/**
 * @ingroup los_event
 * @brief Obtain an event specified by the event ID.
 *
 * @par Description:
 * This API is used to check whether an event expected by the user occurs according to the event ID, event mask,
 * and event reading mode, and process the event based on the event reading mode. The event ID must point to
 * valid memory.
 * @attention
 * <ul>
 * <li>When the mode is LOS_WAITMODE_CLR, the eventID is passed-out.</li>
 * <li>Otherwise the eventID is passed-in.</li>
 * </ul>
 *
 * @param eventID      [IN/OUT] Pointer to the ID of the event to be checked.
 * @param eventMask    [IN] Mask of the event expected to occur by the user, indicating the event obtained after
 * it is logically processed that matches the ID pointed to by mode.
 * @param mode         [IN] Event reading mode. The modes include LOS_WAITMODE_AND, LOS_WAITMODE_OR, LOS_WAITMODE_CLR.
 *
 * @retval 0             The event expected by the user does not occur.
 * @retval #UINT32       The event expected by the user occurs.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventRead | LOS_EventWrite
 */
extern UINT32 LOS_EventPoll(UINT32 *eventID,
                            UINT32 eventMask,
                            UINT32 mode);

/**
 * @ingroup los_event
 * @brief Read an event.
 *
 * @par Description:
 * This API is used to block or schedule a task that reads an event of which the event control block, event mask,
 * reading mode,
 * and timeout information are specified.
 * </ul>
 * @attention
 * <ul>
 * <li>An error code and an event return value can be same. To differentiate the error code and return value, bit 25 of
 * the event mask is forbidden to be used.</li>
 * </ul>
 *
 * @param eventCB      [IN/OUT] Pointer to the event control block to be checked. This parameter must point to
 *                              valid memory.
 * @param eventMask    [IN]     Mask of the event expected to occur by the user, indicating the event obtained after
 *                              it is logically processed that matches the ID pointed to by eventID.
 * @param mode         [IN]     Event reading mode.
 * @param timeOut      [IN]     Timeout interval of event reading (unit: Tick).
 *
 * @retval #LOS_ERRNO_EVENT_SETBIT_INVALID     Bit 25 of the event mask cannot be set because
 *                                             it is set to an error number.
 * @retval #LOS_ERRNO_EVENT_EVENTMASK_INVALID  The passed-in event reading mode is incorrect.
 * @retval #LOS_ERRNO_EVENT_READ_IN_INTERRUPT  The event is being read during an interrupt.
 * @retval #LOS_ERRNO_EVENT_FLAGS_INVALID      The event mode is invalid.
 * @retval #LOS_ERRNO_EVENT_READ_IN_LOCK       The event reading task is locked.
 * @retval #LOS_ERRNO_EVENT_PTR_NULL           The passed-in pointer is null.
 * @retval 0                                   The event expected by the user does not occur.
 * @retval #UINT32                             The event expected by the user occurs.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventPoll | LOS_EventWrite
 */
extern UINT32 LOS_EventRead(PEVENT_CB_S eventCB,
                            UINT32 eventMask,
                            UINT32 mode,
                            UINT32 timeOut);

/**
 * @ingroup los_event
 * @brief Write an event.
 *
 * @par Description:
 * This API is used to write an event specified by the passed-in event mask into an event control block
 * pointed to by eventCB.
 * @attention
 * <ul>
 * <li>To determine whether the LOS_EventRead API returns an event or an error code, bit 25 of the event mask
 * is forbidden to be used.</li>
 * </ul>
 *
 * @param eventCB  [IN/OUT] Pointer to the event control block into which an event is to be written.
 * This parameter must point to valid memory.
 * @param events   [IN] Event mask to be written.
 *
 * @retval #LOS_ERRNO_EVENT_SETBIT_INVALID  Bit 25 of the event mask cannot be set to an event
 * because it is set to an error code.
 * @retval #LOS_ERRNO_EVENT_PTR_NULL        Null pointer.
 * @retval #LOS_OK                          The event is successfully written.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventPoll | LOS_EventRead
 */
extern UINT32 LOS_EventWrite(PEVENT_CB_S eventCB,
                             UINT32 events);

/**
 * @ingroup los_event
 * @brief Clear the event of the eventCB by a specified eventMask.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to set the ID of an event that has a specified mask and of which the information is stored in
 * an event control block pointed to by eventCB to 0. eventCB must point to valid memory.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The value of events needs to be reversed when it is passed-in.</li>
 * </ul>
 *
 * @param eventCB     [IN/OUT] Pointer to the event control block to be cleared.
 * @param eventMask   [IN]     Mask of the event to be cleared.
 *
 * @retval #LOS_ERRNO_EVENT_PTR_NULL  Null pointer.
 * @retval #LOS_OK                    The event is successfully cleared.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventPoll | LOS_EventRead | LOS_EventWrite
 */
extern UINT32 LOS_EventClear(PEVENT_CB_S eventCB,
                             UINT32 eventMask);

/**
 * @ingroup los_event
 * @brief Destroy an event.
 *
 * @par Description:
 * <ul>
 * <li>This API is used to Destroy an event.</li>
 * </ul>
 * @attention
 * <ul>
 * <li>The specific event should be a valid one.</li>
 * </ul>
 *
 * @param eventCB     [IN/OUT] Pointer to the event control block to be Destroyed.
 *
 * @retval #LOS_ERRNO_EVENT_PTR_NULL Null pointer.
 * @retval #LOS_OK                   The event is successfully cleared.
 * @par Dependency:
 * <ul><li>los_event.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_EventPoll | LOS_EventRead | LOS_EventWrite
 */
extern UINT32 LOS_EventDestroy(PEVENT_CB_S eventCB);

extern UINT32 OsEventReadOnce(PEVENT_CB_S eventCB, UINT32 eventMask, UINT32 mode, UINT32 timeOut);

extern UINT32 OsEventWriteOnce(PEVENT_CB_S eventCB, UINT32 events);

/**
 * @ingroup los_err
 * @brief Error handling function.
 *
 * @par Description:
 * This API is used to perform different operations according to error types.
 * @attention
 * <ul>
 * <li>None</li>
 * </ul>
 *
 * @param  fileName  [IN] Log file that stores error information.
 * @param  lineNo    [IN] Line number of the erroneous line which should not be OS_ERR_MAGIC_WORD.
 * @param  errorNo   [IN] Error code.
 * @param  paraLen   [IN] Length of the input parameter pPara.
 * @param  para      [IN] User label of the error.
 *
 * @retval LOS_OK The error is successfully processed.
 * @par Dependency:
 * <ul><li>los_err.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern UINT32 LOS_ErrHandle(CHAR *fileName,
                            UINT32 lineNo,
                            UINT32 errorNo,
                            UINT32 paraLen,
                            void *para);

extern UINT32 ArchEnterSleep(void);

/**
 * @ingroup los_timer
 * @brief Get tick timer control block.
 *
 * @par Description:
 * This API is used to get tick timer control block.
 *
 * @param  None
 *
 * @retval #tick timer control block
 * @par Dependency:
 * <ul><li>los_timer.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern struct ArchTickTimer *ArchSysTickTimerGet(void);

/**
 *  @ingroup  los_tick
 *  @brief: System timer cycles get function.
 *
 *  @par Description:
 *  This API is used to get system timer cycles.
 *
 * @attention:
 * <ul><li>None.</li></ul>
 *
 * @param: None.
 *
 * @retval: current system cycles.
 *
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 *
 */
extern UINT64 LOS_SysCycleGet(void);

extern UINT64 OsTickTimerReload(UINT64 period);

extern void OsTickTimerBaseReset(UINT64 currTime);

extern UINT32 OsTickTimerInit(void);

extern void OsTickSysTimerStartTimeSet(UINT64 currTime);

/**
 * @ingroup los_tick
 * @brief Adjust the system tick timer clock frequency.
 *
 * @par Description:
 * This API is used to adjust the system tick timer clock frequency.
 * @attention
 * <ul>
 * <li>This function needs to be invoked only when the clock frequency of the system tick timer adjust as a result of
 * changing the CPU frequency.</li>
 * </ul>
 *
 * @param  handler [IN] Adjust the system tick timer clock frequency function hooks.
 * @param param   [IN] Function parameters.
 *
 * @retval LOS_OK or Error code.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern UINT32 LOS_SysTickClockFreqAdjust(SYS_TICK_FREQ_ADJUST_FUNC handler,
                                         UINTPTR param);

/**
 * @ingroup los_tick
 * @brief Obtain the number of Ticks.
 *
 * @par Description:
 * This API is used to obtain the number of Ticks.
 * @attention
 * <ul>
 * <li>None</li>
 * </ul>
 *
 * @param  None
 *
 * @retval UINT64 The number of Ticks.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern UINT64 LOS_TickCountGet(void);

/**
 * @ingroup los_tick
 * @brief Obtain the number of cycles in one second.
 *
 * @par Description:
 * This API is used to obtain the number of cycles in one second.
 * @attention
 * <ul>
 * <li>None</li>
 * </ul>
 *
 * @param  None
 *
 * @retval UINT32 Number of cycles obtained in one second.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern UINT32 LOS_CyclePerTickGet(void);

/**
 * @ingroup los_tick
 * @brief Convert Ticks to milliseconds.
 *
 * @par Description:
 * This API is used to convert Ticks to milliseconds.
 * @attention
 * <ul>
 * <li>The number of milliseconds obtained through the conversion is 32-bit.</li>
 * </ul>
 *
 * @param  ticks  [IN] Number of Ticks. The value range is (0,OS_SYS_CLOCK).
 *
 * @retval UINT32 Number of milliseconds obtained through the conversion. Ticks are successfully converted to
 * milliseconds.
 * @par  Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_MS2Tick
 */
extern UINT32 LOS_Tick2MS(UINT32 ticks);

/**
 * @ingroup los_tick
 * @brief Convert milliseconds to Ticks.
 *
 * @par Description:
 * This API is used to convert milliseconds to Ticks.
 * @attention
 * <ul>
 * <li>If the parameter passed in is equal to 0xFFFFFFFF, the retval is 0xFFFFFFFF. Pay attention to the value to be
 * converted because data possibly overflows.</li>
 * </ul>
 *
 * @param  millisec  [IN] Number of milliseconds.
 *
 * @retval UINT32 Number of Ticks obtained through the conversion.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_Tick2MS
 */
extern UINT32 LOS_MS2Tick(UINT32 millisec);

/**
 * @ingroup los_tick
 * @brief Re-initializes the system tick timer.
 *
 * @par Description:
 * This API is used to re-initialize the system Tick timer.
 * @attention
 *
 * @param timer        [IN] Specify the tick timer.
 * @param tickHandler  [IN] Tick Interrupts the execution of the hook function.
 *
 * @retval LOS_OK or Error code.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TickTimerRegister(const struct ArchTickTimer *timer, HWI_PROC_FUNC tickHandler);

extern void LOS_UDelay(UINT64 microseconds);

extern void LOS_MDelay(UINT32 millisec);

extern UINT64 LOS_CurrNanosec(void);

/**
 * @ingroup  los_tick
 * @brief Handle the system tick timeout.
 *
 * @par Description:
 * This API is called when the system tick timeout and triggers the interrupt.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param none.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern void OsTickHandler(void);

/**
 * @ingroup los_tick
 * @brief Convert cycles to milliseconds.
 *
 * @par Description:
 * This API is used to convert cycles to milliseconds.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  cpuTick  [IN]  Number of CPU cycles.
 * @param  msHi     [OUT] Upper 32 bits of the number of milliseconds.
 * @param  msLo     [OUT] Lower 32 bits of the number of milliseconds.
 *
 * @retval #LOS_ERRNO_SYS_PTR_NULL    0x02000011: Invalid parameter.
 * @retval #LOS_OK                   0:  Cycles are successfully converted to microseconds.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsCpuTick2MS(CpuTick *cpuTick, UINT32 *msHi, UINT32 *msLo);

/**
 * @ingroup los_tick
 * @brief Convert cycles to microseconds.
 *
 * @par Description:
 * This API is used to convert cycles to microseconds.
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  cpuTick  [IN]  Number of CPU cycles.
 * @param  usHi     [OUT] Upper 32 bits of the number of microseconds.
 * @param  usLo     [OUT] Lower 32 bits of the number of microseconds.
 *
 * @retval #LOS_ERRNO_SYS_PTR_NULL    0x02000011: Invalid parameter.
 * @retval #LOS_OK                   0: Cycles are successfully converted to microseconds.
 * @par Dependency:
 * <ul><li>los_tick.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsCpuTick2US(CpuTick *cpuTick, UINT32 *usHi, UINT32 *usLo);

extern struct SortLinkAttribute *OsGetSortLinkAttribute(SortLinkType type_);

extern UINT32 OsSortLinkInit(struct SortLinkAttribute *sortLinkHead);

extern void OsAdd2SortLink(struct SortLinkList *node,
                           UINT64 startTime,
                           UINT32 waitTicks,
                           SortLinkType type_);

extern void OsDeleteSortLink(struct SortLinkList *node);

extern UINT64 OsSortLinkGetTargetExpireTime(UINT64 currTime,
                                            const struct SortLinkList *targetSortList);

extern UINT64 OsSortLinkGetNextExpireTime(const struct SortLinkAttribute *sortLinkHead);

extern void OsSortLinkResponseTimeConvertFreq(UINT32 oldFreq);

/**
 * @ingroup los_task
 * @brief Sleep the current task.
 *
 * @par Description:
 * This API is used to delay the execution of the current task. The task is able to be scheduled
 * after it is delayed for a specified number of Ticks.
 *
 * @attention
 * <ul>
 * <li>The task fails to be delayed if it is being delayed during interrupt processing or it is locked.</li>
 * <li>If 0 is passed in and the task scheduling is not locked,
 * execute the next task in the queue of tasks with the priority of the current task.
 * If no ready task with the priority of the current task is available,
 * the task scheduling will not occur, and the current task continues to be executed.</li>
 * <li>The parameter passed in can not be equal to LOS_WAIT_FOREVER(0xFFFFFFFF).
 * If that happens, the task will not sleep 0xFFFFFFFF milliseconds or sleep forever but sleep 0xFFFFFFFF Ticks.</li>
 * </ul>
 *
 * @param mSecs [IN] Type #UINT32 Number of MS for which the task is delayed.
 *
 * @retval None
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern void LOS_Msleep(UINT32 mSecs);

/**
 * @ingroup los_task
 * @brief System kernel initialization function.
 *
 * @par Description:
 * This API is used to start liteOS .
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param: None.
 *
 * @retval #LOS_OK                                  0:LiteOS start success.
 *
 * @par Dependency:
 * <ul><li>los_config.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_Start(void);

extern void LOS_Reboot(void);

extern void LOS_Panic(const CHAR *fmt);

/**
 * @ingroup los_task
 * @brief System kernel initialization function.
 *
 * @par Description:
 * This API is used to Initialize kernel ,configure all system modules.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param: None.
 *
 * @retval #LOS_OK                                  0:System kernel initialization success.
 *
 * @par Dependency:
 * <ul><li>los_config.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_KernelInit(void);

/**
 * @ingroup  los_task
 * @brief Create a task and suspend.
 *
 * @par Description:
 * This API is used to create a task and suspend it. This task will not be added to the queue of ready tasks before
 * resume it.
 *
 * @attention
 * <ul>
 * <li>During task creation, the task control block and task stack of the task that is previously automatically deleted
 * are deallocated.</li>
 * <li>The task name is a pointer and is not allocated memory.</li>
 * <li>If the size of the task stack of the task to be created is 0, configure #LOSCFG_BASE_CORE_TSK_DEFAULT_STACK_SIZE
 * to specify the default task stack size. The stack size should be a reasonable value, if the size is too large, may
 * cause memory exhaustion.</li>
 * <li>The task stack size must be aligned on the boundary of 8 bytes. The size is determined by whether it is big
 * enough to avoid task stack overflow.</li>
 * <li>Less parameter value indicates higher task priority.</li>
 * <li>The task name cannot be null.</li>
 * <li>The pointer to the task executing function cannot be null.</li>
 * <li>The two parameters of this interface is pointer, it should be a correct value, otherwise, the system may be
 * abnormal.</li>
 * <li>If user mode is enabled, user should input user stack pointer and size, the size must fit the stack pointer,
 * uwStackSize remain as the kernel stack size.</li>
 * </ul>
 *
 * @param  taskID        [OUT] Type  #UINT32 * Task ID.
 * @param  taskInitParam [IN]  Type  #TSK_INIT_PARAM_S * Parameter for task creation.
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID        Invalid Task ID, param puwTaskID is NULL.
 * @retval #LOS_ERRNO_TSK_PTR_NULL          Param pstInitParam is NULL.
 * @retval #LOS_ERRNO_TSK_NAME_EMPTY        The task name is NULL.
 * @retval #LOS_ERRNO_TSK_ENTRY_NULL        The task entrance is NULL.
 * @retval #LOS_ERRNO_TSK_PRIOR_ERROR       Incorrect task priority.
 * @retval #LOS_ERRNO_TSK_STKSZ_TOO_LARGE   The task stack size is too large.
 * @retval #LOS_ERRNO_TSK_STKSZ_TOO_SMALL   The task stack size is too small.
 * @retval #LOS_ERRNO_TSK_TCB_UNAVAILABLE   No free task control block is available.
 * @retval #LOS_ERRNO_TSK_NO_MEMORY         Insufficient memory for task creation.
 * @retval #LOS_OK                          The task is successfully created.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * <ul><li>los_config.h: the header file that contains system configuration items.</li></ul>
 * @see LOS_TaskDelete
 */
extern UINT32 LOS_TaskCreateOnly(UINT32 *taskID,
                                 TSK_INIT_PARAM_S *taskInitParam);

/**
 * @ingroup  los_task
 * @brief Create a task.
 *
 * @par Description:
 * This API is used to create a task. If the priority of the task created after system initialized is higher than
 * the current task and task scheduling is not locked, it is scheduled for running.
 * If not, the created task is added to the queue of ready tasks.
 *
 * @attention
 * <ul>
 * <li>During task creation, the task control block and task stack of the task that is previously automatically
 * deleted are deallocated.</li>
 * <li>The task name is a pointer and is not allocated memory.</li>
 * <li>If the size of the task stack of the task to be created is 0, configure #LOSCFG_BASE_CORE_TSK_DEFAULT_STACK_SIZE
 * to specify the default task stack size.</li>
 * <li>The task stack size must be aligned on the boundary of 8 bytes. The size is determined by whether it is big
 * enough to avoid task stack overflow.</li>
 * <li>Less parameter value indicates higher task priority.</li>
 * <li>The task name cannot be null.</li>
 * <li>The pointer to the task executing function cannot be null.</li>
 * <li>The two parameters of this interface is pointer, it should be a correct value, otherwise, the system may be
 * abnormal.</li>
 * <li>If user mode is enabled, user should input user stack pointer and size, the size must fit the stack pointer,
 * uwStackSize remain as the kernel stack size.</li>
 * </ul>
 *
 * @param  taskID        [OUT] Type  #UINT32 * Task ID.
 * @param  taskInitParam [IN]  Type  #TSK_INIT_PARAM_S * Parameter for task creation.
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID        Invalid Task ID, param puwTaskID is NULL.
 * @retval #LOS_ERRNO_TSK_PTR_NULL          Param pstInitParam is NULL.
 * @retval #LOS_ERRNO_TSK_NAME_EMPTY        The task name is NULL.
 * @retval #LOS_ERRNO_TSK_ENTRY_NULL        The task entrance is NULL.
 * @retval #LOS_ERRNO_TSK_PRIOR_ERROR       Incorrect task priority.
 * @retval #LOS_ERRNO_TSK_STKSZ_TOO_LARGE   The task stack size is too large.
 * @retval #LOS_ERRNO_TSK_STKSZ_TOO_SMALL   The task stack size is too small.
 * @retval #LOS_ERRNO_TSK_TCB_UNAVAILABLE   No free task control block is available.
 * @retval #LOS_ERRNO_TSK_NO_MEMORY         Insufficient memory for task creation.
 * @retval #LOS_OK                          The task is successfully created.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * <ul><li>los_config.h: the header file that contains system configuration items.</li></ul>
 * @see LOS_TaskDelete
 */
extern UINT32 LOS_TaskCreate(UINT32 *taskID,
                             TSK_INIT_PARAM_S *taskInitParam);

/**
 * @ingroup  los_task
 * @brief Resume a task.
 *
 * @par Description:
 * This API is used to resume a suspended task.
 *
 * @attention
 * <ul>
 * <li>If the task is delayed or blocked, resume the task without adding it to the queue of ready tasks.</li>
 * <li>If the priority of the task resumed after system initialized is higher than the current task and task scheduling
 * is not locked, it is scheduled for running.</li>
 * </ul>
 *
 * @param  taskID [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID        Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED       The task is not created.
 * @retval #LOS_ERRNO_TSK_NOT_SUSPENDED     The task is not suspended.
 * @retval #LOS_OK                          The task is successfully resumed.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskSuspend
 */
extern UINT32 LOS_TaskResume(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Suspend a task.
 *
 * @par Description:
 * This API is used to suspend a specified task, and the task will be removed from the queue of ready tasks.
 *
 * @attention
 * <ul>
 * <li>The task that is running and locked cannot be suspended.</li>
 * <li>The idle task and swtmr task cannot be suspended.</li>
 * </ul>
 *
 * @param  taskID [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 *
 * @retval #LOS_ERRNO_TSK_OPERATE_IDLE                  Check the task ID and do not operate on the idle task.
 * @retval #LOS_ERRNO_TSK_SUSPEND_SWTMR_NOT_ALLOWED     Check the task ID and do not operate on the swtmr task.
 * @retval #LOS_ERRNO_TSK_ID_INVALID                    Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED                   The task is not created.
 * @retval #LOS_ERRNO_TSK_ALREADY_SUSPENDED             The task is already suspended.
 * @retval #LOS_ERRNO_TSK_SUSPEND_LOCKED                The task being suspended is current task and task scheduling
 *                                                      is locked.
 * @retval #LOS_OK                                      The task is successfully suspended.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskResume
 */
extern UINT32 LOS_TaskSuspend(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Delete a task.
 *
 * @par Description:
 * This API is used to delete a specified task and release the resources for its task stack and task control block.
 *
 * @attention
 * <ul>
 * <li>The idle task and swtmr task cannot be deleted.</li>
 * <li>If delete current task maybe cause unexpected error.</li>
 * <li>If a task get a mutex is deleted or automatically deleted before release this mutex, other tasks pended
 * this mutex maybe never be scheduled.</li>
 * </ul>
 *
 * @param  taskID [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 *
 * @retval #LOS_ERRNO_TSK_OPERATE_IDLE                  Check the task ID and do not operate on the idle task.
 * @retval #LOS_ERRNO_TSK_SUSPEND_SWTMR_NOT_ALLOWED     Check the task ID and do not operate on the swtmr task.
 * @retval #LOS_ERRNO_TSK_ID_INVALID                    Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED                   The task is not created.
 * @retval #LOS_OK                                      The task is successfully deleted.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskCreate | LOS_TaskCreateOnly
 */
extern UINT32 LOS_TaskDelete(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Delay a task.
 *
 * @par Description:
 * This API is used to delay the execution of the current task. The task is able to be scheduled after it is delayed
 * for a specified number of Ticks.
 *
 * @attention
 * <ul>
 * <li>The task fails to be delayed if it is being delayed during interrupt processing or it is locked.</li>
 * <li>If 0 is passed in and the task scheduling is not locked, execute the next task in the queue of tasks with
 * the same priority of the current task.
 * If no ready task with the priority of the current task is available, the task scheduling will not occur, and the
 * current task continues to be executed.</li>
 * <li>Using the interface before system initialized is not allowed.</li>
 * </ul>
 *
 * @param  tick [IN] Type #UINT32 Number of Ticks for which the task is delayed.
 *
 * @retval #LOS_ERRNO_TSK_DELAY_IN_INT              The task delay occurs during an interrupt.
 * @retval #LOS_ERRNO_TSK_DELAY_IN_LOCK             The task delay occurs when the task scheduling is locked.
 * @retval #LOS_ERRNO_TSK_ID_INVALID                Invalid Task ID
 * @retval #LOS_ERRNO_TSK_YIELD_NOT_ENOUGH_TASK     No tasks with the same priority is available for scheduling.
 * @retval #LOS_OK                                  The task is successfully delayed.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskDelay(UINT32 tick);

/**
 * @ingroup  los_task
 * @brief Lock the task scheduling.
 *
 * @par Description:
 * This API is used to lock the task scheduling. Task switching will not occur if the task scheduling is locked.
 *
 * @attention
 * <ul>
 * <li>If the task scheduling is locked, but interrupts are not disabled, tasks are still able to be interrupted.</li>
 * <li>One is added to the number of task scheduling locks if this API is called. The number of locks is decreased by
 * one if the task scheduling is unlocked. Therefore, this API should be used together with LOS_TaskUnlock.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskUnlock
 */
extern void LOS_TaskLock(void);

/**
 * @ingroup  los_task
 * @brief Unlock the task scheduling.
 *
 * @par Description:
 * This API is used to unlock the task scheduling. Calling this API will decrease the number of task locks by one.
 * If a task is locked more than once, the task scheduling will be unlocked only when the number of locks becomes zero.
 *
 * @attention
 * <ul>
 * <li>The number of locks is decreased by one if this API is called. One is added to the number of task scheduling
 * locks if the task scheduling is locked. Therefore, this API should be used together with LOS_TaskLock.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskLock
 */
extern void LOS_TaskUnlock(void);

/**
 * @ingroup  los_task
 * @brief Set a task priority.
 *
 * @par Description:
 * This API is used to set the priority of a specified task.
 *
 * @attention
 * <ul>
 * <li>If the set priority is higher than the priority of the current running task, task scheduling probably occurs.
 * </li>
 * <li>Changing the priority of the current running task also probably causes task scheduling.</li>
 * <li>Using the interface to change the priority of software timer task and idle task is not allowed.</li>
 * <li>Using the interface in the interrupt is not allowed.</li>
 * </ul>
 *
 * @param  taskID   [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 * @param  taskPrio [IN] Type #UINT16 Task priority.
 *
 * @retval #LOS_ERRNO_TSK_PRIOR_ERROR    Incorrect task priority.Re-configure the task priority
 * @retval #LOS_ERRNO_TSK_OPERATE_IDLE   Check the task ID and do not operate on the idle task.
 * @retval #LOS_ERRNO_TSK_ID_INVALID     Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED    The task is not created.
 * @retval #LOS_OK                       The task priority is successfully set to a specified priority.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskPriSet
 */
extern UINT32 LOS_TaskPriSet(UINT32 taskID,
                             UINT16 taskPrio);

/**
 * @ingroup  los_task
 * @brief Set the priority of the current running task to a specified priority.
 *
 * @par Description:
 * This API is used to set the priority of the current running task to a specified priority.
 *
 * @attention
 * <ul>
 * <li>Changing the priority of the current running task probably causes task scheduling.</li>
 * <li>Using the interface to change the priority of software timer task and idle task is not allowed.</li>
 * <li>Using the interface in the interrupt is not allowed.</li>
 * </ul>
 *
 * @param  taskPrio [IN] Type #UINT16 Task priority.
 *
 * @retval #LOS_ERRNO_TSK_PRIOR_ERROR     Incorrect task priority.Re-configure the task priority
 * @retval #LOS_ERRNO_TSK_OPERATE_IDLE    Check the task ID and do not operate on the idle task.
 * @retval #LOS_ERRNO_TSK_ID_INVALID      Invalid Task ID
 * @retval #LOS_ERRNO_TSK_NOT_CREATED     The task is not created.
 * @retval #LOS_OK                        The priority of the current running task is successfully set to a specified
 *                                        priority.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskPriGet
 */
extern UINT32 LOS_CurTaskPriSet(UINT16 taskPrio);

/**
 * @ingroup  los_task
 * @brief Change the scheduling sequence of tasks with the same priority.
 *
 * @par Description:
 * This API is used to move current task in a queue of tasks with the same priority to the tail of the queue of ready
 * tasks.
 *
 * @attention
 * <ul>
 * <li>At least two ready tasks need to be included in the queue of ready tasks with the same priority. If the
 * less than two ready tasks are included in the queue, an error is reported.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID                    Invalid Task ID
 * @retval #LOS_ERRNO_TSK_YIELD_NOT_ENOUGH_TASK         No tasks with the same priority is available for scheduling.
 * @retval #LOS_OK                                      The scheduling sequence of tasks with same priority is
 *                                                      successfully changed.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskYield(void);

/**
 * @ingroup  los_task
 * @brief Obtain a task priority.
 *
 * @par Description:
 * This API is used to obtain the priority of a specified task.
 *
 * @attention None.
 *
 * @param  taskID [IN] Type #UINT32 Task ID. The task id value is obtained from task creation.
 *
 * @retval #OS_INVALID      The task priority fails to be obtained.
 * @retval #UINT16          The task priority.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskPriSet
 */
extern UINT16 LOS_TaskPriGet(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Obtain current running task ID.
 *
 * @par Description:
 * This API is used to obtain the ID of current running task.
 *
 * @attention
 * <ul>
 * <li> This interface should not be called before system initialized.</li>
 * </ul>
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID    Invalid Task ID.
 * @retval #UINT32                      Task ID.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_CurTaskIDGet(void);

/**
 * @ingroup  los_task
 * @brief Obtain next running task ID.
 *
 * @par Description:
 * This API is used to obtain the ID of next running task.
 *
 * @attention None.
 *
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID    invalid Task ID.
 * @retval #UINT32                      task id.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_NextTaskIDGet(void);

/**
 * @ingroup  los_task
 * @brief Obtain next running task ID.
 *
 * @par Description:
 * This API is used to obtain the ID of next running task.
 *
 * @attention None.
 *
 *
 * @retval #NULL            invalid Task name.
 * @retval #CHAR*           task name.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern CHAR *LOS_CurTaskNameGet(void);

/**
 * @ingroup  los_task
 * @brief Obtain a task information structure.
 *
 * @par Description:
 * This API is used to obtain a task information structure.
 *
 * @attention
 * <ul>
 * <li>One parameter of this interface is a pointer, it should be a correct value, otherwise, the system may be
 * abnormal.</li>
 * </ul>
 *
 * @param  taskID    [IN]  Type  #UINT32 Task ID. The task id value is obtained from task creation.
 * @param  taskInfo  [OUT] Type  #TSK_INFO_S* Pointer to the task information structure to be obtained.
 *
 * @retval #LOS_ERRNO_TSK_PTR_NULL        Null parameter.
 * @retval #LOS_ERRNO_TSK_ID_INVALID      Invalid task ID.
 * @retval #LOS_ERRNO_TSK_NOT_CREATED     The task is not created.
 * @retval #LOS_OK                        The task information structure is successfully obtained.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskInfoGet(UINT32 taskID,
                              TSK_INFO_S *taskInfo);

/**
 * @ingroup  los_task
 * @brief Obtain the task status.
 *
 * @par Description:
 * This API is used to obtain the task status.
 *
 * @attention None.
 *
 * @param  taskID     [IN] Type  #TSK_HANDLE_T Task ID.
 * @param  taskStatus [OUT] Type  #UINT32 Pointer to the task status to be obtained.
 *
 * @retval #LOS_ERRNO_TSK_PTR_NULL                    0x02000201: Null parameter.
 * @retval #LOS_ERRNO_TSK_ID_INVALID                  0x02000207: Invalid task ID.
 * @retval #LOS_ERRNO_TSK_NOT_CREATED                 0x0200020a: The task is not created.
 * @retval #LOS_OK                                   0: The task information structure is successfully obtained.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskStatusGet(UINT32 taskID,
                                UINT32 *taskStatus);

/**
 * @ingroup los_monitor
 * @brief Obtain all tasks info.
 *
 * @par Description:
 * This API is used to obtain all tasks info.
 * @attention
 * <ul>
 * <li>This API can be called only after the CPU usage is initialized. Otherwise, -1 will be returned.</li>
 * </ul>
 *
 * @param None.
 *
 * @retval #OS_ERROR           -1:all tasks info obtain failed.
 * @retval #LOS_OK              0:all tasks info is successfully obtained.
 * @par Dependency:
 * <ul><li>los_monitor.h: the header file that contains the API declaration.</li></ul>
 * @see LOS_TaskInfoMonitor
 */
extern UINT32 LOS_TaskInfoMonitor(void);

/**
 * @ingroup  los_task
 * @brief Obtain tasks switch info.
 *
 * @par Description:
 * This API is used to obtain tasks switch info.
 *
 * @attention None.
 *
 * @param  index            [IN]  Type  #UINT32  Switch info array index.
 * @param  taskSwitchInfo   [OUT] Type  #UINT32* First 4 bytes is task id, and then is task name, name len is
 *                                OS_TSK_NAME_LEN.
 *
 * @retval #LOS_ERRNO_TSK_PTR_NULL           0x02000201: Null parameter.
 * @retval #LOS_OK                           0: The task switch information is successfully obtained.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_TaskSwitchInfoGet(UINT32 index,
                                    UINT32 *taskSwitchInfo);

/**
 * @ingroup  los_task
 * @brief Obtain tasks schduling info.
 *
 * @par Description:
 * This API is used to obtain task is scheduled.
 *
 * @attention None.
 *
 * @param None.
 *
 * @retval #TRUE         Tasks is scheduled.
 * @retval #FALSE        Tasks not scheduling yet.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern BOOL LOS_TaskIsRunning(void);

/**
 * @ingroup  los_task
 * @brief Obtain current new task ID.
 *
 * @par Description:
 * This API is used to obtain the ID of new task.
 *
 * @attention None.
 *
 *
 * @retval #LOS_ERRNO_TSK_ID_INVALID    0x02000207: invalid Task ID.
 * @retval # Task ID.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_NewTaskIDGet(void);

/**
 * @ingroup  los_task
 * @brief Obtain the task name.
 *
 * @par Description:
 * This API is used to obtain the task name.
 *
 * @attention None.
 *
 * @param  taskID            [IN]  Task ID.
 *
 * @retval #NULL: invalid Task name.
 * @retval # Task name.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern CHAR *LOS_TaskNameGet(UINT32 taskID);

extern UINT32 LOS_TaskJoin(UINT32 taskID, UINTPTR *retval);

extern UINT32 LOS_TaskDetach(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Initialization a task.
 *
 * @par Description:
 * This API is used to initialization a task.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  UINT32    Initialization result.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 OsTaskInit(void);

/**
 * @ingroup  los_task
 * @brief Create idle task.
 *
 * @par Description:
 * This API is used to create idle task.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  UINT32   Create result.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 OsIdleTaskCreate(void);

/**
 * @ingroup  los_task
 * @brief Check task switch.
 *
 * @par Description:
 * This API is used to check task switch.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern void OsTaskSwitchCheck(void);

/**
 * @ingroup  los_task
 * @brief TaskMonInit.
 *
 * @par Description:
 * This API is used to taskMonInit.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern void OsTaskMonInit(void);

/**
 * @ingroup  los_task
 * @brief Task entry.
 *
 * @par Description:
 * This API is used to task entry.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  taskID  [IN] Type #UINT32   task id.
 *
 * @retval  None.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern void OsTaskEntry(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Get the task water line.
 *
 * @par Description:
 * This API is used to get the task water line.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  taskID [IN] Type #UINT32 task id.
 *
 * @retval  UINT32  Task water line.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsGetTaskWaterLine(UINT32 taskID);

/**
 * @ingroup  los_task
 * @brief Convert task status to string.
 *
 * @par Description:
 * This API is used to convert task status to string.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  taskStatus [IN] Type #UINT16 task status.
 *
 * @retval  UINT8 *  String.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT8 *OsConvertTskStatus(UINT16 taskStatus);

/**
 * @ingroup  los_task
 * @brief Get all task information.
 *
 * @par Description:
 * This API is used to get all task information.
 *
 * @attention
 * <ul>
 * <li>None.</li>
 * </ul>
 *
 * @param  None.
 *
 * @retval  UINT32  All task information.
 * @par Dependency:
 * <ul><li>los_task.h: the header file that contains the API declaration.</li></ul>
 * @see None.
 */
extern UINT32 OsGetAllTskInfo(void);

extern void *OsTskUserStackInit(void *stackPtr, void *userSP, UINT32 userStackSize);

extern UINT32 OsPmEnterHandlerSet(void (*func)(void));

extern void LOS_TaskResRecycle(void);

extern void OsLmsInit(void);

extern void OsLmsCheckValid(UINTPTR checkAddr, BOOL isFreeCheck);

extern void OsLmsLosMallocMark(const void *curNodeStart,
                               const void *nextNodeStart,
                               UINT32 nodeHeadSize);

extern void OsLmsLosFreeMark(const void *curNodeStart,
                             const void *nextNodeStart,
                             UINT32 nodeHeadSize);

extern void OsLmsSimpleMark(UINTPTR startAddr, UINTPTR endAddr, UINT32 value);

extern void OsLmsPrintPoolListInfo(void);

extern void OsLmsReportError(UINTPTR p, UINT32 size, UINT32 errMod);

extern void CheckValid(const CHAR *dest, const CHAR *src);

extern void __asan_store1_noabort(UINTPTR p);

extern void __asan_store4_noabort(UINTPTR p);

extern void __asan_load4_noabort(UINTPTR p);

extern void __asan_load1_noabort(UINTPTR p);

extern void __asan_loadN_noabort(UINTPTR p, UINT32 size);

extern void __asan_storeN_noabort(UINTPTR p, UINT32 size);

extern void __asan_store2_noabort(UINTPTR p);

extern void __asan_load2_noabort(UINTPTR p);

extern void __asan_store8_noabort(UINTPTR p);

extern void __asan_load8_noabort(UINTPTR p);

extern void __asan_load16_noabort(UINTPTR p);

extern void __asan_store16_noabort(UINTPTR p);

extern void __asan_handle_no_return(void);

/**
 * @ingroup los_err
 * @brief Error handling function.
 *
 * @par Description:
 * This API is used to perform different operations according to error types.
 * @attention
 * <ul>
 * <li>None</li>
 * </ul>
 *
 * @param  fileName  [IN] Log file that stores error information.
 * @param  lineNo    [IN] Line number of the erroneous line which should not be OS_ERR_MAGIC_WORD.
 * @param  errorNo   [IN] Error code.
 * @param  paraLen   [IN] Length of the input parameter pPara.
 * @param  para      [IN] User label of the error.
 *
 * @retval LOS_OK The error is successfully processed.
 * @par Dependency:
 * <ul><li>los_err.h: the header file that contains the API declaration.</li></ul>
 * @see None
 */
extern UINT32 LOS_ErrHandle(CHAR *fileName,
                            UINT32 lineNo,
                            UINT32 errorNo,
                            UINT32 paraLen,
                            void *para);

/**
 * @ingroup los_lmk
 * @brief Register a low memory killer node.
 *
 * @par Description:
 * This API is used to register a low memory killer node. A LosLmkOpsNode node
 * can be registered only once.
 *
 * @attention None.
 *
 * @param  lmkNode [IN] The LosLmkOpsNode node to be registered.
 *
 * @retval LOS_OK The LosLmkOpsNode node is registered successfully.
 * @retval LOS_ERRNO_LMK_INVALID_PARAMETER  The parameter is invalid.
 * @retval LOS_ERRNO_LMK_ALREADY_REGISTERED The LosLmkOpsNode node already registered.
 * @par Dependency:
 * <ul><li>los_lmk.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_LmkOpsNodeRegister(struct LosLmkOpsNode *lmkNode);

/**
 * @ingroup los_lmk
 * @brief Unregister a low memory killer node.
 *
 * @par Description:
 * This API is used to unregister a low memory killer node.
 *
 * @attention None.
 *
 * @param  lmkNode [IN] The LosLmkOpsNode node to be registered.
 *
 * @retval LOS_OK The LosLmkOpsNode node is unregistered successfully.
 * @retval LOS_ERRNO_LMK_NOT_REGISTERED The LosLmkOpsNode node is not yet registered.
 * @par Dependency:
 * <ul><li>los_lmk.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_LmkOpsNodeUnregister(struct LosLmkOpsNode *lmkNode);

/**
 * @ingroup los_lmk
 * @brief Initialize low memory killer framework.
 *
 * @par Description:
 * This API is used to initialize the low memory killer framework.
 *
 * @attention None.
 *
 * @param  None.
 *
 * @retval None.
 * @par Dependency:
 * <ul><li>los_lmk.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern void OsLmkInit(void);

/**
 * @ingroup los_lmk
 * @brief Restore the tasks killed by the task which triggers low memory killer.
 *
 * @par Description:
 * This API is used to restore the tasks killed by the task which triggers low memory killer.
 * This function will be invoked by the developer as needed.
 *
 * @attention None.
 *
 * @param  None.
 *
 * @retval LOS_OK  All the restore killed tasks functions are invoked successfully.
 * @retval LOS_ERRNO_LMK_RESTORE_NOT_NEEDED No need to restore since no tasks killed to free memory.
 * @retval LOS_ERRNO_LMK_RESTORE_TASKS_FAILURE Failed to restore the killed tasks by invoking the registered functions.
 * @par Dependency:
 * <ul><li>los_lmk.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_LmkTasksRestore(void);

/**
 * @ingroup los_lmk
 * @brief Kill the tasks to release the used memory.
 *
 * @par Description:
 *  This API is used to kill the tasks to release the used memory when low memory killer is triggered.
 *
 * @attention None.
 *
 * @param  None.
 *
 * @retval LOS_OK All the free memory functions are invoked successfully.
 * @retval LOS_ERRNO_LMK_MEMORY_ALREADY_FREED The registered free memory functions have been invoked.
 * @retval LOS_ERRNO_LMK_FREE_MEMORY_FAILURE Failed to free memory by invoking the registered functions.
 * @par Dependency:
 * <ul><li>los_lmk.h: the header file that contains the API declaration.</li></ul>
 * @see
 */
extern UINT32 LOS_LmkTasksKill(void);
