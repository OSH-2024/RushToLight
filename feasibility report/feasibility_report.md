# C与Rust交互：代码集成与互操作性分析(author: 区家彬)
## 引言
本部分内容旨在探讨C语言和Rust语言之间进行代码交互的可行性，以及这种交互在现代软件开发中的潜在价值。C语言作为系统级编程语言，以其高效、直接的内存管理和底层控制而广受欢迎，而Rust语言则凭借其内存安全、并发性能和现代编程范式，近年来迅速崛起，特别是在对性能和安全性要求极高的领域。

随着软件工程的复杂性增加，开发者们寻求在不同语言之间无缝集成的能力，以便利用各自语言的优势。C和Rust的结合，尽管看似不同寻常，但它们在底层系统编程和性能优化方面的互补性为这种交互提供了可能。本部分内容旨在通过简单的实践和分析，揭示C与Rust如何在实际项目中协同工作，验证我们小组使用rust语言改写LiteOS内核的部分代码的可行性。

## 相关技术背景

**C语言（C）** 是一种历史悠久的系统级编程语言，以其高效、直接的内存管理、强大的标准库和对硬件的直接访问能力而闻名。它允许开发者编写底层代码，控制内存，从而实现高性能和极致的控制。然而，C语言的安全性是其主要挑战之一，尤其是在处理内存错误和并发问题时，容易导致程序崩溃或安全漏洞。

**Rust语言**，诞生于2010年，旨在解决C和C++的内存安全问题，同时保持高性能。Rust通过所有权系统、生命周期管理和零成本抽象等特性，提供了内存安全的保证，避免了空指针引用、数据竞争等常见错误。它还支持并发编程，提供了`async/await`和`Arc`等工具，使得并发编程变得相对容易和安全。

在技术层面，C和Rust之间的交互主要通过 **C Foreign Function Interface（CFFI）** 和 **bindgen** 等工具来实现。CFFI允许Rust代码调用C函数，而bindgen则可以将Rust的结构体和枚举类型生成为C语言的头文件，方便C代码调用。这些工具为C和Rust之间的数据传递和接口定义提供了桥梁。

尽管C和Rust在设计哲学和编程范式上有显著差异，但它们在底层编程和性能优化方面的相似性使得它们能够有效地结合，为开发者提供了在性能和安全性之间取得平衡的新途径。本报告将深入探讨这两种语言如何在实际项目中协同工作，以及它们如何通过工具和技术克服潜在的挑战。

## 方法与验证过程
本文使用的实验平台为虚拟机(Ubuntu 64位操作系统)。具体的可行性验证过程如下：
### C代码调用Rust代码
**主要思路**：
- 创建静态库
- 编写需要被调用的rust代码
- 配置相关文件用来生成静态库文件
- 使用编译命令编译静态库
- 使用cbindgen工具生成静态库C语言头文件
- 新建C语言代码，引入静态库头文件，编译执行
  
#### 一、创建静态链接库
这里默认已经配置好了基本的rust环境(能进行常见cargo命令的执行、rustup相关库的安装等操作)。
rust在vscode的开发环境配置可以参考CSDN文章：https://blog.csdn.net/happylzs2008/article/details/108108830

创建静态链接库的第一步是：
在某个目录(这里是在 **/home/jiabino/Desktop/Rust** 路径)下，打开终端，使用命令行输入命令 ```cargo init --lib (库名称)```。这里执行命令 ```cargo init --lib Linklist```后，得到以下结构目录:

```
│   ...
├── Linklist                //库目录
│   ├── Cargo.toml          //Rust 项目的配置文件，用于管理项目构建、依赖项、版本信息和构建脚本
│   └── src                 //库目录源文件存放的文件夹
│       └── lib.rs          //待编译的库源文件
│   ...
```

#### 二、编写需要被调用的rust代码
这里以双向链表的改写作为示例，具体的rust代码实现如下：
```
#![crate_type = "staticlib"]    //表示这是一个静态库（static library），而非可执行文件
extern crate libc;              //引入C标准库，用于与C语言接口交互
use core::ffi::c_int;           //使用core模块的ffi模块，其中的c_int类型代表C语言中的整型

#[repr(C)]                      // 使用C语言的内存布局
pub struct Node{                //定义一个结构体Node，用于表示链表中的节点
    value: c_int,
    next: *mut Node,
    pre: *mut Node,
}

#[inline]                       //使用inline属性确保这个函数在编译时被内联
#[no_mangle]                    //告诉编译器不要为函数生成特定的名字，以便于与C语言接口兼容
pub extern "C" fn LOS_ListInit(list: &mut Node, node_value: c_int)
{
    list.next = list as *mut Node;
    list.pre = list as *mut Node;
    list.value = node_value;
    println!("List initialized. Head value: {:?}", list.value);
}
```

