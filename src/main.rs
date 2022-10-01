fn main() {
    //let x: i8 = 10;
    //println!("{}", x);

    // let _y: u8 = 10;

    //let decimial = 02_55;
    //let hex = 0xff;
    //let octal = 0o377;
    //let binary = 0b1111_1111;

    //println!("{}", decimial);
    //println!("{}", hex);
    //println!("{}", octal);
    //println!("{}", binary);

    //let byte = b'A';
    //println!("{}", byte);

    //let x = 2.0; //f64 default because on modern CPUs is roughly the same speed as f32
    //let y: f32 = 1.0;
    //let t = true;
    //let f: bool = false;

    //let c = 'c';

    //println!("{}", c);

    // +, -, *, / and %

    //let a = 10;
    //let b = 4;

    //let remainder = a % b;
    //println!("{}", remainder);

   
    //let Tup = (500, "hi", true);
    //println!("{}", Tup.2);

    //let (x, y, z) = Tup;
    //println!("{}", x);
    //println!("{}", y);
    //println!("{}", z);

    //let array = [1, 2, 3];
    //println!("{}", array[0]);

    //let mut array2: [i32; 3] = [4,5,6];
    //println!("{}", array2[0]);
    //array2[0] = 10;
    //println!("{}", array2[0]);

    let mut nums = vec![1,2,3];

    nums.push(4);
    println!("{:?}", nums);
    
    nums.pop();
    println!("{:?}", nums);

    let mut vec = Vec::new();
    vec.push("Help");
    vec.push("me");
    vec.push("now");
    println!("{:?}", vec);
    vec.pop();
    println!("{:?}", vec);

    vec.reverse();
    println!("{:?}", vec);
}
