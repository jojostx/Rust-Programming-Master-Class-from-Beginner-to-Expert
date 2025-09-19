fn main() {
    // -------------------------------------------
    // 		References Rules
    //          - One mutable reference in a scope
    //          - Many immutable references
    //          - Mutable and immutable can not coexist
    //          - Scope of a reference
    //          - Data should not change when immutable references are in scope
    // -------------------------------------------

    // rule 1
    let mut heap_num = vec![4, 5, 6];
    let ref1 = &mut heap_num;
    // let ref2 = &mut heap_num;

    println!("The first reference is {:?}", ref1);

    // rule 2
    let heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!(
        "The first reference is {:?} and the second one is {:?}",
        ref1, ref2
    );

    // rule 3
    let heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    // let ref3 = &mut heap_num;
    println!("Immutable references are {:?} and {:?}", ref1, ref2);

    let heap_num = vec![4, 5, 6];
    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("Immutable references are {:?} and {:?}", ref1, ref2);

    let mut heap_num = vec![4, 5, 6];
    heap_num.push(68);

    let ref1 = &heap_num;
    let ref2 = &heap_num;
    println!("Immutable references are {:?} and {:?}", ref1, ref2);
    // Notice that you can safely mutate heap_num after the ref1 and ref2 are confirmed to be out of scope
    heap_num.push(86);
}
