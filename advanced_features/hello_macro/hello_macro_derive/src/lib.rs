#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

extern crate proc_macro;

use proc_macro::TokenStream; // proc_macro crate 定义并表示令牌序列
use quote::quote; // syn crate 将字符串中的 Rust 代码解析成为一个可以操作的数据结构
use syn; // quote 则将 syn 解析的数据结构转换回 Rust 代码

#[proc_macro_derive(HelloMacro)]
// 定义过程宏的函数以 TokenStream 作为输入并生成 TokenStream 作为输出
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    // gen 用于生成内部的代码
    let gen = quote! {
        // #name 是模版机制, quote! 会以名为 name 的变量值来替换它
        // 比如真正是为 struct Pancakes 实现 HelloMacro
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    eprintln!("attr: {:#?}", attr);
    eprintln!("item: {:#?}", item);
    item
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}