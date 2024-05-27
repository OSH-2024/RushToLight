#[inline]
pub fn OsMemListDelete(pool: &mut OsMemPoolHead, listIndex: UINT32, node: &mut OsMemFreeNodeHead)
{
    if (node == pool->freeList[listIndex]) {
        pool->freeList[listIndex] = node->next;
        if (node->next.isnull()) { //这里需修改一下
            OsMemClearFreeListBit(pool, listIndex);
        } else {
            node->next->prev = ptr::null_mut();
        }
    } else {
        node->prev->next = node->next;
        if (!node->next.isnull()) {
            node->next->prev = node->prev;
        }
    }
    OS_MEM_SET_MAGIC(&node->header);
}