trait SetVehicle {
    fn set_mpg(&mut self, _new_mpg: u32) {}
    fn set_color(&mut self, _new_color: String) {}
    fn set_top_speed(&mut self, _new_top_speed: u32) {}
}

struct Car {
    mpg: u32,
    color: String,
    top_speed: u32
}

impl SetVehicle for Car {
    fn set_mpg(&mut self, new_mpg: u32) {
        self.mpg = new_mpg;
    }
    fn set_color(&mut self, new_color: String) {
        self.color = new_color;
    }
    fn set_top_speed(&mut self, new_top_speed: u32) {
        self.top_speed = new_top_speed;
    }
}

struct Motorcycle {
    mpg: u32,
    color: String,
    top_speed: u32,
}

impl SetVehicle for Motorcycle {
    fn set_mpg(&mut self, new_mpg: u32) {
        self.mpg = new_mpg;
    }
    fn set_color(&mut self, new_color: String) {
        self.color = new_color;
    }
    fn set_top_speed(&mut self, new_top_speed: u32) {
        self.top_speed = new_top_speed;
    }
}

fn main() {
    let mut car = Car{mpg: 0, color: "null".to_owned(), top_speed: 0};
    println!("The base mpg is {}, the base color is {} and the base top speed is {}", car.mpg, car.color, car.top_speed);
    car.set_mpg(80);
    car.set_color("Red".to_owned());
    car.set_top_speed(150);
    println!("The mpg is {}, the color is {} and the top speed is {}", car.mpg, car.color, car.top_speed);
    println!();

    let mut motorcycle = Motorcycle{mpg: 0, color: "null".to_owned(), top_speed: 0};
    println!("The base mpg is {}, the base color is {} and the base top speed is {}", motorcycle.mpg, motorcycle.color, motorcycle.top_speed);
    motorcycle.set_mpg(100);
    motorcycle.set_color("Green".to_owned());
    motorcycle.set_top_speed(300);
    println!("The mpg is {}, the color is {} and the top speed is {}", motorcycle.mpg, motorcycle.color, motorcycle.top_speed)
}