fn new_stack(maxsize: usize) -> Vec<String> {
    Vec::with_capacity(maxsize)
}

fn push(stack: &mut Vec<String>, item: String, maxsize: usize) -> Result<String, &str> {
    let stack_len: u32 = stack.len() as u32;
    let maxsize = maxsize as u32;
    if stack_len >= maxsize || stack_len + 1 > maxsize {
        return Err("unable to push item to stack");
    }

    let str_clone = item.clone();
    stack.push(item);

    Ok(str_clone)
}

fn pop(stack: &mut Vec<String>) -> Option<String> {
    stack.pop()
}

fn size(stack: &[String]) -> usize {
    stack.len()
}

fn individual_symbols(expr: String) -> Vec<String> {
    let mut tokenized_ops: Vec<String> = vec![];
    let mut temp: Vec<char> = vec![];

    for ch in expr.chars() {
        match ch {
            '+' | '-' | '*' | '/' | '(' | ')' => {
                if !temp.is_empty() {
                    let op = temp.iter().collect::<String>();
                    tokenized_ops.push(op.trim().to_string());
                    temp.clear();
                }
                tokenized_ops.push(ch.to_string());
            }
            _ => {
                temp.push(ch);
            }
        }
    }

    if !temp.is_empty() {
        let op = temp.iter().collect::<String>();
        tokenized_ops.push(op.trim().to_string());
    }

    tokenized_ops
}

fn main() {
    let input_string = String::from("(2 + 55) - 36");
    let tokenized_expr = individual_symbols(input_string);

    // let maxsize = input_string.len();
    // let mut stack = new_stack(maxsize);

    println!("{:?}", tokenized_expr);
}
