fn main() {
    // -------------------------------------------
    // 			Loops
    // 			- For loop
    //          	- Looping through elements using an immutable reference
    //          	- Looping through elements using a mutable reference
    // -------------------------------------------

    for (i, val) in [45, 30, 85, 90, 41, 39].iter().enumerate() {
        println!("The {}th value in the vector is {}", i, val);
    }

    for i in &mut [45, 30, 85, 90, 41, 39].iter_mut() {
        // or &mut some_vec
        *i += 5;
        println!("{}", i);
    }
}
