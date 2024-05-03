pub const OS_MEM_SLI: u32 = 3;
pub const OS_MEM_SMALL_BUCKET_COUNT: u32 = 31;
pub const OS_MEM_SMALL_BUCKET_MAX_SIZE: u32 = 128;
pub const OS_MEM_LARGE_BUCKET_COUNT: u32 = 24;
pub const OS_MEM_LARGE_START_BUCKET: u32 = 7;
pub type UINT8 = ::std::os::raw::c_uchar;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct OsMemNodeHead {
    pub ptr: OsMemNodeHeadUnion,
    pub size_and_flag: UINT32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union OsMemNodeHeadUnion {
    pub prev: *mut OsMemNodeHead,
    pub next: *mut OsMemNodeHead,
}
