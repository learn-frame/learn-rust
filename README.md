# Learn Rust

| Logo                                                                                             | Ferris                                                                                        | Me                                                                                            |
| ------------------------------------------------------------------------------------------------ | --------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| <img src="https://edge.yancey.app/beg/6w9ggjnu-1677482302383.png" alt="rust-logo" height="160"/> | <img src="https://edge.yancey.app/beg/zrwdrsxd-1677482047553.png" alt="ferris" height="160"/> | <img src="https://edge.yancey.app/beg/ckk43ylh-1677482091462.jpg" alt="ferris" height="160"/> |

## Rust 初衷

传统语言很难编写内存安全和线程安全的代码, 如 20 世纪 90 年代, 欧洲空间局阿丽亚娜五号运载火箭发射失败, 原因是 Ada 语言在将 64 位浮点数转换为 16 位无符号整数时发生了溢出.

现代语言层面, 性能不是瓶颈(当然 Rust 的性能也很牛), 安全性才是, 因此 Rust 初衷是:

- 必须是更加安全, 不易崩溃的, 尤其在操作内存时候;
- 不需要有垃圾回收这样的系统, 不能为了内存安全而引入性能负担;
- 不是一门仅仅拥有一个主要特性的语言, 而应该拥有一系列的广泛特性, 这些特性之间又不乏一致性. 这些特性可以很好地相互协作, 从而使该语言更容易编写, 维护和调试, 让程序员写出更安全, 更高效的代码.

## Rust 类型系统

Rust 是一门强类型且类型安全的静态语言. Rust 中一切皆表达式, 表达式皆有值, 值皆有类型. 所以可以说, Rust 中一切皆类型. 除了一些基本的原生类型和复合类型, Rust 把作用域也纳入了类型系统(生命周期系统).
为了安全, Rust 囊括了编程中会遇到的各种情况, 如 `Option<T>`, `Result<T, E>`, `never` 类型.

### 类型系统的分类

在编译期进行类型检查的语言属于**静态类型**, 在运行期进行类型检查的语言属于**动态类型**. 如果一门语言不允许类型的自动隐式转换, 在强制转换前不同类型无法进行计算, 则该语言属于**强类型**, 反之则属于**弱类型**.

### 多态性与类型系统

对象是什么样的类型, 决定了它有什么样的行为; 反过来, 对象在不同上下文中的行为, 也决定了它的类型. 这称为**多态性**. 如果一个类型系统允许一段代码在不同的上下文中具有不同的类型, 这样的类型系统就叫作多态类型系统. 对于静态类型的语言来说, 多态性的好处是可以在不影响类型丰富的前提下, 为不同的类型编写通用的代码.

现代编程语言包含了三种多态形式: 参数化多态(Parametric polymorphism), Ad-hoc 多态 (Ad-hoc polymorphism)和子类型多态(Subtype polymorphism). 如果按多态发生的时间来划分, 又可分为静多态(Static Polymorphism)和动多态(Dynamic Polymorphism).

静多态发生在编译期, 动多态发生在运行时. 参数化多态和 Ad-hoc 多态一般是静多态, 子类型多态一般是动多态. 静多态牺牲灵活性获取性能, 动多态牺牲性能获取灵活性. Rust 语言同时支持静多态和动多态, 静多态就是一种零成本抽象.

- **参数化多态**实际就是指泛型
- **Ad-hoc 多态**也叫特定多态: Ad-hoc 多态是指同一种行为定义, 在不同的上下文中会响应不同的行为实现, Rust 的 trait 系统来支持 Ad-hoc 多态.
- **子类型多态**的概念一般用在面向对象语言中, 尤其是 Java 语言. Java 语言中的多态就是子类型多态, 它代表一种包含关系, 父类型的值包含了子类型的值, 所以子类型的值有时也可以看作父类型的值, 反之则不然.

Rust 中的类型系统目前只支持参数化多态和 Ad-hoc 多态, 也就是泛型和 trait.

### 零成本抽象

| In general, C++ implementations obey the zero-overhead principle: What you don’t use, you don‘t pay for. And further: What you do use, you couldn’t hand code any better.
|
| -- Bjarne Stroustrup "Foundations of C++"

一般而言, 构成一个零成本抽象包含以下几个要素:

