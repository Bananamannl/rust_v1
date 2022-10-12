
struct Car {
    mpg: u32,
    color: String,
    top_speed: u32,
}

impl Car {
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
    let mut car = Car{mpg: 0, color: "null".to_string(), top_speed: 0};
    println!("The basic car: is mpg = {}, color = {}, top speed = {}", car.mpg, car.color, car.top_speed);
    car.set_mpg(7);
    car.set_color(String::from("Blue"));
    car.set_top_speed(60);
    println!("The new car is: mpg = {}, color = {}, top speed = {}", car.mpg, car.color, car.top_speed)
}