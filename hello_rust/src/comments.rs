//! 一般放在顶部, 用于解释整个文档, 例子如下
//!
//! Constants specific to the `f32` single-precision floating point type.
//!
//! *[See also the `f32` primitive type][f32].*
//!
//! Mathematically significant numbers are provided in the `consts` sub-module.
//!
//! For the constants defined directly in this module
//! (as distinct from those defined in the `consts` sub-module),
//! new code should instead use the associated constants
//! defined directly on the `f32` type.

/// 用来注释一些方法, 结构体之类的, Dart 也有! 用于生成结构化的文档, 例子如下:
///
/// The radix or base of the internal representation of `f32`.
/// Use [`f32::RADIX`] instead.
///
/// # Examples
///
/// ```rust
/// // deprecated way
/// # #[allow(deprecated, deprecated_in_future)]
/// let r = std::f32::RADIX;
///
/// // intended way
/// let r = f32::RADIX;
/// ```



// 单行注释

/*
* 块注释
*/