- 没有全局成本: 零成本抽象不应该对不使用该功能的程序的性能产生负面影响. 例如, 它不能要求每个程序都携带一个沉重的语言运行时(runtime), 以使唯一使用该功能的程序模块受益.
- 最佳的性能: 一个零成本的抽象应该编译成相当于底层指令编写的最佳实现. 它不能引入额外的成本, 而这些成本在没有抽象的情况下是可以避免的.
- 改善开发者的体验: 抽象的意义在于提供一个新的工具, 由底层组件组装而成, 让开发者更容易写出他们想写的代码. 零成本抽象, 如同所有其他抽象一样, 一定要比其他方法提供更好的使用体验.

### Rust 的零成本的抽象对象

- **所有权和借用**. 当然这个是最大的一个, 在没有垃圾回收器的情况下, 保证内存和线程的安全, 是 Rust 最初的最大的成功故事.
- **迭代器和闭包 API**. 这又是一个经典之作. 虽然有些情况下, 内部迭代可能会优化得更好, 但你可以在切片上写 map, filter, 循环迭代等, 它可以优化到相当于一些手写的 C 语言, 这绝对是令人震惊的.
- **Async/await 和 Future**. Futures API 是一个重要的例子, 因为早期版本的 futures 在零成本抽象的"零成本"部分做得非常好, 但实际上并没有提供足够好的用户体验来吸引用户采用. 通过增加 pining 支持 async/await, 跨 await 的引用等等, 我们做了一个真的可以解决用户的问题的产品, 让 Rust 更适合编写高性能的网络服务.
- **Unsafe 和模块边界**. 在所有这些, 以及 Rust 的每一个成功案例的背后, 都是不安全块和隐私的概念, 让我们可以侵入到原始指针操作中去构建这些零成本的抽象. 如果没有这种真正根本性的能力, 任何一个 Rust 的辉煌成就都是不可能实现的, 因为我们可以在本地环境打破规则, 将系统扩展到类型检查器所能处理的范围之外. 这就是零成本抽象, 它是 Rust 中所有其他零成本抽象的基础.

### Rust 的类型大小

编程语言中不同的类型本质上是内存占用空间和编码方式的不同, Rust 也不例外. Rust 中没有 GC, 内存首先由编译器来分配, Rust 代码被编译为 LLVM IR, 其中携带了内存分配的信息. 所以编译器需要事先知道类型的大小, 才能分配合理的内存.

Rust 中绝大部分类型都是在编译期可确定大小的类型(Sized Type), Rust 也有少量的动态大小的类型(Dynamic Sized Type, DST). 如 &str 存储于栈上, str 字符串序列存储于堆上, &str 由两部分组成: 指针和长度信息, 这样一 来, &str 就变成了可确定大小的类型, 编译器就可以正确地为其分配栈内存空间, str 也会在运行时在堆上开辟内存空间. 这种包含了动态大小类型地址信息和携带了长度信息的指针，叫作**胖指针**(Fat Pointer).

## 内存管理

编程语言分为手动管理内存语言(如 C 语言)和自动管理内存语言(如 JavaScript, Java 等). 手动内存管理类需要开发者手动使用 malloc 和 free 等函数显式管理内存, 后者则使用垃圾回收机制.

