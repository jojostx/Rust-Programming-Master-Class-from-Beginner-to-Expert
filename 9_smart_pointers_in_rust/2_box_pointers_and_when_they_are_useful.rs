// -------------------------------------------
// 			Smart Pointers
//          	- Box Pointers
//          	- Use Case of Box Pointers
// -------------------------------------------

use List::{Cons, Nil};

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("{:?}", list);

    let (x, b) = match list {
        Cons(x, b) => (x, b),
        Nil => (0, Box::new(Nil)),
    };

    println!("{:?}", x);
    println!("{:?}", b);
}
