 fn main() {
    // let var = 1; // created on the stack
    // let mut s = "hello".to_string(); // created on the heap
    // s.push_str(", world");

    // let x = vec!("Merijn".to_string());
    // let y = x;
    // let z = y;
    // println!("{:?}", z);
    
    // let x = vec!("Merijn".to_string());
    // let y = x.clone();
    // let z = y.clone();
    // println!("{:?}", x);
    // println!("{:?}", y);
    // println!("{:?}", z);

    // let x = 1;
    // let y = x;
    // println!("x = {} and y = {}", x, y);

    // let s = String::from("takes");
    // takes_ownership(s);

    // let value = 1;
    // make_copy(value);

    // let str1: String = give_ownership();
    // println!("{}", str1);

    // let str3: String = take_and_give(str1);

    // if (true) {
    //     let str4 = str3;
    // } else {
    //     let str5 = str3;
    // }
    // println!("{}", str3);
    // let mut str1= 1;
    // let mut str2;
    // loop {
    //     str2 = str1;
    //     println!("{}", str2);
    //     str1 += str2;
    //     if str1 > 10000 {
    //         break
    //     }
    // }

//     let mut s = String::from("Hello");
//     cange_string(&mut s);
//     println!("{}", s);
 }

// fn cange_string(some_string: &mut String) {
//     some_string.push_str(", world");
// }
// fn takes_ownership(s: String) {
//     let string = s;
//     println!("{}", string);
// }

// fn make_copy(one: i32) {
//     let val1 = one;
//     println!("{}", val1);
// }

// fn give_ownership() -> String {
//     "given".to_string()
// }

// fn take_and_give(str2: String) -> String {
//     str2
// }
// var is dropped
// s is also dropped