#### 三、配置相关文件用来生成静态库文件
打开目录下的```Cargo.toml```配置文件，默认内容为：
```
[package]
name = "Linklist"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```
其中```[package]```定义了项目的元数据，通常包括:
```
- name: 项目的名称，用于识别和下载。
- version: 项目的版本号，遵循 SemVer（Semantic Versioning）标准，如1.0.0。
- authors: 项目的作者或团队成员列表。
- license: 项目的许可证，如MIT、Apache等。
- edition: Rust的版本，如2018或2021。
- description: 项目的简短描述，用于仓库页面和文档。
- repository: 项目的源代码存储库地址，如GitHub、GitLab等。
- keywords: 项目的关键字，用于搜索和分类。
- readme: 项目的README文件路径，通常包含项目的介绍和安装指南。
- build: 包含构建设置，如构建命令、构建工具等。
```
而```dependencies```则定义了项目依赖的其他Rust包(crates)，通常包括：
```
- dependencies: 这部分定义了项目依赖的其他Rust包（crates）。每一行都包含一个依赖项的名称（通常是包名，如 "serde" 或 "tokio"），
  版本范围（如 "=1.0.0" 或 "^2.0"），以及可选的其他属性，如默认特征（default-features）、平台限制（target）、仓库地址（source）等。
- dev-dependencies: 用于开发和测试的依赖，这些包通常不会包含在最终发布的二进制文件中。
- features: 可选的特性开关，允许在编译时启用或禁用特定的依赖或功能。
```

将Cargo.toml配置文件修改如下：
```
[package]
name = "Linklist"           # 名称，用于识别项目
version = "0.1.0"           # 版本号，遵循 SemVer 标准
edition = "2021"            # Rust 语言的版本

[lib]
name = "rust_linklist"      # 库的名称，与Cargo.toml文件中的其他模块区分
crate-type = ["staticlib"]  # 指定生成的库类型，这里是静态库（staticlib）
path = "src/lib.rs"         # 指定源代码文件的位置，这里是 src 目录下的 lib.rs 文件

[dependencies]
libc = "0.2"                # "libc" 是一个依赖项，它是一个标准C库的Rust绑定,版本号为"0.2"
```

#### 四、使用编译命令编译静态库
在配置好```Cargo.toml```文件后，进入Linklist目录，在命令行输入：```cargo build --release```。
```
Tips:如果出现以下报错:
warning: spurious network error (3 tries remaining): [6] Couldn't resolve host name (Could not resolve host: index.crates.io)
warning: spurious network error (2 tries remaining): [6] Couldn't resolve host name (Could not resolve host: index.crates.io)
warning: spurious network error (1 tries remaining): [6] Couldn't resolve host name (Could not resolve host: index.crates.io)
极可能是网络出现问题，检查网络是否通畅
```
成功执行后会出现如下日志：
![alt text](image-2.png)
输入后Cargo编译器会对代码进行优化，生成静态库，并输出到 **/Linklist/target/release** 目录下。
执行完命令后目录结构变为：
```
│   ...
├── Linklist
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   └── lib.rs
│   └── target
│       ├── CACHEDIR.TAG      //元数据文件,用于标记一个目录是 cargo 缓存的一部分，以便在重新构建时，如果某些依赖项没有改变，可以重用已经下载的依赖，从而提高构建速度。
│       └── release
│           ├── build
│           │   ├── libc-01c76c0d0c0e01fd
│           │   │   ├── invoked.timestamp                       //记录了这个编译单元被触发的时间，用于追踪构建历史
│           │   │   ├── out
│           │   │   ├── output
│           │   │   ├── root-output                             //可能包含编译后的所有文件，如果编译生成了多个文件。
│           │   │   └── stderr                                  //编译过程中可能产生的错误输出，用于诊断问题。
│           │   └── libc-2b375d3ab6fa5f4b
│           │       ├── build-script-build                      //自定义的构建脚本，用于构建 libc 库，可能包含特定于库的构建步骤。
│           │       ├── build_script_build-2b375d3ab6fa5f4b
│           │       └── build_script_build-2b375d3ab6fa5f4b.d   //构建脚本的备份或中间文件。
│           ├── deps
│           │   ├── libc-2b347ce2db932a4a.d                     //包含了libc库的编译依赖信息，可能包含符号表和依赖关系。
│           │   ├── liblibc-2b347ce2db932a4a.rlib               //rlib（Rust Library）是编译后的静态库
│           │   ├── liblibc-2b347ce2db932a4a.rmeta              //rmeta是元数据文件，用于链接和调试。
│           │   ├── librust_linklist-179cbc5c93812de9.a         //rust_linklist 库的静态链接文件，用于编译时链接。
│           │   └── rust_linklist-179cbc5c93812de9.d            //rust_linklist 库的调试信息文件。
│           ├── examples
│           ├── incremental                                     //存储编译过程中的增量信息。
│           ├── librust_linklist.a                              //静态链接库文件，用于编译时链接到项目中，提供库的功能。
│           └── librust_linklist.d                              //调试信息文件，与 librust_linklist.a 相关，用于在调试时提供源代码级别的信息
│   ...
```

