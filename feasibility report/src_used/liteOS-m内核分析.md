# 重要的数据结构 
### 双向链表 ```LOS_DL_LIST```
双向链表```LOS_DL_LIST```定义在list.c中,是含有两个指向双向链表数据类型指针(*LOS_DL_LIST)的结构体
在list.c文件中,还定义了一些对双向链表的配套操作，包括
```LOS_ListInit(LOS_DL_LIST *list)```对链表进行初始化(首尾相连)
```LOS_ListAdd(LOS_DL_LIST *list, LOS_DL_LIST *node)```实现将node插入list后面
```LOS_ListTailInsert(LOS_DL_LIST *list, LOS_DL_LIST *node)```进行尾插
```LOS_ListDelete(LOS_DL_LIST *node)```对链表节点进行删除
等等...
**双向链表是组成其他结构体和数据结构的基本单元**

### 任务就绪队列 ```g_priQueueList```

1.任务就绪队列是一个元素为**双向链表结构体(LOS_DL_LIST)** 的数组, g_losPriorityQueueList是指向该数组的指针,数组的每个元素都可以看做是一个LosTaskCB类型双向链表的头节点,数组元素个数为任务的优先级数量,在liteOS中,总共有32个优先级(记为0~31),数值越小优先级越大
2.g_priQueueBitmap是一个unsigned int类型数据,其上的32bit分别记录着32个双向链表队列各自上面是否有等待的任务,1表示有就绪任务,0表示无就绪任务,最高位存放着优先级为0的任务队列
![pic](./src/优先级队列.png)




### 任务排序链表 ```g_taskSortLinkList```
1.任务排序链表是用于处理**任务延迟到期/超时唤醒**的数据结构,也就是将某个需要延期执行的任务放置到特定位置上(位置与延迟时间相关),之后唤醒
2.```TaskSortLinkAttr```是用来表示任务排序链表的结构体，其内定义了双向链表指针类型(```LOS_DL_LIST *```)的变量sortLink是用于指向一个**双向链表数组**的指针，数组的每个元素(双向链表)作为头结点,具有一个位置索引```sortindex```(0~31),挂载着数个任务块(```LosTaskCB```),每个任务里面含有一个滚轮数(idxRollNum),每次轮到自身执行后rollnum自减,直到为0后从链表移除.
3.TaskSortLinkAttr还定义了一个unsigned int16变量游标```cursor```，记录当前访问的数组的位置,每过一个Tick，游标指向下一个位置，转一轮需要32 ticks。当运行到的数组位置，双向链表不为空，则把第一个节点维护的滚动数减1。这样的数据结构类似钟表表盘，也称为时间轮。使得可以环状访问数组元素
4.对于给定延时的任务,根据延时mod32的商，来分配rollnum,根据mod32的余数来分配sortindex
![pic](./src/任务排序链表.png)
##### 相关函数:
**OsTaskInit()**:对任务排序链表进行初始化,包括为双向链表数组分配初始空间、清空信息，以及对每个双向链表元素进行各自的初始化(首位相连、初始化位置索引)
**OsTaskAdd2TimerList(LosTaskCB *taskCB, UINT32 timeout)**:taskCB指向所要延迟的任务,timeout记录其延迟的时长,通过取timeout低5位作sortindex(之后还要加上游标),高27位作rollnum
```
sortIndex = timeout & OS_TSK_SORTLINK_MASK;
rollNum = (timeout >> OS_TSK_SORTLINK_LOGLEN);
```
接下来,将任务插入到合适的位置(**满足时延，同时不影响其他的任务时延**).如果此处双向链表为空，直接插入链表里。如果链表不为空，执行⑻获取第一个链表节点对应的任务taskDelay，然后遍历循环双向链表，把任务插入到合适的位置。⑼处如果待插入任务taskCB的滚动数大于等于当前链表节点对应任务的滚动数，则从待插入任务taskCB的滚动数中减去当前链表节点对应任务的滚动数，然后执行⑾获取下一个节点继续遍历。⑽处如果待插入任务taskCB的滚动数小于当前链表节点对应任务的滚动数，则从当前链表节点对应任务的滚动数中减去待插入任务taskCB的滚动数，然后跳出循环。执行⑿，完成任务插入。
```
 if (listObject->pstNext == listObject) {
        LOS_ListTailInsert(listObject, &taskCB->timerList);
    } else {
⑻      taskDelay = LOS_DL_LIST_ENTRY((listObject)->pstNext, LosTaskCB, timerList);
        do {
⑼          if (UWROLLNUM(taskDelay->idxRollNum) <= UWROLLNUM(taskCB->idxRollNum)) {
                UWROLLNUMSUB(taskCB->idxRollNum, taskDelay->idxRollNum);
            } else {
⑽              UWROLLNUMSUB(taskDelay->idxRollNum, taskCB->idxRollNum);
                break;
            }

⑾          taskDelay = LOS_DL_LIST_ENTRY(taskDelay->timerList.pstNext, LosTaskCB, timerList);
        } while (&taskDelay->timerList != (listObject));

⑿      LOS_ListTailInsert(&taskDelay->timerList, &taskCB->timerList);
    }
```

