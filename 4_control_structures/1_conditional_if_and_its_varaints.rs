fn main() {
    // -------------------------------------------
    // 			Condition If
    // 			- Simple If
    // 			- If with multiple conditions
    // 			- If else
    // 			- If else if ladder
    // -------------------------------------------

    // 1. Simple if statement
    if true {
        // statements to execute if condition proves true
    }

    // Example
    let some_number = 60;
    if some_number < 50 {
        println!("The number is less than 40");
    } else {
        println!("The number is greater than 40");
    }
    println!("This line will execute irrespective of the condition above");

    let marks = 65;
    if (60..=70).contains(&marks) {
        println!("The grade is satisfactory");
    }

    let marks = 95;
    let mut grade = 'N';
    if marks >= 90 {
        grade = 'A';
    } else if marks >= 80 {
        grade = 'B';
    } else if marks >= 70 {
        grade = 'C';
    } else if marks >= 60 {
        grade = 'D';
    } else {
        grade = 'F';
    }

    println!("The obtained grade is {}", grade);
}
