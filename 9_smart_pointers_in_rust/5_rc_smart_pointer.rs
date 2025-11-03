// ---------------------------
// Reference Counting Pointers
// ---------------------------

use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Nil)))));
    let b = Rc::new(Cons(2, Rc::clone(&a)));
    let c = Rc::new(Cons(3, Rc::clone(&a)));

    print!("values a, b, c: {:?}, {:?}, {:?}", a, b, c);

    make_rc();
}

fn make_rc() -> Rc<String> {
    let s1 = Rc::new(String::new());
    println!("count when pointer is created {}", Rc::strong_count(&s1));

    let s2 = s1.clone();
    println!("count after clone {}", Rc::strong_count(&s1));

    s2
}
