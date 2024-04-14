# Rust特性
Rust 是一种注重安全、并发和内存效率的系统编程语言。它通过一套独特的特性组合来实现内存安全而不需要垃圾回收器。下面是对Rust中的几个核心概念——所有权、借用、生命周期以及宏编程的详细介绍。

**Rust学习地址：**[Rust语言圣经https://course.rs](https://course.rs/)

**所有权（Ownership）**

所有权是Rust最独特的特性之一，它帮助管理内存安全。所有权有三个基本规则：
- 每个值在Rust中都有一个变量，称为其所有者。
- 一次只能有一个所有者。
- 当所有者（变量）离开作用域时，值将被丢弃。
  
这些规则确保Rust在编译时就能避免数据竞争、悬挂指针、内存泄露等问题。

下以**转移所有权**为例详细说说Rust的所有权特性：
```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```
编译后发现报错：
```
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:5:28
  |
2 |     let s1 = String::from("hello");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2 = s1;
  |              -- value moved here
4 |
5 |     println!("{}, world!", s1);
  |                            ^^ value borrowed here after move
  |
  = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
help: consider cloning the value if the performance cost is acceptable
  |
3 |     let s2 = s1.clone();
  |                ++++++++

For more information about this error, try `rustc --explain E0382`.
```
为什么呢？
String 类型是一个复杂类型，由存储在栈中的堆指针、字符串长度、字符串容量共同组成。假设在String拷贝过程中s1没有失效，则会出现一个值拥有两个**所有者**的情况。当变量离开作用域后，Rust 会自动调用 drop 函数并清理变量的堆内存。不过由于两个 String 变量指向了同一位置。这就有了一个问题：当 s1 和 s2 离开作用域，它们都会尝试释放相同的内存。这是一个叫做 二次释放（double free） 的错误，属于内存安全性 BUG 。

所以当 s1 被赋予 s2 后，Rust 认为 s1 不再有效，因此也无需在 s1 离开作用域后 drop 任何东西，这就是把所有权从 s1 转移给了 s2，s1 在被赋予 s2 后就马上失效了。

**借用（Borrowing）**
借用是Rust用于访问数据而不取得其所有权的机制。Rust通过引用来实现借用，分为两种：不可变引用（&T）和可变引用（&mut T）。借用规则如下：

1. 可以有任意数量的不可变引用（&T），但在同一时间，这些不可变引用所指向的数据不能被修改。
2. 同时只能有一个可变引用（&mut T），这期间不能有不可变引用存在。这避免了数据竞争。
   
这些规则确保内存安全和线程安全，防止数据竞争。
例如代码：
```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);
```
编译后出现报错
```
error[E0499]: cannot borrow `s` as mutable more than once at a time 同一时间无法对 `s` 进行两次可变借用
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ first mutable borrow occurs here 首个可变引用在这里借用
5 |     let r2 = &mut s;
  |              ^^^^^^ second mutable borrow occurs here 第二个可变引用在这里借用
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- first borrow later used here 第一个借用在这里使用
```
原因是同一作用域，特定数据只能有一个可变引用或者多个不可变引用。

**生命周期（Lifetimes）**
生命周期用于保证所有的借用都是有效的,确保了引用所指向的内存在引用存在的时候保持有效（即避免悬垂引用）。可以在函数签名中使用生命周期参数来注明引用的生命周期：

```rust
fn function<'a>(param: &'a i32);
```
生命周期注解告诉Rust param参数的生命周期不得短于'a。Rust编译器使用这些注解来检查借用的有效性，防止悬挂引用的产生。

值得注意的是，**生命周期标注并不会改变任何引用的实际作用域**。

**宏编程（Macros）**
宏提供了一种元编程的方式，允许写代码来生成其他代码。Rust宏可以在编译时进行代码的扩展，从而写出更加灵活和可复用的代码。Rust宏主要有两种：

1. 声明式宏（Declarative Macros）：
  - 类似于模式匹配的方式，用于生成重复的代码。
  - 示例：macro_rules! 宏。
  - 这种宏允许我们写出类似 match 表达式的代码，将传入的 Rust 代码与模式进行匹配，然后执行相关联的代码。
2. 过程式宏（Procedural Macros）：
  - 更复杂的宏，可以接受 Rust 代码作为输入并操作这些代码。
  - 用于实现自定义派生等高级功能。
例如：使用宏实现简化版的 vec!
```rust
#[macro_export] //简化版的 vec!
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    };
}
```
### 模式匹配
模式匹配是一种强大的控制流结构，可以根据数据的形状做出决策。它主要通过match语句和if let表达式实现，非常适合用于解构枚举、结构体、元组和其他复合类型的值。
例如：四选一模式匹配
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        },
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y);
        },
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b);
        },
    }
}
```
有点像verilog里的四选一选择器，满足那个枚举变量就进行相应的操作。


# Rust改写各部分可行性分析
## 初步分析
此部分主要分析改写各部分的好处，从而选择最合适的模块进行改写。

**kernel/src/los_memory.c**
动态内存管理。动态内存管理是操作系统中非常关键的一部分,也是容易出现安全漏洞的地方。使用Rust的所有权和借用机制可以在编译时防止常见的内存安全问题,如缓冲区溢出、悬垂指针等。

**kernel/src/los_task.c**
任务管理。任务管理涉及到任务创建、销毁、调度等操作,如果实现不当,可能导致任务越权访问、资源泄露等问题。Rust的类型系统和所有权机制可以在编译时捕获很多此类错误。

**kernel/src/los_mux.c**
互斥锁。互斥锁是实现任务同步的重要机制,但如果使用不当,可能导致死锁、资源竞争等问题。Rust提供了更安全的同步原语,如Mutex和RwLock,可以在编译时防止很多并发问题。

**kernel/src/los_sem.c**
信号量。信号量也是常用的同步机制,与互斥锁类似,如果使用不当,也可能导致各种并发问题。使用Rust重写可以提高其安全性和正确性。

**arch/arm/src/los_context.S**
上下文切换。上下文切换是操作系统的核心功能,直接影响系统的性能和稳定性。虽然Rust不能直接替换汇编代码,但可以考虑用Rust提供更安全的上层接口,并对上下文切换的参数进行严格的类型检查。

**arch/arm/src/los_interrupt.S**
中断处理。中断处理也是操作系统的关键部分,如果实现不当,可能导致系统崩溃或安全漏洞。同样,可以考虑用Rust提供更安全的上层接口,对中断向量表、中断栈等进行更严格的管理。

## components/fs
文件系统是另一个容易出现安全漏洞的模块,如缓冲区溢出、路径遍历等。使用Rust重写文件系统,可以利用其强大的类型系统和编译时检查来防止很多常见的安全问题。

## components/net
网络模块直接暴露在外部环境中,是攻击者的主要目标之一。使用Rust重写网络协议栈,可以大大提高其安全性和可靠性,防止各种网络攻击。

**经过小组讨论，我们决定改写研究los_memory.c,los_task.c,以及相关文件**，因为这两个模块涉及底层且有较好的泛用性。
## 详细分析
**los_memory.c**
- 代码量:2300line+
- 部分相关文件:los_memory.h
- 相关文件代码量:500line+

los_memory.c是LiteOS的动态内存管理模块,建0议用Rust的alloc模块替换。Rust的alloc模块提供了安全的动态内存分配和释放机制,可以在编译时防止常见的内存安全问题。Rust的alloc模块已经非常成熟,可以满足大多数动态内存管理的需求。使用Rust重写los_memory.c后,可以在编译时捕获大多数内存安全问题,提高系统的可靠性。但是,可能会引入一定的内存和性能开销,需要仔细评估和优化。

**los_task.c**
- 代码量:1500line+
- 部分相关文件:los_memory.h
- 相关文件代码量:1800line+

los_task.c是LiteOS的任务管理模块,建议用Rust重写任务的创建、销毁、调度等核心功能,提高其安全性和可靠性。Rust有丰富的并发编程特性,如线程、协程等,可以用来实现任务管理。但是,LiteOS的任务管理与硬件体系结构和汇编代码紧密相关,完全用Rust重写可能会比较困难。使用Rust重写los_task.c的核心功能后,可以提高任务管理的安全性和可靠性,减少潜在的内存错误和并发问题。但是,由于任务管理与硬件和汇编代码紧密相关,完全重写的难度较大,可能需要分步实施。
