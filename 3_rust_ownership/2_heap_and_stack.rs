// -------------------------------------------
// 			Heap and Stack
// -------------------------------------------

const MAX_VALUE: i32 = 40_000; // stored on the stack

fn main() {
    let (x, y) = (2, 4);
    let sum_value = square_sum(x, y);
    println!("The value of Square of Sum = {}", sum_value);
}

fn square_sum(num1: i32, num2: i32) -> i32 {
    square(num1 + num2)
}

fn square(num: i32) -> i32 {
    num * num
}
