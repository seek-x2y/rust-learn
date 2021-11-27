fn main() {
    //  help: if this is intentional, prefix it with an underscore: `_y`, 未使用的变量的命名必须采用类似其他语言中私有变量的形式？
    // let x = 2.0;
    // let y: f32 = 3.0;
    let _test = 't';

    let sum = 5 + 3;
    println!("sum is: {}", sum);
    let difference = 95.5 - 4.3;
    println!("difference is: {}", difference);
    let product = 4 * 30;
    println!("product is: {}", product);
    let quotient = 56.7 / 32.2;
    println!("quotient is: {}", quotient);

    let tup: (i32, f64, u8, bool) = (500, 6.4, 1, true);
    println!("tup0 is: {}", tup.0);
    println!("tup1 is: {}", tup.1);
    println!("tup2 is: {}", tup.2);
    println!("tup3 is: {}", tup.3);

    let (x, y, z, a) = tup;
    println!("x is: {}", x);
    println!("y is: {}", y);
    println!("z is: {}", z);
    println!("a is: {}", a);

    //  help: convert the identifier to snake case: `bool_array` rush 强制变量命名采用snake case ，不允许驼峰？
    let bool_array = [true, false];
    println!("bool arrray 0 is :{}", bool_array[0]);

    let array_test = [1,2,3,4,5];

    let index = 10;
    let ele = array_test[index];

    println!("The value of ele is: {}", ele);
}
