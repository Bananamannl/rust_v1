fn main() {
    let val1 = 5;
    let val2 = 2;
    let ans = val1 % val2;
    println!("The answer of {} Mod {} = {}", val1, val2, ans);
    println!();

    let mut vec = vec!(2, 4, 5, 6, 8, 10);
    println!("{:?}", vec);
    vec.pop();
    vec.push(12);
    println!("{:?}", vec);
    println!();

    concat_string("Hello".to_string())
}

fn concat_string(mut string1: String) {
    string1.insert_str(5, " world");
    println!("{}", string1);
}