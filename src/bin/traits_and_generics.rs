// struct Point<T, U> {
//     x: T, //i32
//     y: U, //&str
// }

trait Overview {
    fn overview(&self) -> String {
        String::from("This is a rust course")
    }
}

struct Course {
    headline: String,
    author: String,
}

impl Drop for Course {
    fn drop(&mut self) {
        
    }
}

struct AnothereCourse {
    headline: String,
    author: String,
}

impl Overview for Course {
    fn overview(&self) -> String {
       format!("{}, {}", self.author, self.headline) 
    }
}

impl Overview for AnothereCourse {}

fn main() {
    // let coord = Point {x: 5.0, y: 5.0};
    // let coord2 = Point {x: "five", y: 5.0};
    
    let course1 = Course{headline: String::from("Headline!"), author: String::from("Merijn!")};
    let course2 = AnothereCourse{headline: String::from("another Headline!"), author: String::from("another Merijn!")};

    // println!("{}", course1.overview());
    // println!("{}", course2.overview());

    // call_overview(&course1);
    // call_overview(&course2);
}

fn call_overview<T: Overview>(item: &T) {
    println!("Overview: {}", item.overview());
}

//diffirent traitbounds:
// fn overview(item1: &impl Overview, item2: &impl Overview)
// fn overview<T: Overview>(item1: &T, item2: &T)
// fn overview(item1: &impl Overview + AnotherTrait)
// fn overview<T: Overview + AnotherTrait>(item1: &T, item2: &T)