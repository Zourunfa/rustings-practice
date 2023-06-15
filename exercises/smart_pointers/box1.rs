// box1.rs
//
// At compile time, Rust needs to know how much space a type takes up. This becomes problematic
// for recursive types, where a value can have as part of itself another value of the same type.
// To get around the issue, we can use a `Box` - a smart pointer used to store data on the heap,
// which also allows us to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the `cons list` - a data structure
// frequently found in functional programming languages. Each item in a cons list contains two
// elements: the value of the current item and the next item. The last item is a value called `Nil`.
//
// Step 1: use a `Box` in the enum definition to make the code compile
// Step 2: create both empty and non-empty cons lists by replacing `todo!()`
//
// Note: the tests should not be changed
//
// Execute `rustlings hint box1` or use the `hint` watch subcommand for a hint.

/*
在编译时，Rust 需要知道一个类型占用多少空间。
这对于递归类型来说会有问题，因为一个值可能包含另一个相同类型的值。
为了解决这个问题，我们可以使用 Box - 一种智能指针，用于在堆上存储数据，
同时也允许我们包装递归类型。

在这个练习中，我们要实现的递归类型是 cons list -
一种在函数式编程语言中经常出现的数据结构。每个元素包含两个部分：
当前元素的值和下一个元素。最后一个元素是一个称为 Nil 的值。

在 Rust 中，递归类型需要使用 Box 或者其他智能指针来实现。这是因为 Rust 需要在编译时知道类型的大小，而递归类型的大小是无限的，无法得知。
使用 Box 可以将递归类型的所有元素都存储在堆上，从而避免了大小无限的问题。
 */

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil
}

pub fn create_non_empty_list() -> List {
    List::Cons(2, Box::new(List::Nil))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
