
enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}

impl Shape {
    fn corners(self) -> Result<i32, String> {
        match self {
            Shape::Triangle => Ok(3),
            Shape::Square => Ok(4),
            Shape::Pentagon => Err("This is clasified by the united states government".to_owned()),
            Shape::Octagon => Ok(8),
        }
    }
}

fn main() {
    let x = Shape::Pentagon.corners();
    match x {
        Ok(i) => println!("The shape has this many corners: {}", i),
        Err(i) => println!("The pentagon has this many corners: {}", i),
    }
}