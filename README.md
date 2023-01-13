# Learn Rust

~~蟹妖, 刚下飞机, 人已生锈.~~ Rust 虐我千百遍, 我待 Rust 如初恋.

| Rust                                                                        | Ferris                                                            |
| --------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| <img src="./public/assets/rust-logo-blk.svg" alt="rust-logo" height="160"/> | <img src="./public/assets/ferris.png" alt="ferris" height="160"/> |

## Rust 初衷

传统语言很难编写内存安全和线程安全的代码, 如 20 世纪 90 年代, 欧洲空间局阿丽亚娜五号运载火箭发射失败, 原因是 Ada 语言在将 64 位浮点数转换为 16 位无符号整数时发生了溢出.

现代语言层面, 性能不是瓶颈(当然 Rust 的性能也很牛), 安全性才是, 因此 Rust 初衷是:

- 必须是更加安全, 不易崩溃的, 尤其在操作内存时候;
- 不需要有垃圾回收这样的系统, 不能为了内存安全而引入性能负担;
- 不是一门仅仅拥有一个主要特性的语言, 而应该拥有一系列的广泛特性, 这些特性之间又不乏一致性. 这些特性可以很好地相互协作, 从而使该语言更容易编写, 维护和调试, 让程序员写出更安全, 更高效的代码.

## 内存管理

编程语言分为手动管理内存语言(如 C 语言)和自动管理内存语言(如 JavaScript, Java 等). 手动内存管理类需要开发者手动使用 malloc 和 free 等函数显式管理内存, 后者则使用垃圾回收机制.

