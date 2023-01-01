use std::io;

fn main (){
    println!("A basic calculator program, you can only use whole numbers and it will only display whole numbers");

    println!("Input number 1.");

    let mut num_1 = String::new();

    io::stdin()
        .read_line(&mut num_1)
        .expect("failed to read line");
    
    println!("Your first number is: {num_1}");

    println!("Input number 2.");

    let mut num_2 = String::new();

    io::stdin()
        .read_line(&mut num_2)
        .expect("failed to read line");

    println!("Your second number is: {num_2}");

    println!("Put in operation.");

    let mut operation = String::new();

    io::stdin()
        .read_line(&mut operation)
        .expect("failed to read line");

    println!("Your operation is: {operation}");

    let x: i32 = num_1.trim().parse().unwrap();

    let y: i32 = num_2.trim().parse().unwrap();

    let answer = match operation.trim() {
        "+" => x + y,
        "-" => x - y,
        "*" => x * y,
        "/" => x / y,
        "^" => power(x, y),
        _ => {
            eprintln!("Wrong operation.");
            std::process::exit(1)
        }
    };

    println!("The answer is: {answer}");
}

fn power (x: i32, mut y: i32) -> i32 {
    let mut tally = x;
    if y == 0 {
        tally = 1
    }
    while y > 1 {
        tally = tally * x;
        y = y - 1;
    } tally
}