#### 五、使用cbindgen工具生成静态库C语言头文件
在完成静态链接库的编译后，新建```cbindgen.toml```文件，添加```language = "C"```，然后执行命令：
```
cbindgen --config cbindgen.toml --crate (库文件名称) --output (目标头文件名称)
```
执行成功后，目录将变化为：
```
│   ...
├── Linklist
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── cbindgen.toml               //新建的cbindgen配置文件
│   ├── rust_linklist.h             //生成的头文件
│   ├── src
│   │   └── lib.rs
│   └── target
│       ├── CACHEDIR.TAG
│       └── release
│           ├── ...
│   ...
```
查看头文件内的内容，已经变成了c语言风格的代码了。
![alt text](image-3.png)

#### 六、新建C语言代码，引入静态库头文件，编译执行
在```Linklist```目录下新建一个```main.c```文件用于测试```rust_linklist.h```头文件。```main.c```文件内容如下：
```
#include <stdio.h>
#include <malloc.h>
#include "rust_linklist.h"
int main()
{
    Node* p;
    p = (Node *)malloc(sizeof(Node));
    LOS_ListInit(p, 5);
    printf("%d", (*p).value);
    free(p);
    return 0;
}
```
输入命令对文件进行编译：
```gcc -o (可执行文件名字) (测试程序名字) -I. -Ltarget/release -l(库名称) -ldl -lpthread -Wl,-gc-section```
![alt text](image-4.png)
目录变化为：
```
│   ...
├── Linklist
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── cbindgen.toml
│   ├── main                //编译生成的可执行文件
│   ├── main.c              //测试程序
│   ├── rust_linklist.h
│   ├── src
│   │   └── lib.rs
│   └── target
│       ├── ...
│   ...
```
执行```main```文件，输出结果为：
![alt text](image-5.png)

至此，一个简单的C文件调用Rust文件的过程完成了。

### Rust代码调用C代码
**主要思路**:

- 编写需要被调用的C语言源文件
- 编写构建脚本```build.rs```
- 配置```Cargo.toml```文件
- 编写rust主函数内容
- 构建，验证
  
#### 一、编写需要被调用的C语言源文件
本文编写斐波那契数列计算的递归函数作为测试文件。源代码如下：
```
//fibonacci.h
#ifndef FIBONACCI_H
#define FIBONACCI_H

#ifdef __cplusplus
extern "C" {
#endif

int fibonacci(int n);   // 函数声明

#ifdef __cplusplus
}
#endif

#endif // FIBONACCI_H

/*********************************************************************/

//fibonacci.c

#include "fibonacci.h"

int fibonacci(int n) {
    if (n <= 1) {
        return n;
    } 
    else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
```
#### 二、编写构建脚本```build.rs```
```build.rs``` 是Rust项目中的一个特殊文件，它是一个编译脚本，通常用于执行编译过程中的构建步骤，比如编译C/C++源文件、生成Rust绑定、构建依赖等。```build.rs``` 的作用主要有以下几点：

1. **自定义构建过程**：当你需要在编译Rust项目时执行额外的构建步骤，比如编译C/C++代码、生成头文件（`.h`）或静态库（`.a`），`build.rs` 就派上用场。例如，上面的 `cc::Build::new()` 代码就是在 `build.rs` 中编译C源文件。

2. **依赖管理**：如果你的项目依赖于其他非Rust库，如C或C++库，`build.rs` 可以帮助管理这些依赖，确保它们被正确编译和链接。

3. **生成Rust绑定**：`cc` 等库允许你在 `build.rs` 中生成Rust绑定，使得C/C++函数可以直接在Rust中使用，无需 `unsafe` 代码块。

4. **构建配置**：`build.rs` 可以根据不同的环境（如不同的平台、构建配置等）执行不同的构建逻辑，提供更灵活的构建选项。

