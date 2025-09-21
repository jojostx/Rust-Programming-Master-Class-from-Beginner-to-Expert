fn main() {
    // -------------------------------------------
    // 			Break: for stopping a loop
    // 			Continue: for skipping the current iteration
    // -------------------------------------------

    let mut var = 100;
    loop {
        var -= 1;
        if var % 13 == 0 {
            break;
        }
    }
    println!(
        "The highest number lesser than the given number divisible by 13 is {}",
        var
    );

    let mut var = 0;
    let mut count = 0;
    let req_number: i32 = loop {
        var += 1;
        if var % 5 == 0 && var % 3 == 0 {
            println!(
                "\nThe number which is divisible by by both 3 and 5 = {} \n",
                var
            );
            count += 1;

            if count == 3 {
                break var;
            } else {
                continue;
            }
        }
        print!("{} ", var);
    };

    println!(
        "the required third highest number divisible by both 3 and 5 is {}",
        req_number
    );
}
