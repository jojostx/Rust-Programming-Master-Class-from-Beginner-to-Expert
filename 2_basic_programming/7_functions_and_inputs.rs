fn main() {
    println!("{}", average(Vec::from([1, 2, 3, 4, 5])));
    println!("{}", average(Vec::from([1, 2, 3, 4, 5, 1])));

    // code block used for assignment
    let fullname = { format!("{} {}", "afsar", "nurul") };

    println!("my fullname is {}", fullname);

    // reading user input into String
    let mut my_string = String::new();

    std::io::stdin()
        .read_line(&mut my_string)
        .expect("unable to read from stdin");

    println!("{}", my_string);

    // reading in numbers
    let mut num = String::new();

    std::io::stdin()
        .read_line(&mut num)
        .expect("unable to read from stdin");

    println!("the num as a string {}", num);

    let num: i32 = num.trim().parse().unwrap();

    println!("the num as a number {}", num);
}

fn average(nums: Vec<i32>) -> f32 {
    let sum: i32 = nums.iter().sum();

    sum as f32 / (nums.len() as f32)
}
