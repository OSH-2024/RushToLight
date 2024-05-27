#[inline]
pub fn OsMemMulRegionsLink(poolHead: &mut OsMemPoolHead,lastStartAddress: *mut c_void, lastLength: UINT32,
    lastEndNode: &mut OsMemNodeHead, memRegion: &const LosMemRegion)
{
let mut curLength: UINT32;
let mut gapSize: UINT32;
let mut curEndNode: &mut OsMemNodeHead;
let mut curFreeNode: &mut OsMemNodeHead;
let mut curStartAddress: *mut c_void = ptr::null_mut();

curStartAddress = memRegion->startAddress;
curLength = memRegion->length;
#[cfg(feature = "LOSCFG_KERNEL_LMS")]{  //需要更改条件编译
let mut resize: UINT32 = 0;
if (!g_lms.isnull()) {   //需要考虑全局变量
/*
* resize == 0, shadow memory init failed, no shadow memory for this pool, set poolSize as original size.
* resize != 0, shadow memory init successful, set poolSize as resize.
*/
resize = g_lms->init(curStartAddress, curLength);
curLength = (resize == 0) ? curLength : resize;
}
}

// mark the gap between two regions as one used node
gapSize = (curStartAddress (as  &const UINT8)) - ((poolHead)(as &const UINT8) + poolHead->info.totalSize);
lastEndNode->sizeAndFlag = gapSize + OS_MEM_NODE_HEAD_SIZE;
OS_MEM_SET_MAGIC(lastEndNode);
OS_MEM_NODE_SET_USED_FLAG(lastEndNode->sizeAndFlag);

// mark the gap node with magic number
OS_MEM_MARK_GAP_NODE(lastEndNode);

poolHead->info.totalSize += (curLength + gapSize);
poolHead->info.totalGapSize += gapSize;

curFreeNode = (struct OsMemNodeHead *)curStartAddress;
curFreeNode->sizeAndFlag = curLength - OS_MEM_NODE_HEAD_SIZE;
curFreeNode->ptr.prev = lastEndNode;
OS_MEM_SET_MAGIC(curFreeNode);
OsMemFreeNodeAdd(poolHead, curFreeNode as &const OsMemFreeNodeHead);

curEndNode = OS_MEM_END_NODE(curStartAddress, curLength);
curEndNode->sizeAndFlag = 0;
curEndNode->ptr.prev = curFreeNode;
OS_MEM_SET_MAGIC(curEndNode);
OS_MEM_NODE_SET_USED_FLAG(curEndNode->sizeAndFlag);

#[cfg(LOSCFG_MEM_WATERLINE == 1)]{
poolHead->info.curUsedSize += OS_MEM_NODE_HEAD_SIZE;
poolHead->info.waterLine = poolHead->info.curUsedSize;
}

}

pub fn LOS_MemRegionsAdd(pool: &mut OsMemPoolHead, const LosMemRegion *const memRegions, memRegionCount: UINT32)->UINT32//将void*的pool更改成引用
{
let mut ret: UINT32;
let mut lastLength: UINT32;
let mut curLength: UINT32;
let mut regionCount: UINT32;
let mut poolHead: &mut OsMemPoolHead //之前这几个变量是指针
let mut lastEndNode: &mut OsMemNodeHead;
let mut firstFreeNode: &mut OsMemNodeHead;
let memRegion: &const LosMemRegion;
let mut lastStartAddress: *mut c_void = ptr::null_mut();
let mut curStartAddress: *mut c_void = ptr:null_mut();


ret = OsMemMulRegionsParamCheck(pool, memRegions, memRegionCount);
if (ret != LOS_OK) {
return ret;
}

memRegion = memRegions;
regionCount = 0;
if (!pool.isnull()) { // 这里无法判断引用是否为空指针
poolHead = pool;
lastStartAddress = pool;
lastLength = poolHead->info.totalSize;
} else { // initialize the memory pool with the first memory region
lastLength = memRegion->length;
poolHead = (memRegion->startAddress) as &mut OsMemPoolHead;
ret = LOS_MemInit(lastStartAddress, lastLength);
if (ret != LOS_OK) {
return ret;
}
memRegion++;
regionCount++;
}

firstFreeNode = OS_MEM_FIRST_NODE(lastStartAddress); //这里是裸指针
lastEndNode = OS_MEM_END_NODE(lastStartAddress, poolHead->info.totalSize);
/* traverse the rest memory regions, and initialize them as free nodes and link together */
while (regionCount < memRegionCount) {
curStartAddress = memRegion->startAddress;
curLength = memRegion->length;

OsMemMulRegionsLink(poolHead, lastStartAddress, lastLength, lastEndNode, memRegion);
lastStartAddress = curStartAddress;
lastLength = curLength;
lastEndNode = OS_MEM_END_NODE(poolHead, poolHead->info.totalSize);
memRegion++;
regionCount++;
}

firstFreeNode->ptr.prev = lastEndNode;
return ret;
}
#endif //<-这一个endif配套的if不在任务范围内(在前一个任务里面)