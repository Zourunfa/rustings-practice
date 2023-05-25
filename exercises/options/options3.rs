// options3.rs
// Execute `rustlings hint options3` or use the `hint` watch subcommand for a hint.

#[derive(Clone)]
struct Point {
    x: i32,
    y: i32,
}
/**
 *
 * y 变量已经被移动（moved），并且不能再次使用。
 * Rust 的所有权与借用系统规定了变量的使用方式，
 * 变量只能被一个所有权拥有者（ownership holder）拥有，
 * 而不能被多个所有权拥有者共享，也不能被无权使用者（unauthorized user）使用。
 * 这是为了保证代码的安全性和可维护性。
 *
 */
fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y.clone() {
        Some(p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => panic!("no match!"),
    }
    y; // Fix without deleting this line.
}
