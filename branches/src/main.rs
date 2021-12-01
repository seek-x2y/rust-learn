fn main() {
    let number = 3;

    // if number < 5 {
    //     println!("condition was true!");
    // }else{
    //     println!("condition was false!");
    // }

    if number != 0 {
        println!("number was something other than zero");
    }


    let condition = true;

    let number = if condition {
        5
    }else{
       6
    //    "mix"
    };

    println!("The value of number is : {}", number);

    // loop
    // loop{
    //     println!("again!");
    // }
    let mut counter = 0;

    let res =  loop{
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(res, 20);

    // while 
    let mut num = 3;

    while num != 0 {
        println!("{}! ", num);
        num -= 1;
    }

    println!("LIFTOFF!!!!");

    let a = [10, 20, 30, 40, 50];

    // let mut index = 0;
    // while index < 6 {
    //     println!("1. The value is: {}", a[index]);
    //     index += 1;
    // }

    // for 
    for elem in a.iter() {
        println!("2. The value is: {}", elem);
    }

    for e in (1..4).rev() {
        println!("The value is: {}", e);   
    }
    println!("LIFTOFF!!!!");
}
