// lifetimes3.rs
//
// Lifetimes are also needed when structs hold references.
//
// Execute `rustlings hint lifetimes3` or use the `hint` watch subcommand for a hint.


struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    /*

    在创建结构体实例时，将字符串类型的变量的引用作为字段值传递给结构体是不合法的，
    因为这些变量的生命周期可能会在结构体实例被使用时结束，导致悬垂引用。
     */
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };

    println!("{} by {}", book.title, book.author);
}
