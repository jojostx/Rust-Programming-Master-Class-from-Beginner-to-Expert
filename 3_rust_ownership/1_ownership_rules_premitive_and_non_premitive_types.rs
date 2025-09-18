// -------------------------------------------
// 			Rust Ownership
// 			- Each value in Rust has a variable that’s called its owner.
// 			- There can be only one owner at a time.
// 			- When the owner goes out of scope, the value will be dropped.
// -------------------------------------------

fn main() {
    let x = 32.5;
    let y = x;

    println!("x: {}, y: {}", x, y); // this works because integers are copied and not moved
    // primitives: bools, chars, static strings, integers, floats, fixed-size arrays
    // are copied and not moved because they're stored on the stack and their sizes can be
    // determined at compile time

    let str1 = String::from("abc");
    let str2 = str1;
    let str3 = &str2;

    // fails because str1 no more owns the underlying string borrow of moved value: `str1` value borrowed here after move
    // println!("s1: {}, s2: {}", str1, str2);

    // this one works because we are using a referenced variable, s3
    println!("s2: {}, s3: {}", str2, str3);

    println!("s2: {}", str2);

    // the same applies for vectors
    let vec_1 = vec![1, 2, 3, 4, 5];
    let vec_2 = vec_1;
    let vec_3 = vec_2.clone();

    // println!("vec_1: {}, vec_2: {}", vec_1, vec_2); // this fails because of rust ownership rules
    println!("vec_1: {:?}, vec_2: {:?}", vec_2, vec_3);

    // variables are dropped when they go out of scope.
    {
        let scoped_x = 32;
        println!(
            "this prints the scoped_x from within the scope {}",
            scoped_x
        );
    }

    // trying to print the scoped_x outside the scope fails
    /*
        cannot find value `scoped_x` in this scope
    not found in this scope
     */
    // println!(
    //     "this prints the scoped_x from within the scope {}",
    //     scoped_x
    // );
}
