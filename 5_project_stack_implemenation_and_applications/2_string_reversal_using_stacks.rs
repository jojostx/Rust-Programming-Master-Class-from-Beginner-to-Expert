fn new_stack(maxsize: usize) -> Vec<char> {
    Vec::with_capacity(maxsize)
}

fn push(stack: &mut Vec<char>, item: char, maxsize: usize) -> Result<char, &str> {
    let stack_len: u32 = stack.len() as u32;
    let maxsize = maxsize as u32;
    if stack_len >= maxsize || stack_len + 1 > maxsize {
        return Err("unable to push item to stack");
    }

    stack.push(item);
    Ok(item)
}

fn pop(stack: &mut Vec<char>) -> Option<char> {
    stack.pop()
}

fn size(stack: &[char]) -> usize {
    stack.len()
}

fn main() {
    let input_string = String::from("Welcome to rust");
    let maxsize = input_string.len();
    let mut stack = new_stack(maxsize);

    let mut rev_string = String::new();

    for c in input_string.chars() {
        let _ = push(&mut stack, c, maxsize);
    }

    for _ in 0..size(&stack) {
        rev_string.push(pop(&mut stack).unwrap());
    }

    println!("{}", input_string);
    println!("{}", rev_string);
}
