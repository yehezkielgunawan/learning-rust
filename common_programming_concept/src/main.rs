fn main() {
    // VARIABLES
    // let mut x = 5;
    // const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("The value of x is: {x}");
    // x = 6;
    // println!("The value of x is: {x}");

    // SHADOWING
    // let x = 5;
    // let x = x + 1;
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is {x}")
    // }
    // print!("The value of x is: {x}")

    // DATA TYPES
    // let guess: u32 = "42".parse().expect("Not a number!");

    // let x = 2.0;
    // let y: f32 = 3.0;

    // let sum = 5 + 10;
    // let difference = 95.5 - 4.3;
    // let product = 4 * 30;
    // let quotient = 56.7 / 32.2;
    // let truncated = -5 / 3;
    // let remained = 42 % 5;

    // let t = true;
    // let f: bool = false;

    // let c = 'z';
    // let z: char = 'Z';
    // let hearted_eyed_cat = '😻';

    // TUPLE
    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // println!("The value of y is: {y}");
    // println!("The value of x is: {x}");
    // println!("The value of z is: {z}");

    // let x: (i32, f64, u8) = (500, 6.4, 1);
    // let five_hundred = x.0;
    // let six_point_four = x.1;
    // let one = x.2;

    // ARRAY
    // let a = [1, 2, 3, 4, 5];

    // let first = a[0];
    // let second = a[1];

    // println!("Please enter an array index.");

    // let mut index = String::new();

    // io::stdin()
    //     .read_line(&mut index)
    //     .expect("Failed to read line!");

    // let index: usize = index
    //     .trim()
    //     .parse()
    //     .expect("Index entered was not a number");

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");

    // FUNCTIONS
    // println!("Hello, world!");
    // another_function(5, 'h');

    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    // println!("The value of y is: {y}");

    // let x = five();
    // println!("The value of x is: {x}");

    // let x = plus_one(5);
    // println!("The value of x is: {x}");

    // CONTROl FLOW
    // let number = 7;

    // if number < 5 {
    //     println!("condition was true");
    // }

    // let number = 3;
    // if number != 0 {
    //     println!("The number was there");
    // }

    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is not divisible by 4, 3, or 2");
    // }

    // let condition = true;
    // let number = if condition { 5 } else { "six" };

    // println!("The value of the number is: {number}");

    // LOOPING
    // loop {
    //     println!("again")
    // }

    // let mut counter = 0;

    // let result = loop {
    //     counter += 1;

    //     if counter == 10 {
    //         break counter * 2;
    //     }
    // };

    // println!("The result is {result}");

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }
    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");

    // let a = [10, 20, 30, 40, 50];

    // for element in a {
    //     println!("the value is {element}");
    // }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!")
}

// fn five() -> i32 {
//     5
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// fn another_function(value: i32, unit_label: char) {
//     println!("The measurement is: {value} {unit_label}")
// }