5. **自定义输出**：你可以通过 `build.rs` 控制编译后的输出，比如生成的库文件、可执行文件等。

编写完源文件后，将c文件放入src目录下，在项目一级目录下新建```build.rs```文件，文件目录如图：
```
│   ...
├── c_to_rust_test
│   ├── build.rs
│   ├── Cargo.toml
│   └── src
│       ├── fibonacci.c
│       ├── fibonacci.h
│       └── main.rs
│   ...
```
在```build.rs```文件内编辑内容：
```
extern crate cc;    //Rust编译器插件(Cargo crate),允许从Rust项目中编译C和C++代码

fn main() {
    cc::Build::new().file("src/fibonacci.c").compile("libfibonacci.a");
    //编译 src/fibonacci.c 文件，并生成一个名为 libfibonacci.a 的静态库。
}
```
#### 三、配置Cargo.toml文件
```Cargo.toml```文件的内容如下：
```
[package]
name = "c_to_rust_test"
version = "0.1.0"
edition = "2021"
build = "build.rs"          \\告知需要使用build.rs文件进行提前构建

[dependencies]              \\main.rs执行时依赖的库
libc = "0.2"

[build-dependencies]        \\build.rs执行时依赖的库
cc = "1.0"
```

#### 四、编写rust主函数内容
为了对C语言静态库进行验证，Rust主函数```main.rs```测试代码如下：
```
extern crate libc;                                                 
//引入libc库，包含标准库函数与类型定义

extern {       
    fn fibonacci(n: libc::c_int) -> libc::c_int;                    
}
//外部声明：引入名为fibonacci的C函数，它接受一个c_int类型的参数并返回c_int类型

fn main()
{
    for i in 1..10{
        println!("fibonacci({:?}) = {:?}", i, unsafe{fibonacci(i)});
    }
    //由于C函数的类型和安全性以及Rust默认不支持直接调用C函数，需要unsafe块来调用
}
```

#### 五、构建，验证
在项目目录下输入命令```cargo build```命令。
![alt text](image-6.png)
得到以下目录：
```
│   ...
├── c_to_rust_test
│   ├── build.rs
│   ├── Cargo.lock
│   ├── Cargo.toml
│   ├── src
│   │   ├── fibonacci.c
│   │   ├── fibonacci.h
│   │   └── main.rs
│   └── target
│       ├── CACHEDIR.TAG
│       └── debug
│           ├── build
│           │   │   ...
│           ├── c_to_rust_test                      //生成的可执行文件
│           ├── c_to_rust_test.d
│           ├── deps
│           │   │   ...
│           ├── examples
│           └── incremental
│               ├── build_script_build-ltpi8hdq3ehx
│               │   │   ...
│               └── c_to_rust_test-1omcoksitrz7y
│                   │   ...
│   ...
```
在命令行输入命令```./target/debug/c_to_rust_test```,执行结果为：
![alt text](image-7.png)

至此，Rust文件调用C静态库的过程完成。

## 总结


经过验证，C与Rust语言的交互展现了两种截然不同但互补的编程范式之间的无缝融合。C语言以其高效和底层控制而闻名，而Rust则以内存安全和并发性为特点。两者结合，为开发者提供了丰富的选择和灵活性。

从简单的函数调用开始，通过`cc`库或`bindgen`工具，Rust可以轻松地调用C函数，实现代码的复用和性能优化。然而，这需要谨慎处理，特别是在`unsafe`块中，以确保内存安全和正确性。反之，Rust的FFI（ Foreign Function Interface）允许C调用Rust，但同样需要关注内存管理和错误处理。

在条件编译方面，C和Rust的编译器特性可以协同工作，根据目标平台或特定条件编译不同的代码路径。这有助于优化性能和适应不同的环境需求。

性能方面，C调用Rust可能会带来一定的性能损失，但通过精心设计和优化，可以减小影响。同时，Rust的性能优化工具和设计哲学也对C-Rust交互的性能有着积极影响。

内存安全是两者交互的关键，Rust的`NonNull`和`Drop`机制在调用C时提供了额外的保护，而C开发者则需要遵循最佳实践，避免潜在的内存问题。

总结来说，C与Rust的交互是一个强大的工具组合，它允许开发者在保持高效性的同时，享受Rust的现代编程特性。然而，这种交互也要求开发者具备深厚的理解和谨慎处理，以确保代码的可靠性和安全性。随着技术的不断发展，我们期待看到更多创新的C-Rust集成解决方案，推动软件开发的边界。

## 参考资料
https://blog.csdn.net/phthon1997/article/details/126469708
https://www.cnblogs.com/renxinyuan/articles/15830009.html