##### OsTimerListDelete(LosTaskCB *taskCB)


# 任务处理
### 任务控制块```LosTaskCB```
LosTaskCB在```kernel\include\los_task.h```中定义,用于任务的控制,定义如下:
```
typedef struct {
    VOID                        *stackPointer;            /* 任务栈指针 */
    UINT16                      taskStatus;               /* 任务状态 */
    UINT16                      priority;                 /* 任务优先级 */
    INT32                       timeSlice;                /* 剩余的时间片 */
    UINT32                      waitTimes;
    SortLinkList                sortList;                 /* 任务超时排序链表节点 */
    UINT64                      startTime;
    UINT32                      stackSize;                /* 任务栈大小 */
    UINT32                      topOfStack;               /* 栈顶指针 */
    UINT32                      taskID;                   /* 任务编号Id */
    TSK_ENTRY_FUNC              taskEntry;                /* 任务入口函数 */
    VOID                        *taskSem;                 /* 任务持有的信号量 */
    VOID                        *taskMux;                 /* 导致任务阻塞的互斥锁 */
    UINT32                      arg;                      /* 任务入口函数的参数 */
    CHAR                        *taskName;                /* 任务名称 */
    LOS_DL_LIST                 pendList;                 /* 就绪队列等链表节点 */
    LOS_DL_LIST                 timerList;                /* 任务超时排序链表节点 */
    EVENT_CB_S                  event;
    UINT32                      eventMask;                /* 事件掩码 */
    UINT32                      eventMode;                /* 事件模式 */
    VOID                        *msg;                     /* 分给给队列的内存*/
    INT32                       errorNo;
} LosTaskCB;
```
其中,**``taskStatus``记录任务的状态**,一共有```Running```(运行态),```Suspend```(阻塞态),```Ready```(就绪态),```Delay```(延迟态),```Unused```(未使用),```Exit```(退出态)...每一个任务都会分配一个栈空间,由一个指针指向栈顶
### 任务模块初始化
在```OsTaskInit()```中,根据开发板配置的最大任务数```g_taskMaxNum```，计算需要申请的内存大小size，为任务控制块TCB数组（也叫作任务池）g_taskCBArray申请内存。在为任务池分配好内存后,还会初始化两个特殊的链表: 空闲任务链表(g_losFreeTask)和回收任务链表(g_taskRecyleList)并将任务池的所有任务尾插入空闲任务链表.除此之外,还会初始化```g_losTask```,该变量用来维护当前正在执行的任务.最后,调用```OsSchedInit()```初始化任务调度模块
### 任务调度
LiteOS-m使用g_losTask维护当前运行任务(```runTask```)和新任务(```newTask```),并且使用**优先级队列算法**来从就绪队列挑选运行
```OsSchedInit()```在Task初始化时被调用,会将优先级队列```g_priQueueList```和排序队列```g_taskSortLinkList```初始化
```VOID LOS_Schedule()```调用```VOID OsSchedStart()```,从优先级队列中选出最高优先的任务,设置其为runTask和newTask,设置其运行时间,将其移除优先级队列,并且设定任务到期时间,**即当前运行时间加上时间片的长度**
### 任务创建
```LOS_TaskCreateOnly(UINT32 *taskID, TSK_INIT_PARAM_S *taskInitParam)```接受一个记录创建任务的信息的结构体```TSK_INIT_PARAM_S```,在调用```OsTaskInitParamCheck()```检测合法性后,从空闲链表中找到一个空闲任务(```LOS_DL_LIST_FIRST(&g_losFreeTask)```),(如果没有,则从回收链表中回收一个),并且将它从空闲链表中删除```LOS_ListDelete(LOS_DL_LIST_FIRST(&g_losFreeTask))```,挂载到优先级队列中,之后调用```OsNewTaskInit(LosTaskCB *taskCB, TSK_INIT_PARAM_S *taskInitParam, VOID *topOfStack)```初始化任务控制块,值得注意的是,在初始化的过程中,会给该任务的status赋予Suspend阻塞态.
如果需要该任务进入就绪态,就需要```LOS_TaskCreate()```,该函数调用```LOS_TaskCreateOnly()```,并且调用```OsSchedTaskEnQueue(taskCB)```设置状态为就绪态,将其插入**就绪队列**,并且调用```LOS_Schedule()```触发调度
### 任务回收
在任务执行完毕后,会从优先级队列(运行态)中挂载到```taskRecyleList```链表上(回收态/退出态),在将要创建一个新的任务,或者是系统进入空闲的时候,会调用```OsRecyleFinishedTask()```,该函数找到任务回收链表的首个任务,将它从任务回收链表中删去,然后挂载到空闲链表上,**并且释放掉任务栈的内存**
### 任务延迟

