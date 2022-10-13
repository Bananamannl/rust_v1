fn main() {
    let mut val1 = vec![1,3,5,7];
    println!("val1 = 1: {}", check_val(&val1));
    val1.push(15);
    println!("{:?}", val1);
}

fn check_val(val: &Vec<i32>) -> bool{
     val[0] == 1 
}