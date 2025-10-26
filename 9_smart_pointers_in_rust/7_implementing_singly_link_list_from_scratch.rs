// -------------------------------------------
// 		Link List (Part 1)
// -------------------------------------------

#[derive(Debug)]
struct LinkedList {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

impl Node {
    fn new(value: Option<i32>, pointer: Pointer) -> Option<Self> {
        value.map(|value| Self {
            value,
            next: pointer,
        })
    }
}

fn main() {
    let node = Node::new(Some(1), None);

    println!("{:?}", node);
}
