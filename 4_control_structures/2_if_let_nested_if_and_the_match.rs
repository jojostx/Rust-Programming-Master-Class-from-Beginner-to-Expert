use std::io::stdin;

fn main() {
    // -------------------------------------------
    // 			Condition If
    // 			- Nested If
    // 			- If let
    // 			- If let (in case of if else ladder)
    // 			- Match expression
    // ------------------------------------------

    // Example
    println!("enter a number: ");
    let mut buf = String::new();
    stdin()
        .read_line(&mut buf)
        .expect("unable to read your input. try again");
    let num = buf
        .trim()
        .parse::<i32>()
        .expect("an error occured when reading your number input");

    if num < 1996 {
        println!("you are a millennial");
    } else if num < 2009 {
        println!("you are a gen-z");
    } else {
        println!("baba you don old o");
    }

    // 2. if let
    /*
    let variable_name = if condition {
        // Statements to execute and
        // return value of variable without a semicolon
    } else {
        // Statements to execute and
        // value of variable of variable without a semicolon
    };  // semicolon
    */

    /*
    let value = if true {
        1
    } else {
        2   // 2.0 will generate an error
            // please note that the returns from all the statements should be of the same type
            // The else part must exist otherwise the variable may be empty which the rust wont allow
    };
    println!("Value = {}", value);
    */

    let marks = 80;

    let grade = if marks >= 90 {
        "A+"
    } else if marks > 80 {
        "A"
    } else if marks > 70 {
        "B"
    } else if marks > 60 {
        "C"
    } else if marks > 50 {
        "D"
    } else {
        "F"
    };

    println!("The obtained grade is {:?}", grade);
}
