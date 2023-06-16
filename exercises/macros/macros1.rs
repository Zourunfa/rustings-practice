// macros1.rs
// Execute `rustlings hint macros1` or use the `hint` watch subcommand for a hint.

/*
Rust 宏是一种代码生成工具，它可以用于生成重复的或模板化的代码。宏可以帮助我们减少代码量，
提高代码的可读性和可维护性，同时也可以提高代码的执行效率。

Rust 宏有两种类型：声明式宏和过程式宏。

声明式宏是一种基于模式匹配的宏，它使用 macro_rules! 宏来定义。
声明式宏可以匹配不同的模式，并生成不同的代码。它可以用于生成重复的代码、
简化代码结构、
提高代码可读性等。声明式宏的缺点是它的匹配规则比较复杂，容易出现错误。

过程式宏是一种基于函数调用的宏，它使用 proc_macro crate 来定义。
过程式宏可以将代码作为输入，生成新的代码作为输出。它可以用于代码转换、
语法扩展、代码检查等。过程式宏的优点是它的匹配规则比较简单，容易理解和使用。

 */
// I AM NOT DONE

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
