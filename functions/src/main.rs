fn main() {
    another_function();
    let res = return_demo();
    println!("The value of res is :{}", res);

    let res2 = return_demo2(10);
    println!("The value of res2 is :{}", res2);
}

fn another_function() {
    // let x = (let y = 6);

    // let y = 6;
    // let x = y;

    // println!("The value of x is: {}", x);

    let x = 5;
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn return_demo() -> i32 {
    5
}

// return function
fn return_demo2(x: i32) -> i32 {
    x + 1 //;
}
