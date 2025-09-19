// -------------------------------------------
//	  Dereferencing
// -------------------------------------------
fn main() {
    let mut some_data = 42;
    let ref_1 = &mut some_data;
    let deref_copy = *ref_1;
    *ref_1 = 13;
    println!("some_data is: {some_data}, deref_copy is: {deref_copy}");

    let mut heap_data = vec![5, 6, 7];
    let ref_1 = &mut heap_data;
    // let ref_2 = *ref_1; // problenatic code
    let ref_2 = ref_1.clone();
    println!("ref_2: {:?}", ref_2);

    let move_out = ref_1; // a move occurs here
    // let move_out_again = ref_1; // so another move fails
    println!("move_out: {:?}", move_out);
}