手动内存管理的优势在于性能, 因为可以人工操纵. 但随着代码量的增多很容易忘记释放内存, 造成内存泄漏. 另外一个问题就是容易造成悬垂引用, 即如果某个指针引用的内存被非法释放掉了, 而该指针却依旧指向被释放的内存, 这种情况下的指针就叫悬垂指针. 而垃圾回收可以避免
这些问题, 但会造成全停顿问题, 具体可以看[从 JavaScript 编译原理到作用域(链)及闭包](https://yanceyleo.com/post/2ef92b49-b3d4-43bb-983a-771912265f6c)这篇文章. 而 Rust 则采用了所有权和生命周期的思想.

### 虚拟内存

现代操作系统在保护模式下都采用虚拟内存管理栈术. 虚拟内存是一种对物理存储设备的统一抽象, 其中物理在储设备包括物理内存, 磁盘, 寄存器, 高速缓存等. 这样统一抽象的好处是, 方便同时运行多道程序, 使得每个进程都有各自独立的进程地址空间, 并且可以通过操作系统调度将外存当作内存来使用. 这就引出了一个新的概念: **虚拟地址空间**.

如下是 Linux 系统中的虚拟地址空间示意图, 其中栈内存由高地址向低地址变化, 即随着入栈, 地址的十六进制数是减小的; 而堆内存则相反, 是由低地址向高地址, 这样的设计是为了更加有效地利用内存.

![Linux 系统中的虚拟地址空间示意图](https://edge.yancey.app/beg/0jw8h4gh-1669486736532.png)

### 栈内存和堆内存

关于栈不多说, 后进先出, 操作栈的一端被称为栈顶, 相反的一端被称为栈底. 物理内存本身并不区分堆和栈, 但是虚拟内存空间需要分出一部分内存, 用于支持 CPU 入栈或出栈的指令操作, 这部分内存空间就是栈内存.

栈顶由栈指针寄存器 ESP 保存, 起初栈顶指向栈底的位置, 当有数据入栈时, 栈顶地址向下增长, 地址由高地址变成低地址; 当有数据被弹出时, 栈顶地址向上增长, 地址由低地址变成高地址. 因此, 降低 ESP 的地址等价于开辟横空间, 增加 ESP 的地址等价于回收栈空间.

**栈内存最重要的作用是在程序运行过程中保存函数调用所要维护的信息**. 存储每次函数调用所需信息的记录单元被称为栈帧(Stack Record), 有时也叫活动记录(Activate Record).

栈帧一般包括三方面的内容:

- 函数的返回地址和参数;
- 临时变量, 包括函数内部的非静态局部变量和编译器产生的临时变量;
- 保存的上下文.

## 内存安全

只有当程序访问未定义内存的时候才会产生内存错误. 一般来说, 发生以下几种情况就会产生内存错误:

- 引用空指针
- 使用未初始化内存
- 释放后使用, 也就是使用悬垂指针
- 缓冲区滥出, 比如数组越界.
- 非法释放已经释放过的指针或未分配的指针, 也就是重复释放.

这些情况之所以会产生内存错误, 是因为它们都访问了未定义内存. 为了保证内存安全, Rust 语言建立了严格的安全内存管理模型:

- **所有权系统**: 每个被分配的内存都有一个独占其所有权的指针. 只有当该指针被销毁时, 其对应的内存才能随之被释放.
- **借用和生命周期**: 每个变量都有其生命周期, 一旦超出生命周期, 变量就会被自动释放. 如果是借用, 则可以通过标记生命周期参数供编译器检查的方式, 防止出现悬垂指针, 也就是释放后使用的情况.

## Rust 编译概览

Rust 是跨平台语言, 一次编译, 到处运行, 这得益于 LLVM. Rust 编译器是一个 LLVM 编译前端, 它将代码编译为 LLVM IR, 然后经过 LLVM 编译为相应的平台目标.

Rust 源码经过分词和解析, 生成 AST. 然后把 AST 进一步简化处理为 HIR (High-level IR), 目的是让编译器更方便地做类型检查. HIR 会进一步被编译为 MIR (Middle IR), 这是一种中间表示, 它在 Rust 1.12 版本中被引入, 主要用于以下目的:

- **缩短编译时间**. MIR 可以帮助实现增量编译, 当你修改完代码重新编译的时候, 编译器只 算更改过的部分, 从而缩短了编译时间.
- **缩短执行时间**. MIR 可以在 LLVM 编译之前实现更细粒度的优化, 因为单纯依赖 LLVM 的优化粒度太粗, 而且 Rust 无法控制, 引入 MIR 就增加了更多的优化空间.
- **更精确的类型检查**. MIR 将帮助实现更灵活的借用检查, 从而可以提升 Rust 的使用体验.

最终, MIR 会被翻译为 LLVM IR, 然后被 LLVM 的处理编译为能在各个平台上运行的目标机器码.

## Available Scripts

- `cargo fmt`: 自动格式化
- `cargo fix`: 自动修复代码
- `cargo clippy`: 更严格的 lint

## Recipe

### 热更新

```bash
cargo install cargo-watch
cargo watch -x run
```

## Menu

- 入门指南
  - [hello_world](./hello_world)
  - [hello_cargo](./hello_cargo)
- 猜猜看游戏
  - [guessing_game](./guessing_game/src/main.rs)
  - [guessing_game_with_comments](./guessing_game/src/dev.rs)
- 常见编程概念
  - [variables_and_mutability](./variables_and_mutability)
  - [data_types](./data_types)
  - [how_functions_work](./functions)
  - [comments](./comments)
  - [control_flow](./control_flow)
- 认识所有权
  - [understanding_ownership](./ownership/src/main.rs)
  - [references_and_borrowing](./ownership/src/references_and_borrowing.rs)
  - [return_value](./ownership/src/return_value.rs)
  - [slice](./ownership/src/slice.rs)
- 使用结构体来组织相关联的数据
  - [structs](./structs/src/main.rs)
  - [method_syntax](./structs/src/method_syntax.rs)
- 枚举和模式匹配
  - [enums](./enums/src/main.rs)
  - [match_control_flow_operator](./enums/src/match_control_flow_operator.rs)
- 使用包, Crate 和模块管理不断增长的项目
  - [lib](./restaurant/src/lib.rs)
  - [pub_struct](./restaurant/src/use_struct.rs)
- 常见集合
  - [vector](./collections/src/vector.rs)
  - [string](./collections/src/string.rs)
  - [hashmap](./collections/src/hashmap.rs)
- 错误处理
  - [recoverable_errors_with_result](./error_handling/src/recoverable_errors_with_result.rs)
  - [unrecoverable_errors_with_panic](./error_handling/src/unrecoverable_errors_with_panic.rs)
- 泛型, trait 和生命周期
  - [generics](./generics_traits_lifetimes/src/generics.rs)
  - [traits](./generics_traits_lifetimes/src/traits.rs)
  - [lifetimes](./generics_traits_lifetimes/src/lifetimes.rs)
- 编写自动化测试
  - [how_to_write_tests](./writing_automated_tests/src/how_to_write_tests.rs)
  - [how_to_run_tests](./writing_automated_tests/src/how_to_run_tests.rs)
  - [how_to_organize_tests_files](./writing_automated_tests/src/how_to_organize_tests_files.rs)
- 一个 I/O 项目：构建一个命令行程序
  - [main](./minigrep/src/main.rs)
  - [lib](./minigrep/src/lib.rs)
- Rust 中的函数式语言功能: 迭代器与闭包
  - [iterators](./iterators_closures/src/iterators.rs)
  - [closures](./iterators_closures/src/closures.rs)
- 更多关于 Cargo 和 Crates.io 的内容
  - [more_command_of_cargo](./more_about_cargo)
  - [workspace](./workspace)
- 智能指针
  - [box_pointer](./smart_pointers/src/box_pointer.rs)
  - [deref_trait](./smart_pointers/src/deref_trait.rs)
  - [drop_trait](./smart_pointers/src/drop_trait.rs)
  - [rc_pointer](./smart_pointers/src/rc_pointer.rs)
  - [refcell_pointer_and_interior_mutability](./smart_pointers/src/refcell_pointer_and_interior_mutability.rs)
  - [reference_cycles](./smart_pointers/src/reference_cycles.rs)
- 无畏并发
  - [threads](./concurrency/src/threads.rs)
  - [message_passing](./concurrency/src/message_passing.rs)
  - [shared_state](./concurrency/src/shared_state.rs)
- Rust 的面向对象编程特性
  - [main](./oop/src/main.rs)
  - [design_patterns]('./oop/src/design_patterns.rs')
- 模式与模式匹配
  - [all_the_places_for_patterns](./patterns/src/all_the_places_for_patterns.rs)
  - [refutability](./patterns/src/refutability.rs)
  - [pattern_grammer](./patterns/src/pattern_grammer.rs)
- 高级特性
  - [unsafe_rust](./advanced_features/src/unsafe_rust.rs)
  - [advanced_traits](./advanced_features/src/advanced_traits.rs)
  - [advanced_types](./advanced_features/src/advanced_types.rs)
  - [advanced_functions_and_closures](./advanced_features/src/advanced_functions_and_closures.rs)
  - [macros](./advanced_features/src/macros.rs)
- 带线程池的 Web Server
  - [main](./web_server/src/main.rs)
  - [lib](./web_server/src/lib.rs)

## License

Learn Rust is open source software licensed as [MIT](https://opensource.org/licenses/MIT).
