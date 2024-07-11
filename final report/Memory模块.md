## memory模块介绍
LiteOS-M的memory模块用于对连续的内存池进行初始化,以及对内存块的动态分配、回收,此外还定义了有关内存组织的结构体

#### 结构体定义
为了组织内存,LiteOS-M定义了如下重要的结构体：

##### ①内存池头节点
```OsMemPoolHead```包含一个记录内存池信息的结构体```OsMemPoolInfo```(记录大小、开始地址..),以及```空闲内存链表数组freeList[]```,还有```空闲内存链表位图数组freeListBitmap[]```,一共是7个int32整型,每一bit都表示一个空闲链表是否有空闲(除去map[0]的首位)
``` c
struct OsMemPoolHead {
    struct OsMemPoolInfo info;
    UINT32 freeListBitmap[OS_MEM_BITMAP_WORDS];
    //指示哪些长度区间有空闲块
    struct OsMemFreeNodeHead *freeList[OS_MEM_FREE_LIST_COUNT];
    //空闲链表数组
};
struct OsMemPoolInfo {
    VOID *pool;  //起始地址
    UINT32 totalSize; //大小
    UINT32 attr;  //该内存池的一些特征
};
```
##### ②内存块头节点
通过指针域连接各内存块
``` c
struct OsMemNodeHead {
    union {
        struct OsMemNodeHead *prev; /* The prev is used for current node points to the previous node */
        struct OsMemNodeHead *next; /* The next is used for sentinel node points to the expand node */
    } ptr;
    UINT32 sizeAndFlag; //既表示了内存块的长度,又标记了一些特征,比如空闲
};
```
#### 内存组织
在内存块的组织方面,LiteOS-M有两种组织方式:
**其一**,按照内存块起始地址在内存池的前后顺序,通过内存块头节点的指针域进行连接,可以实现对内存块按起始地址大小顺序的顺序访问,使用``OS_MEM_NEXT_NODE()``函数即可通过加上偏移量对下个内存块进行访问
![pic](./src/内存结构.png)

**其二**,LiteOS-m根据**长度**对**空闲内存块**进行了划分,首先将字节在```[4:127]```的内存块等长地划分为了31个一级区间,每个区间对应的内存块长度都是4的整数倍(4,8,12.....124),每个区间分配有一级编号```FL(1~31)```.接下来对字节在```2^7:2^31-1```的内存块根据```TLSF(内存两级分配算法)```进行分割,先粗粒度划分,所有在```2^k~2^(k+1)-1```的内存块被划分为一个一级区间,它的一级编号```FL(first level)```，接下来在每个一级区间```2^k~2^(k+1)-1```中以```2^(k-3)```作为分隔片分割成8个二级小区间,拥有标号```SL(second level)```
因此，不同的内存块经过TLSF分割成了最细粒度的内存单元，包括[4:127]的31个一级区间和和```2^7:2^31-1```的个二级区间,每个区间的空闲内存块将由一个空闲链表维护,一共223个空闲链表,他们的头节点都是```空闲内存链表数组freeList[]```的元素
**由此,通过维护一个空闲链表数组的方式,实现了对特定大小区间内存块的直接访问,访问的时间复杂度大大降低**
![pic](./src/TLSF.png)

#### 内存操作
LiteOS-M的memory模块实现的内存操作API主要有以下:
##### ①```LOS_MemInit()```:
功能是,接受到将要作为内存池的一片内存块的**起始地址与长度**,对内存池头节点```OsMemPoolHead```结构体进行初始化起始地址和长度,对尾部哨兵节点初始化,将剩余的内存块长度视作一个空闲节点,初始内存块的头节点```OsMemNodeHead```(将```sizeAndFlag```赋予内存块长度等信息),并将其挂载到空闲链表数组中
##### ②```LOS_MemAlloc()```:
功能是,根据接收到的内存池头节点的起始位置,需要申请的内存块大小,从而使用```bestfit```内存分配算法,先从所申请内存块长度所在区间对应的```freeList```数组元素链表中寻找适合的内存块,对应空间没有空闲内存块,则向更大的区间搜索,如果搜索成功,则将该内存块分割成两块,分别更改内存块头节点```OsMemNodeHead```的信息(将前者标记为使用,后者标记为空闲),将前者从空闲链表取出作返回值,将后者挂入空闲链表,
**LOS_MemAlloc实现了对内存需求既快又准的分配**
##### ③```LOS_MemFree()```:
这个API与```LOS_MemFree()```恰好相反,通过获得内存池起始地址以及所释放内存块起始地址,LiteOS-M可以将其挂载到空闲链表上,并标记为空闲,**此外LiteOS-M为了避免内存块不断分割产生过多碎片的问题,在释放时,会试图合并该节点与其在内存池中的前驱节点(如果前驱节点仍然空闲)**
##### ④```LOS_MemAllocAlign()```:
功能是申请一块边界对齐的内存块,其中对齐边界由传入参数boundary给定,LiteOS-M的实现策略是,申请一块大小为size+boundary的内存块,将对齐后的内存返回,这块内存既对齐boundary又满足不小于size,由于对齐损失的内存区域被称为gap,会有给定区域存放gapsize,并且会标记sizeAndFalg的部分区域,表示它是一个```AlignGap```节点

##### ⑤```LOS_MemRealloc()```
功能是给定一个已分配的内存块给定地址,重新分配一个给定大小的内存块,其策略是

以上API由一些基础操作组成,比如内存块的查找、内存块的合并、内存块的分割,LiteOS-M将这些基础操作封装成了一下函数:
##### 1.合并 ```OsMemMergeNode()```
```OsMemMergeNode(struct OsMemNodeHead *node)```将给定节点node与它的前一个空闲节点进行合并,形成的新节点,节点头为前一个节点的头

##### 2.分割```OsMemSplitNode()```
```VOID OsMemSplitNode(VOID *pool, struct OsMemNodeHead *allocNode, UINT32 allocSize)```将根据所需的内存块长度,将allocNode分割成所需大小节点和剩余节点```newFreeNode```(并且初始化newFreeNode的节点头),如果newFreeNode的后一节点(如果有)也是未使用节点,把后一节点从链表上删除,将二者合并(可能是为了减少碎块),并重新插入链表(新分割出的newFreeNode本身并不在链表上)

##### 3.找到满足大小的内存块```OsMemFreeNodeGet(VOID *pool, UINT32 size)```
该函数接收size作为申请内存块的长度,先通过计算所需内存块对应的FL和SL(属于的区间),检查其bitmap是否为1,如果如此,说明恰好有空闲等大空间,直接返回,反之,向更长的的区间去寻找,然后把获取到的内存块**从空闲内存链表删除**，返回内存节点地址。

#### 内存保护检查
LiteOS-M对内存具有一定的检查能力,其中包括
①空指针检查:LiteOS-M在部分函数内部会对传入的指针进行是否为空的判断
②对齐性检查:LiteOS-M规定动态内存的对齐边界是4的整数倍,因此在接收到传入地址时会对其是否对齐进行判断
③魔术字检查:LiteOS-M通过设立一个魔术字```magic```来检测该内存块是否被恶意篡改,这个魔术字较为独特,当该块有效时保证魔术字不被改变,如果该处被覆写则说明被其他内存块覆盖
④内存锁:LiteOS-M允许对访问内存的一些特殊操作上锁,以防止并行进程同时访问时引发的读者写者问题

