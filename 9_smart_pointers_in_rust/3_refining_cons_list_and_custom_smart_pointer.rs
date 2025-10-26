// -------------------------------------------
// 			Smart Pointers
//          	- Box Pointers
//          	- Issue with Box Pointer implementation of Cons variant in List
// 			- Custom Defined Smart Pointers
// -------------------------------------------

use std::ops::Deref;

#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

struct MySmartPointer {
    value: i32,
}

impl MySmartPointer {
    // fn new(value: i32) -> MySmartPointer {
    //     MySmartPointer { value }
    // }

    fn new(value: i32) -> Self {
        Self { value }
    }
}

impl Deref for MySmartPointer {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!(
            "This function runs when the variable being dropped (manually or when it goes out of scope"
        );
    }
}

fn main() {
    let list = List::Cons(1, Some(Box::new(List::Cons(2, None))));
    println!("{:?}", list);

    let List::Cons(x, some) = list;

    println!("the contained number is {}", x);
    println!("the contained child list is {:?}", some);

    let v = 50;
    let mysmtpr = MySmartPointer::new(v);

    // dereffing the pointer
    let v2 = *mysmtpr;

    println!("this is the dereffed value {}", v2);
}
