/* automatically generated by rust-bindgen 0.59.1 */

pub const LOSCFG_SYS_HEAP_SIZE: UINT32 = 65536;
pub const OS_MEM_SLI: UINT32 = 3;
pub const OS_MEM_SMALL_BUCKET_COUNT: UINT32 = 31;
pub const OS_MEM_SMALL_BUCKET_MAX_SIZE: UINT32 = 128;
pub const OS_MEM_LARGE_BUCKET_COUNT: UINT32 = 24;
pub const OS_MEM_LARGE_START_BUCKET: UINT32 = 7;
pub const OS_MEM_FREE_LIST_COUNT: UINT32 = 223;
pub const OS_MEM_BITMAP_WORDS: UINT32 = 7;
pub type VOID = ::std::os::raw::c_void;
pub type UINT8 = ::std::os::raw::c_uchar;
pub type UINT16 = ::std::os::raw::c_ushort;
pub type UINT32 = ::std::os::raw::c_uint;
pub type INT8 = ::std::os::raw::c_schar;
pub type INT16 = ::std::os::raw::c_short;
pub type INT32 = ::std::os::raw::c_int;
pub type FLOAT = f32;
pub type DOUBLE = f64;
pub type CHAR = ::std::os::raw::c_char;
pub type UINT64 = ::std::os::raw::c_ulonglong;
pub type INT64 = ::std::os::raw::c_longlong;
pub type UINTPTR = ::std::os::raw::c_uint;
pub type INTPTR = ::std::os::raw::c_int;
pub type Atomic = INT32;
pub type Atomic64 = INT64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsMemNodeHead {
    #[cfg(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK)]
    magic: UINT32,

    #[cfg(LOSCFG_MEM_LEAKCHECK)]
    linkReg: [usize; LOSCFG_MEM_RECORD_LR_CNT],

    ptr: Ptr,

    #[cfg(all(not(LOSCFG_TASK_MEM_USED), LOSCFG_MEM_FREE_BY_TASKID))]
    taskID: UINT32,

    #[cfg(all(not(LOSCFG_TASK_MEM_USED), LOSCFG_MEM_FREE_BY_TASKID))]
    sizeAndFlag: UINT32,

    #[cfg(all(LOSCFG_TASK_MEM_USED, not(LOSCFG_MEM_FREE_BY_TASKID)))]
    taskID: UINT32,

    #[cfg(all(LOSCFG_TASK_MEM_USED, not(LOSCFG_MEM_FREE_BY_TASKID)))]
    sizeAndFlag: UINT32,

    #[cfg(all(LOSCFG_TASK_MEM_USED, LOSCFG_MEM_FREE_BY_TASKID))]
    taskID: UINT32,

    #[cfg(all(LOSCFG_TASK_MEM_USED, LOSCFG_MEM_FREE_BY_TASKID))]
    sizeAndFlag: UINT32,

    #[cfg(not(any(LOSCFG_BASE_MEM_NODE_INTEGRITY_CHECK, LOSCFG_MEM_LEAKCHECK, LOSCFG_TASK_MEM_USED, LOSCFG_MEM_FREE_BY_TASKID)))]
    sizeAndFlag: UINT32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Ptr {
    prev: *mut OsMemNodeHead,
    next: *mut OsMemNodeHead,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OsMemPoolInfo {
    pub pool: *mut VOID,
    pub totalSize: UINT32,
    pub attr: UINT32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsMemFreeNodeHead {
    pub header: OsMemNodeHead,
    pub prev: *mut OsMemFreeNodeHead,
    pub next: *mut OsMemFreeNodeHead,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OsMemPoolHead {
    pub info: OsMemPoolInfo,
    pub freeListBitmap: [UINT32; 7usize],
    pub freeList: [*mut OsMemFreeNodeHead; 223usize],
}
