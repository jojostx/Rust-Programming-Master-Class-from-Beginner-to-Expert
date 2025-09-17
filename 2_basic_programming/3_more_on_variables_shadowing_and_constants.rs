fn main() {
    // -------------------------------------------------
    // 			Initializing mutliple variables
    // -------------------------------------------------

    let (first_number, second_number) = (250, 480.32);
    println!(
        "The first number is {} and the second number is {}",
        first_number, second_number
    );

    // -------------------------------------------------
    // 			Readability of large numbers
    // -------------------------------------------------

    let large_number = 1_000_000;
    println!("The value of the large number is {}", large_number);

    // -------------------------------------------------
    // 			Integer Overflow
    // -------------------------------------------------

    //let overflow_number:u8 = 256;

    // -------------------------------------------------
    // 			Decimal numbers in other formats
    // -------------------------------------------------

    let x = 255;
    println!(
        "The value of the variable x in hexademical is {:o} and in octal is {:X} and in binary {:b}",
        x, x, x
    );

    // -------------------------------------------------
    // 			Operations on number in
    // 			different formats
    // -------------------------------------------------

    let n1 = 14;
    let n2 = 15.6;
    let n3 = n2 as i32 + n1;
    println!("The value of n3 = {}", n3);

    // -------------------------------------------------
    // 			Shadowing
    // -------------------------------------------------

    // Case 1: simple shadowing
    println!("\n***************************************************** \n");

    // Case 4: The shadowing in case of code segments
    let s = 65;

    {
        let s = 60; // after this change s to let s = 60 and will change the output to 65
        println!("The value of the variable s inside the inner scope = {}", s);
    }

    println!("The value of the variable s = {}", s);

    // -------------------------------------------------
    // 			Constants
    // -------------------------------------------------

    const MAX_SALARY: u32 = 100_000;
    println!("The value of the constant is {}", MAX_SALARY);
}
