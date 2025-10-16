use std::{io::stdin, process::exit};

fn new_stack(maxsize: usize) -> Vec<u32> {
    Vec::with_capacity(maxsize)
}

fn push(stack: &mut Vec<u32>, item: u32, maxsize: usize) -> Result<u32, &str> {
    let stack_len: u32 = stack.len() as u32;
    let maxsize = maxsize as u32;
    if stack_len >= maxsize || stack_len + 1 > maxsize {
        return Err("unable to push item to stack");
    }

    stack.push(item);
    Ok(item)
}

fn pop(stack: &mut Vec<u32>) -> Option<u32> {
    stack.pop()
}

fn size(stack: &[u32]) -> usize {
    stack.len()
}

fn input() -> u32 {
    let mut num = String::new();

    stdin().read_line(&mut num).expect("unable to read input");

    num.trim().parse::<u32>().expect("unable to read input")
}
fn main() {
    println!("Let's first create a stack for our use");
    println!("what is the size of our stack?");

    let maxsize = input() as usize;
    let mut stack = new_stack(maxsize);

    loop {
        println!("\n -- Menu -- \n");
        println!(" 1. Push \n 2. Pop \n 3. Display Stack \n 4. size \n 5. Exit");
        let choice = input();

        match choice {
            1 => {
                println!("please specify the number to add to the stack");
                let item = input();

                match push(&mut stack, item, maxsize) {
                    Ok(n) => print!("item {} has been added to the stack", n),
                    Err(msg) => println!("{}", msg),
                }
            }
            2 => {
                let n = pop(&mut stack);

                match n {
                    Some(n) => println!(
                        "{} was removed from the stack, and this is the new stack => {:?}",
                        n, stack
                    ),
                    None => println!(
                        "no item was removed from the stack, and this is the new stack => {:?}",
                        stack
                    ),
                }
            }
            3 => println!("{:?}", stack),
            4 => println!("This is the size of the stack now => {}", size(&stack)),
            5 => exit(0),
            _ => println!("incorrect option selected, please choose between 1 to 5"),
        }
    }
}
