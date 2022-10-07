fn main() {
    // let var = 1; // created on the stack
    // let mut s = "hello".to_string(); // created on the heap
    // s.push_str(", world");

    let x = vec!("Merijn".to_string());
    let y = x;
    let z = y;
    println!("{:?}", z);
}

// var is dropped
// s is also dropped