### 任务上下文切换
在任务切换时,上一个进程会将当前的寄存器数值等信息保存在任务上下文(```Taskcontext```)结构体中，下一个进程会将自己保存的寄存器数值恢复

# 内存管理
LiteOS使用分区式存储管理

## 静态内存管理
在嵌入式实时操作系统中，对内存的分配时间要求更为苛刻，分配内存的时间必须是确定的。并且内存十分有限珍贵，而在分配中随着内存不断被分配和释放,应避免产生内存碎片,而**静态内存管理机制,分配的效率高，且不会造成内存碎片**,缺点是定长的内存,会导致存储资源的浪费,影响并发性
### 静态内存池```Memory box```
LiteOS的静态内存池```pool```由一个控制块```LOS_MEMBOX_INFO```和若干个固定长度的静态内存块```Memory block```构成,在空间中连续分布,如图是控制块的定义，它记录了内存池的信息，并且有一个指向首个内存块的指针
```
⑴  typedef struct tagMEMBOX_NODE {
        struct tagMEMBOX_NODE *pstNext; /**< 静态内存池中空闲节点指针，指向下一个空闲节点 */
    } LOS_MEMBOX_NODE;

⑵  typedef struct LOS_MEMBOX_INFO {
        UINT32 uwBlkSize;               /**< 静态内存池的总长度
        UINT32 uwBlkNum;                /**< 静态内存池的内存块总数量 */
        UINT32 uwBlkCnt;                /**< 静态内存池的已分配的内存块总数量 */
    #if (LOSCFG_PLATFORM_EXC == 1)
        struct LOS_MEMBOX_INFO *nextMemBox; /**< 指向下一个静态内存池 */
    #endif
        LOS_MEMBOX_NODE stFreeList;     /**< 静态内存池的空闲内存块单向链表 */
    } LOS_MEMBOX_INFO;
```
静态内存块有数据域```data```和内存块头```LOS_MEMBOX_NODE```二者组成,指针域指向下一个内存块的头部,从而将整个静态内存串联起来形成链表
![pic](./src/静态内存.png)

### 初始化静态内存
```LOS_MemboxInit(VOID *pool, UINT32 poolSize, UINT32 blkSize)```接受pool作为内存池的首地址指针, poolSize作为内存池长度, blkSize作为**数据段**所需内存长度,在对参数进行合理性检测后,计算内存块的长度、内存块的数量,对控制块进行更新,接下来从控制块的```stFreeList```开始把每个内存块通过指针连成一个**空闲内存单向链表**，**通过该链表对内存块进行管理**

### 清空内存块数据段信息
```LOS_MemboxClr(VOID *pool, VOID *box)```会清除pool这个内存池中,从box开始的那一段数据段(box应该指向数据段的首地址),清除的长度即为内存块长度-块头长度 (此处并没有检测box是否为数据段首地址)

