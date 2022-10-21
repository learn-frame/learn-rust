# Learn Rust

~~蟹妖, 刚下飞机, 人已生锈.~~ Rust 虐我千百遍, 我待 Rust 如初恋.

| Rust                                                                        | Ferris                                                            |
| --------------------------------------------------------------------------- | ----------------------------------------------------------------- |
| <img src="./public/assets/rust-logo-blk.svg" alt="rust-logo" height="160"/> | <img src="./public/assets/ferris.png" alt="ferris" height="160"/> |

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

- 缩短编译时间. MIR 可以帮助实现增量编译, 当你修改完代码重新编译的时候, 编译器只 算更改过的部分, 从而缩短了编译时间.
- 缩短执行时间. MIR 可以在 LLVM 编译之前实现更细粒度的优化, 因为单纯依赖 LLVM 的优化粒度太粗, 而且 Rust 无法控制, 引入 MIR 就增加了更多的优化空间.
- 更精确的类型检查. MIR 将帮助实现更灵活的借用检查, 从而可以提升 Rust 的使用体验.

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
