fn main() {
    let s = String::from("hello world");
    println!("The length of {} is {}", s, calculate_len(&s));

    // change(&s);
    let mut s1 = String::from("hello world");

    // let r1 = &mut s1;
    // let r2 = &mut s1;
    //
    // println!(" {} -- {}",r1, r2);
    // {
    //     let r1 = &mut s1;
    // }
    // let r2 = &mut  s1;
    // println!("{} -- {}", r1, r2);

    let ref_to_nothing = dangle();
}

fn dangle()->&String {
    let s = String::from("hello world");
    &s
}

fn calculate_len(s: &String)->(usize) {
    let len = s.len();
    len
}

// fn change(s: &String) {
//     s.push_str(" ! Hahaha");
// }