手动内存管理的优势在于性能, 因为可以人工操纵. 但随着代码量的增多很容易忘记释放内存, 造成内存泄漏. 另外一个问题就是容易造成悬垂引用, 即如果某个指针引用的内存被非法释放掉了, 而该指针却依旧指向被释放的内存, 这种情况下的指针就叫悬垂指针. 而垃圾回收可以避免这些问题, 但会造成全停顿问题, 具体可以看[从 JavaScript 编译原理到作用域(链)及闭包](https://yanceyleo.com/post/2ef92b49-b3d4-43bb-983a-771912265f6c)这篇文章. 而 Rust 则采用了所有权和生命周期的思想.

### 虚拟内存

现代操作系统在保护模式下都采用虚拟内存管理栈术. 虚拟内存是一种对物理存储设备的统一抽象, 其中物理在储设备包括物理内存, 磁盘, 寄存器, 高速缓存等. 这样统一抽象的好处是, 方便同时运行多道程序, 使得每个进程都有各自独立的进程地址空间, 并且可以通过操作系统调度将外存当作内存来使用. 这就引出了一个新的概念: **虚拟地址空间**.

虚拟地址空间是线性空间, 用户所接触到的地址都是虚拟地址, 而不是真实的物理地址. 利用这种虚拟地址不但能保护操作系统, 让进程在各自的地址空间内操作内存, 更重要的是, 用户程序可以使用比物理内存更大的地址空间. 虚拟地址空间被人为地分为两部分: 用户空间和内核空间, 它们的比例是 3:1(Linux 系统中)或 2:2(Windows 系统中). 以 Linux 系统为例, 32 位计算机的地址空间大小是 4GB, 寻址范围是 OxOOOOOOOO-OxFFFFFFFF. 然后通过内存分页等底层复杂的机制来把虚拟地址翻译为物理地址.

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

### 内存安全

只有当程序访问未定义内存的时候才会产生内存错误. 一般来说, 发生以下几种情况就会产生内存错误:

- 引用空指针
- 使用未初始化内存
- 释放后使用, 也就是使用悬垂指针
- 缓冲区滥出, 比如数组越界.
- 非法释放已经释放过的指针或未分配的指针, 也就是重复释放.

这些情况之所以会产生内存错误, 是因为它们都访问了未定义内存. 为了保证内存安全, Rust 语言建立了严格的安全内存管理模型:

- **所有权系统**: 每个被分配的内存都有一个独占其所有权的指针. 只有当该指针被销毁时, 其对应的内存才能随之被释放.
- **借用和生命周期**: 每个变量都有其生命周期, 一旦超出生命周期, 变量就会被自动释放. 如果是借用, 则可以通过标记生命周期参数供编译器检查的方式, 防止出现悬垂指针, 也就是释放后使用的情况.

## Rust 编译原理概览

Rust 是跨平台语言, 一次编译, 到处运行, 这得益于 LLVM. Rust 编译器是一个 LLVM 编译前端, 它将代码编译为 LLVM IR, 然后经过 LLVM 编译为相应的平台目标.

Rust 源码经过分词和解析, 生成 AST. 然后把 AST 进一步简化处理为 HIR (High-level IR), 目的是让编译器更方便地做类型检查. HIR 会进一步被编译为 MIR (Middle IR), 这是一种中间表示, 它在 Rust 1.12 版本中被引入, 主要用于以下目的:

- **缩短编译时间**. MIR 可以帮助实现增量编译, 当你修改完代码重新编译的时候, 编译器只 算更改过的部分, 从而缩短了编译时间.
- **缩短执行时间**. MIR 可以在 LLVM 编译之前实现更细粒度的优化, 因为单纯依赖 LLVM 的优化粒度太粗, 而且 Rust 无法控制, 引入 MIR 就增加了更多的优化空间.
- **更精确的类型检查**. MIR 将帮助实现更灵活的借用检查, 从而可以提升 Rust 的使用体验.

最终, MIR 会被翻译为 LLVM IR, 然后被 LLVM 的处理编译为能在各个平台上运行的目标机器码.

## Cargo 命令一览

- `cargo fmt`: 自动格式化
- `cargo fix`: 自动修复代码
- `cargo clippy`: 更严格的 lint

## 好用的三方包和库

### 监听代码变化

毕竟写前端出身的, 没有热更新可受不了, 虽然后端项目没有热更新一说, 但代码改了之后能够自动重启多少也是一件省力的事情, 可以安装 [cargo-watch](https://crates.io/crates/cargo-watch)

```bash
cargo install cargo-watch
cargo watch -x run
```

### 深入 cargo install

`cargo install` 安装的**是包不是库**, 它安装的是一个二进制的产物, 安装到 `/Users/XXX/.cargo/bin/` 底下, 有点类似于 `npm install -g xxx`.

而你在 Cargon.toml 里安装的是库, 它是不能够使用 `cargo install` 的, 土鳖的办法是手动填写依赖和版本, 但实在太 low 了, 使用 [cargo-edit](https://crates.io/crates/cargo-edit) 可以解决这个问题.

```bash
cargo install cargo-edit

# 它扩展了 cargo, 后续可以优雅的使用如下命令来安装库
cargo add regex
```

## Menu

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
- Rust 中的函数式语言功能: 迭代器与闭包
  - [iterators](./iterators_closures/src/iterators.rs)
  - [closures](./iterators_closures/src/closures.rs)
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

## License

Learn Rust is open source software licensed as [MIT](https://opensource.org/licenses/MIT).
