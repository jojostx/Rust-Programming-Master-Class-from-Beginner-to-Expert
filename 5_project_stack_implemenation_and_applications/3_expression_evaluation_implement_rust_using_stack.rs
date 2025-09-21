use std::collections::HashMap;

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

fn is_operator(ch: char) -> bool {
    for ch_ in ['+', '-', '*', '/', '(', ')'].into_iter() {
        if ch_ == ch {
            return true;
        }
    }

    false
}
fn get_priority(op: &String) -> u8 {
    let priority_op: HashMap<&'static str, u8> =
        HashMap::from([("-", 1), ("+", 2), ("*", 3), ("/", 4), ("^", 5)]);

    let priority = priority_op.get(op.as_str()).unwrap_or(&0);

    *priority
}

fn individual_symbols(expr: String) -> Vec<String> {
    let mut tokenized_ops: Vec<String> = vec![];
    let mut temp: Vec<char> = vec![];

    for ch in expr.chars() {
        if is_operator(ch) {
            if !temp.is_empty() {
                let op = temp.iter().collect::<String>();
                tokenized_ops.push(op.trim().to_string());
                temp.clear();
            }
            tokenized_ops.push(ch.to_string());
        } else {
            temp.push(ch);
        }
    }

    if !temp.is_empty() {
        let op = temp.iter().collect::<String>();
        tokenized_ops.push(op.trim().to_string());
    }

    tokenized_ops
}

fn build_postfix(tokens: Vec<String>) -> Vec<String> {
    /*
    1. if operator <= to current operator in priority in the stack, pop items into the postfix list until otherwise
    2. if character is ( push to stack
    3. if character is ) pop items from stack to postfix list until you encounter (
    4. if character is an operand, push to postfix list
     */
    // - + * / ^
    let stack_size = tokens.len();
    let mut stack = new_stack(stack_size);
    let mut postfix_vec: Vec<String> = Vec::new();

    for ch in tokens {
        match ch.as_str() {
            "+" | "-" | "*" | "/" | "^" => {
                if !stack.is_empty() {
                    if get_priority(&ch) <= get_priority(stack.last().unwrap()) {
                        while get_priority(&ch) <= get_priority(stack.last().unwrap()) {
                            // keep popping from stack and pushing into the postfix
                            postfix_vec.push(pop(&mut stack).unwrap());
                        }

                        // push to the stack
                        push(&mut stack, ch, stack_size);
                    } else {
                        push(&mut stack, ch, stack_size);
                    }
                } else {
                    push(&mut stack, ch, stack_size);
                }
            }
            "(" => {
                push(&mut stack, ch, stack_size);
            }
            ")" => {
                while stack.last().unwrap() != "(" {
                    // keep popping from stack and pushing into the postfix
                    postfix_vec.push(pop(&mut stack).unwrap());
                }
                pop(&mut stack).unwrap();
            }
            _ => postfix_vec.push(ch),
        }
    }

    while !stack.is_empty() {
        postfix_vec.push(pop(&mut stack).unwrap());
    }

    postfix_vec
}

fn main() {
    let input_string = String::from("(2 + 55) - 36");
    let tokens = individual_symbols(input_string);

    println!("{:?}", tokens);

    let postfix = build_postfix(tokens);

    println!("{:?}", postfix);
}
