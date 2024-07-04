/*STATIC INLINE struct OsMemNodeHead *PreSentinelNodeGet(const VOID *pool, const struct OsMemNodeHead *node)
{
    UINT32 nextSize;
    struct OsMemNodeHead *nextNode = NULL;
    struct OsMemNodeHead *sentinelNode = NULL;

    sentinelNode = OS_MEM_END_NODE(pool, ((struct OsMemPoolHead *)pool)->info.totalSize);
    while (sentinelNode != NULL) {
        if (OsMemIsLastSentinelNode(sentinelNode)) {
            PRINT_ERR("PreSentinelNodeGet can not find node 0x%x\n", node);
            return NULL;
        }
        nextNode = OsMemSentinelNodeGet(sentinelNode);
        if (nextNode == node) {
            return sentinelNode;
        }
        nextSize = OS_MEM_NODE_GET_SIZE(sentinelNode->sizeAndFlag);
        sentinelNode = OS_MEM_END_NODE(nextNode, nextSize);
    }

    return NULL;
}
*/

use std::ptr::null_mut;//空指针
//检查为对的后面加 //
#[inline]
fn pre_sentinel_node_get(pool: *const u8, node: *const OsMemNodeHead) -> *mut OsMemNodeHead {
    let mut next_size: u32;//
    let mut next_node: *mut OsMemNodeHead = null_mut();//
    let mut sentinel_node: *mut OsMemNodeHead = null_mut();//空指针

    sentinel_node = os_mem_end_node(pool, unsafe { (*pool as *const OsMemPoolHead).info.total_size });//
    while !sentinel_node.is_null() {//
        if os_mem_is_last_sentinel_node(sentinel_node) {//
            println!("PreSentinelNodeGet can not find node 0x{:x}", node as usize);//
            return null_mut();//
        }
        next_node = os_mem_sentinel_node_get(sentinel_node);//
        if next_node == node {//
            return sentinel_node;//
        }
        next_size = os_mem_node_get_size(unsafe { (*sentinel_node).size_and_flag });//
        sentinel_node = os_mem_end_node(next_node as *const u8, next_size);//
    }

    null_mut()//
}
/*
STATIC INLINE BOOL OsMemIsLastSentinelNode(struct OsMemNodeHead *sentinelNode)
{
    if (OsMemSentinelNodeCheck(sentinelNode) == FALSE) {
        PRINT_ERR("%s %d, The current sentinel node is invalid\n", __FUNCTION__, __LINE__);
        return TRUE;
    }

    if ((OS_MEM_NODE_GET_SIZE(sentinelNode->sizeAndFlag) == 0) ||
        (sentinelNode->ptr.next == NULL)) {
        return TRUE;
    }

    return FALSE;
}
*/
fn os_mem_is_last_sentinel_node(sentinel_node: *mut OsMemNodeHead) -> bool {//
    if !os_mem_sentinel_node_check(sentinel_node) {//
        println!("{} {}, The current sentinel node is invalid", file!(), line!());//
        return true;//
    }//

    if os_mem_node_get_size(unsafe { (*sentinel_node).size_and_flag }) == 0
        || unsafe { (*sentinel_node).ptr.next.is_null() }//
    {
        return true;//
    }

    false//
}

/*
STATIC INLINE VOID OsMemSentinelNodeSet(struct OsMemNodeHead *sentinelNode, VOID *newNode, UINT32 size)
{
    if (sentinelNode->ptr.next != NULL) {
        sentinelNode = OsMemLastSentinelNodeGet(sentinelNode);
    }

    sentinelNode->sizeAndFlag = size;
    sentinelNode->ptr.next = newNode;
    OS_MEM_NODE_SET_USED_FLAG(sentinelNode->sizeAndFlag);
    OS_MEM_NODE_SET_LAST_FLAG(sentinelNode->sizeAndFlag);
}
*/

fn os_mem_sentinel_node_set(sentinel_node: &mut OsMemNodeHead, new_node: *mut u8, size: u32) {//
    let mut sentinel_node = sentinel_node;//为了在函数中修改参数的值
    if !sentinel_node.ptr.next.is_null() {//
        sentinel_node = os_mem_last_sentinel_node_get(sentinel_node);//
    }

    sentinel_node.size_and_flag = size;//
    sentinel_node.ptr.next = new_node as *mut OsMemNodeHead;//？
    os_mem_node_set_used_flag(&mut sentinel_node.size_and_flag);//
    os_mem_node_set_last_flag(&mut sentinel_node.size_and_flag);//
}
/* 
STATIC INLINE VOID *OsMemSentinelNodeGet(struct OsMemNodeHead *node)
{
    if (OsMemSentinelNodeCheck(node) == FALSE) {
        return NULL;
    }

    return node->ptr.next;
}
*/
fn os_mem_sentinel_node_get(node: &OsMemNodeHead) -> *mut u8 {
    if !os_mem_sentinel_node_check(node) {
        return null_mut();
    }

    node.ptr.next as *mut u8  //存疑，可以翻译成u8嘛
}

/*
STATIC INLINE struct OsMemNodeHead *PreSentinelNodeGet(const VOID *pool, const struct OsMemNodeHead *node)
{
    UINT32 nextSize;
    struct OsMemNodeHead *nextNode = NULL;
    struct OsMemNodeHead *sentinelNode = NULL;

    sentinelNode = OS_MEM_END_NODE(pool, ((struct OsMemPoolHead *)pool)->info.totalSize);
    while (sentinelNode != NULL) {
        if (OsMemIsLastSentinelNode(sentinelNode)) {
            PRINT_ERR("PreSentinelNodeGet can not find node 0x%x\n", node);
            return NULL;
        }
        nextNode = OsMemSentinelNodeGet(sentinelNode);
        if (nextNode == node) {
            return sentinelNode;
        }
        nextSize = OS_MEM_NODE_GET_SIZE(sentinelNode->sizeAndFlag);
        sentinelNode = OS_MEM_END_NODE(nextNode, nextSize);
    }

    return NULL;
}
*/
//待检查
fn pre_sentinel_node_get(pool: *const u8, node: *const OsMemNodeHead) -> *mut OsMemNodeHead {//
    let mut next_size: u32;//
    let mut next_node: *mut OsMemNodeHead = null_mut();//
    let mut sentinel_node: *mut OsMemNodeHead = null_mut();//

    sentinel_node = os_mem_end_node(pool, unsafe { (*pool as *const OsMemPoolHead).info.total_size });//
    while !sentinel_node.is_null() {//
        if os_mem_is_last_sentinel_node(sentinel_node) {//
            println!("PreSentinelNodeGet can not find node 0x{:x}", node as usize);//
            return null_mut();//
        }
        next_node = os_mem_sentinel_node_get(unsafe { &*sentinel_node });
        if next_node == node as *mut OsMemNodeHead {
            return sentinel_node;
        }
        next_size = os_mem_node_get_size(unsafe { (*sentinel_node).size_and_flag });
        sentinel_node = os_mem_end_node(next_node as *const u8, next_size);
    }

    null_mut()
}
