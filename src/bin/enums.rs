enum Pet {Dog, Cat, Fish}

impl Pet {
    fn what_am_i(self) -> &'static str {
        match self {
            Pet::Dog => "I am a dog",
            Pet::Cat => "I am a cat",
            Pet::Fish => "I am a fish",
        }
    }
}

enum IpAddrKind{
    V4(String),
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main () {
    let dog = Pet::Dog;
    println!("{}", dog.what_am_i());

    let home = IpAddrKind::V4(String::from("127.0.0.1"));

    let loopack = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    let some_number = Some(5);
    let some_string = Some("A string");
    let nothing: Option<i32> = None; //Option<T>, let x = 5 == i32


    let x: i32 = 5;
    let y: Option<i32> = Some(5);

    //let sum = x + y; //is not possible

    let five = Some(0.0);
    let six = devide_1_by_x(five);
    let none = devide_1_by_x(None);

    println!("{:?}", five);
    println!("{:?}", six);
    println!("{:?}", none);

    what_pet("Dog");
    what_pet("Cat");
    what_pet("Fish");
    what_pet("Cow");

    let dog2 = Some(Pet::Dog);
    if let Some(Pet::Dog) = dog2 {
        println!("Your pet is a dog!")
    } else {
        println!("You pet is not a dog!")
    }

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top)
    }

    // let x = 1;
    // match x {
    //     1 | 2 => println!("one or two"),
    //     _ => println!("not one or two"),
    // }

    let x = 5;
    match x {
        1..=5 => println!("matches"),
        _ => println!("not matching")
    }

    let x = Some(5);
    let y = 5;
    match x {
        Some(10) => println!("10"),
        Some(x) if x == y => println!("matches!"),
        _ => println!("default")
    }
    
}

fn devide_1_by_x(x: Option<f64>) -> Option<Result<f64, &'static str>>{
    match x {
        Some(i) => {
            Some(if i != 0.0 {
                Ok(1.0 / i)
            } else {
                Err("Cannot divide by 0")
            })
        },

        None => None,
    }
}

fn what_pet(input: &str) {
    match input {
        "Dog" => println!("I have a dog"),
        "Fish" => println!("I have a fish"),
        "Cat" => println!("I have a cat"),
        _ => println!("I have no clue what pet you have"),
    }
}