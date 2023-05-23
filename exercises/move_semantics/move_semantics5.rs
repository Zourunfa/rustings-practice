// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

/**
 *
 * Rust 的 borrow checker 在编译时会检查在哪些位置对可变引用进行借用和修改，以确保 Rust 代码的线程安全性。

在上述代码中，第一个可变引用在声明后立即被用于赋值给变量 y，第二个可变引用在声明后被用于赋值给变量 z。这两个可变引用都指向变量 x 的相同内存位置。根据 borrow checker 的规则，在给定的时间内，只有一个可变引用可以指向一块特定的内存。

因此，当第一个可变引用被借用时，编译器不允许另一个可变引用对同一块内存进行借用和修改，这导致编译错误。

为了解决这个问题，您可以在代码中分配两个不同的 i32 变量，并分别将它们借用给 y 和 z。或者，您可以将 y 和 z 引用分别移到不同的作用域，并确保同一时间只有一个引用是活动的
 *
 *
 *
 */

/**简单来说就是编译器不允许指向同意地址的引用同时借用修改修改 */
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;

    assert_eq!(x, 1200);
}
