fn main() {
    // let mut s1 = String::from("hello");
    // s1.push_str(" world");
    // println!("s1 is: {}", s1);
    // // assign s1 to s2
    // // Rust considers s1 to no longer be valid. It know as "move"
    // // let s2 = s1;
    // // println!("s1 is: {}", s1);
    //
    // // deep copy
    // let s2 = s1.clone();
    // println!("s1 is {}, s2 is: {}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);

    // println!("{}", s);

    let x = 5;
    makes_copy(x);

    println!("{}", x);

    let s1 = String::from("hello");

    let (s2, len) = calculate_len(s1);

    println!("The length of '{}' is {}", s2, len);
}

fn calculate_len(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
