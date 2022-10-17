
enum Shape {
    Triangle,
    Square,
    Pentagon,
    Octagon,
}

impl Shape {
    fn corners(self) -> i32 {
        match self {
            Shape::Triangle => 3,
            Shape::Square => 4,
            Shape::Pentagon => 5,
            Shape::Octagon => 8,
        }
    }
}

fn main() {
    println!("{}", Shape::Triangle.corners());
    println!("{}", Shape::Square.corners());
    println!("{}", Shape::Pentagon.corners());
    println!("{}", Shape::Octagon.corners())
}