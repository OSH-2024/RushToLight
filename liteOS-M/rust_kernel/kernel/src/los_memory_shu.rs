

// 假设你已经定义了 OsMemNodeHead 结构体和相关的函数
struct OsMemNodeHead {
    size_and_flag: usize,
    ptr: Option<Box<OsMemNodeHead>>,
}

fn os_mem_sentinel_node_check(node: &OsMemNodeHead) -> bool {
    // 这里插入你的检查逻辑
    true
}

fn os_mem_node_get_size(size_and_flag: usize) -> usize {
    // 这里插入你的获取大小的逻辑
    0
}

#[inline]
fn os_mem_is_last_sentinel_node(sentinel_node: &OsMemNodeHead) -> bool {
    if !os_mem_sentinel_node_check(sentinel_node) {
        eprintln!("{} {}, The current sentinel node is invalid", file!(), line!());
        return true;
    }

    if os_mem_node_get_size(sentinel_node.size_and_flag) == 0 || sentinel_node.ptr.is_none() {
        return true;
    }

    false
}

fn os_mem_last_sentinel_node_get(node: &mut OsMemNodeHead) -> &mut OsMemNodeHead {
    // 这里插入你的获取最后一个哨兵节点的逻辑
    node
}

fn os_mem_node_set_used_flag(size_and_flag: &mut usize) {
    // 这里插入你的设置已使用标志的逻辑
}

fn os_mem_node_set_last_flag(size_and_flag: &mut usize) {
    // 这里插入你的设置最后标志的逻辑
}

#[inline]
fn os_mem_sentinel_node_set(sentinel_node: &mut OsMemNodeHead, new_node: Option<Box<OsMemNodeHead>>, size: usize) {
    if sentinel_node.ptr.is_some() {
        sentinel_node = os_mem_last_sentinel_node_get(sentinel_node);
    }

    sentinel_node.size_and_flag = size;
    sentinel_node.ptr = new_node;
    os_mem_node_set_used_flag(&mut sentinel_node.size_and_flag);
    os_mem_node_set_last_flag(&mut sentinel_node.size_and_flag);
}

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

STATIC INLINE VOID *OsMemSentinelNodeGet(struct OsMemNodeHead *node)
{
    if (OsMemSentinelNodeCheck(node) == FALSE) {
        return NULL;
    }

    return node->ptr.next;
}

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