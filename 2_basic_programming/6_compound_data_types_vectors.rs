fn main() {
    // vectors are growable arrays

    let mut nums_vec = vec![1, 2, 3, 4, 3, 5];

    println!("{}", nums_vec[0]);
    println!("{:?}", nums_vec);

    nums_vec[0] = 100;
    println!("{}", nums_vec[0]);

    // prefilled vector
    let mut nums_vec = vec![0; 10];
    println!("{:?}", nums_vec);

    // updating adding an element to the end
    nums_vec.push(14);
    println!("{:?}", nums_vec);

    // get the len of the array
    println!("{}", nums_vec.len());

    // get an elem using the get method
    println!("{:?}", nums_vec.get(2));

    // removing an element
    println!("{:?}", nums_vec.remove(2));

    // use the contains method
    println!("{:?}", nums_vec.contains(&2));
}
