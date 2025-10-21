// -------------------------------------------
// 			Iterators
//           	- Basics
//           	- Some useful functions for iterators
//           	- Common statistics
//           	- Modifying and collecting values
// -------------------------------------------

fn main() {
    let a = vec![0, 1, 2, 3, 4, 5, 6, 7];
    let filtered_values = a.iter().filter(|&a| *a > 2);
    let filtered_values = filtered_values.collect::<Vec<&u32>>();
    println!(" {:?}", filtered_values);

    let b = a.clone();
    let filtered_values = a.into_iter().filter(|x| *x >= 5).collect::<Vec<u32>>();
    println!(" {:?}", filtered_values);

    //   println!(" {:?}", a);

    let mapped_values = b.iter().map(|x| *x * 2).collect::<Vec<u32>>();
    println!(" {:?}", mapped_values);

    let mapped_values = b
        .iter()
        .map(|x| *x * 2)
        .filter(|x| *x > 10)
        .collect::<Vec<u32>>();

    /*
        This works too because of Rust’s Deref coercion and operator overloading.

        The Mul (*) operator for integers is implemented such that it works with references to integers as well — i.e. &u32 * u32 is defined.
        Rust automatically dereferences x when performing arithmetic.
    */

    println!("b = {:?}", b);
    println!("mapped_values = {:?}", mapped_values);

    // https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map
}
