// -------------------------------------------
// 	        Lifetime Elision
// -------------------------------------------

/*
1. Each parameter that is a reference, gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to
    all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
   the lifetime of self is assigned to all output lifetime parameters.
*/

// Code with Lifetime Elision
fn return_str(s_1: &str) -> &str {
    s_1
}

// Code without Lifetime Elision
fn return_str2<'a>(s_1: &'a str) -> &'a str {
    s_1
}

// Example 2:

fn return_str3<'a, 'b>(s_1: &'a str, s_2: &'b str) -> &'a str {
    s_1
}

fn main() {
    let str_1 = "some str";

    let recieved_str = return_str(&str_1);

    println!("{} has a length of {}", recieved_str, recieved_str.len());

    let recieved_str2 = return_str2(&str_1);

    println!("{} has a length of {}", recieved_str2, recieved_str2.len());

    let str_1 = "some str";
    let str_2 = "other str";
    let recieved_str = return_str3(&str_1, &str_2);

    println!("{} has a length of {}", recieved_str, recieved_str.len());
}
