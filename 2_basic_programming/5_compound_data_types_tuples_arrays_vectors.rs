fn main() {
    let my_tuple = ("tuple", 15);
    println!("I retrieved the first item in the tuple: {}", my_tuple.0);
    println!("I retrieved the second item in the tuple: {}", my_tuple.1);

    // destructing assignment for tuples
    let (my_type, my_number) = my_tuple;
    println!("my_type is the first destructured element: {}", my_type);
    println!(
        "my_number is the second destructured element: {}",
        my_number
    );

    // nested tuple
    let my_nested_tuple = (1, "second", ("inner_tuple", 2), "last");
    let inner_tuple = my_nested_tuple.2;
    let inner_tuple_string = inner_tuple.0;

    println!(
        "{:?}{:?}{}",
        my_nested_tuple, inner_tuple, inner_tuple_string
    );

    let empty_tuple = ();

    println!("empty tuple: {:?}", empty_tuple);

    let mut num_array = [1, 2, 3, 45];

    println!("{}", num_array[0]);
    println!("{:?}", num_array);

    // modify a value in the array
    num_array[3] = 34;
    println!("{}", num_array[3]);

    // initialize an array with default elements
    let array_w_def_elems = [0; 10];

    println!("{:?}", array_w_def_elems);

    let string_arr = ["apple", "mango", "avocado", "guava"];
    let fruits = ["no fruit"; 6];

    println!("{:?}", string_arr);
    println!("{:?}", fruits);

    // getting a slice of an array
    let sub_fruits = &fruits[0..=1];

    println!("{:?}", sub_fruits);

    // get length of array
    println!("length of the array: {}", sub_fruits.len());

    // get the amount of memory it occupies
    println!(
        "this array occupies {} bytes",
        std::mem::size_of_val(sub_fruits)
    );

    // use the get method to retrieve values
    println!("testing out the get function {:?}", sub_fruits.get(4));
    println!("testing out the get function {:?}", num_array.get(1));
}
