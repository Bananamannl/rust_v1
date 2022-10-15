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