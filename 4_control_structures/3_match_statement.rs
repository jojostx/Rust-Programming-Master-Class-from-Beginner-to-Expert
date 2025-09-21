fn main() {
    // -------------------------------------------
    // 			Match
    // 			- Simple match
    //			- If else ladder into a match
    // 			- If let syntax style match
    // -------------------------------------------
    /* General syntax

       match value {

           possible_value(s) => {Statements to execute},
           possible_value(s) => {Statements to execute},
           possible_value(s) => {Statements to execute},

           _ = { default_execution_statements },
       };
    */

    let some_number = 100;
    match some_number {
        1 | 2 => println!("this number either a 1 or 2: {}", some_number),
        3 | 4 => println!("this number either a 3 or 4: {}", some_number),
        5 | 6 => println!("this number either a 5 or 6: {}", some_number),
        7..=100 => println!("this number between 7 and 100: {}", some_number),
        _ => println!("this number is greater than 100"),
    }

    let marks = 50;
    let mut grade = 'N';

    match marks {
        90..=100 => grade = 'A',
        80..=89 => grade = 'B',
        70..=79 => grade = 'C',
        60..=69 => grade = 'D',
        _ => grade = 'F',
    }

    println!("The grade achieved is {}", grade);

    /*
        let variable = match value {
         possible_value(s) = {Statements to execute},
         possible_value(s) = {Statements to execute},
         possible_value(s) = {Statements to execute},

         _ = { default_execution_statements }
     };
    */

    let marks = Some(98);
    let grade = match marks {
        Some(num) => match num {
            90..=100 => "A+",
            80..=89 => "A",
            70..=79 => "B",
            60..=69 => "C",
            50..=59 => "D",
            _ => "F",
        },
        None => "F",
    };
    println!("The grade achieved is {}", grade);
}