### 静态内存的申请
```VOID *LOS_MemboxAlloc(VOID *pool)```会从空闲内存单向链表```stFreeList```中取出首个内存块,并且在pstNext处设置魔术字,表示其已经被分配,同时返回该块**数据段的首地址**,并修改控制块使```uwBlkCnt```自减
### 静态内存的释放
```LOS_MemboxFree(VOID *pool, VOID *box)```首先会调用```OsCheckBoxMem()```对box对应的内存块的首地址进行验证,即计算它与内存池空间首个内存块首地址的偏移量,要求其必须处在规定范围内，且偏移量必须是内存块长度的整数倍,还将检测ptrNext域是否是魔术字,以判断该块是否真的被分配出去,**目的释放到链表的内存是先前分配的内存**,保证静态内存的稳定.检测成功后将该块挂载到空闲内存链表,使```uwBlkCnt```自增



## 动态内存管理

### 内存块组织结构
#### 1.内存块划分:```TLSF```算法
LiteOS-m根据**长度**对内存块进行了划分,首先将字节在```[4:127]```的内存块等长地划分为了31个一级区间,每个区间对应的内存块长度都是4的整数倍(4,8,12.....124),每个区间分配有一级编号```FL(1~31)```.接下来对字节在```2^7:2^31-1```的内存块根据```TLSF(内存两级分配算法)```进行分割,先粗粒度划分,所有在```2^k~2^(k+1)-1```的内存块被划分为一个一级区间,它的一级编号```FL(first level)```，接下来在每个一级区间```2^k~2^(k+1)-1```中以```2^(k-3)```作为分隔片分割成8个二级小区间,拥有标号```SL(second level)```
因此，不同的内存块经过TLSF分割成了最细粒度的内存单元，包括[4:127]的31个一级区间和和```2^7:2^31-1```的个二级区间,每个区间的空闲内存块将由一个空闲链表维护,一共223个空闲链表,他们的头节点都是```空闲内存链表数组freeList[]```的元素
![pic](./src/TLSF.png)
#### 2.动态内存池```MemPool```
动态内存池将由```动态内存池头结构体OsMemPoolHead```和```空闲内存块节点freenode```、```使用内存块节点usednode```三部分组成
```OsMemPoolHead```包含一个记录内存池信息的结构体```OsMemPoolInfo```(记录大小、开始地址..),以及```空闲内存链表数组freeList[]```,还有```空闲内存链表位图数组freeListBitmap[]```,一共是7个int32整型,每一bit都表示一个空闲链表是否有空闲(除去map[0]的首位)
![pic](./src/动态内存.png)
#### 3.内存块
内存块由节点头结构体```OsMemNodeHead```和数据段```data```构成,节点头结构体含有指向上一个块的指针域```prev```和记录当前节点大小以及**是否被使用**信息的```Size and Tag```

### 动态内存块的操作
#### 1.合并 ```OsMemMergeNode()```
```OsMemMergeNode(struct OsMemNodeHead *node)```将给定节点node与它的前一个空闲节点进行合并,形成的新节点,节点头为前一个节点的头

#### 2.分割```OsMemSplitNode()```
```VOID OsMemSplitNode(VOID *pool, struct OsMemNodeHead *allocNode, UINT32 allocSize)```将根据所需的内存块长度,将allocNode分割成所需大小节点和剩余节点```newFreeNode```(并且初始化newFreeNode的节点头),如果newFreeNode的后一节点(如果有)也是未使用节点,把后一节点从链表上删除,将二者合并(可能是为了减少碎块),并重新插入链表(新分割出的newFreeNode本身并不在链表上)

#### 3.找到满足大小的内存块```OsMemFreeNodeGet(VOID *pool, UINT32 size)```
该函数接收size作为申请内存块的长度,先通过计算所需内存块对应的FL和SL(属于的区间),检查其bitmap是否为1,如果如此,说明恰好有空闲等大空间,直接返回,反之,向更长的的区间去寻找,然后把获取到的内存块**从空闲内存链表删除**，返回内存节点地址。

#### 4.内存块申请```OsMemAlloc(VOID *pool, UINT32 size)```
首先检测size是否合法,接下来调用```OsMemFreeNodeGet(VOID *pool, UINT32 size)```找到一个不小于size的空闲内存块,当大小恰好相等时,可以返回,否则说明它大于所需内存长度,对其进行分割操作,分割出一块合适大小的块，将其移出空闲链表,将剩余节点挂载到对应长度的空闲链表上