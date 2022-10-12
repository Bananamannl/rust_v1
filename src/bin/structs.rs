// struct User {
//     active: bool, 
//     username: String,
//     sign_in_count: u32,
// }

// struct Coordinates(i32,i32,i32);

// struct Square {
//     width: u32,
//     hight: u32,
// }

// impl Square {
//     fn area(&self) -> u32 {
//         self.width * self.hight
//     }

//     fn whats_my_width(&self) -> u32 {
//         self.width
//     }

//     fn change_width(&mut self, new_width: u32) {
//         self.width = new_width
//     }
// }

// struct MyString<'a> {
//     text: &'a str
// }

fn main() {


    // let str1 = String::from("This is my string");
    // let x = MyString{text: str1.as_str()};
    // let s: &'static str = "I have a static life time!";
    // let user1 = User{active: true, username: String::from("Merijn"), sign_in_count: 0};
    // println!("{}", user1.username);

    // let user2 = build_user(String::from("Merijn2"));
    // println!("{}", user2.username);

    // let cords = Coordinates(1,2,3);
    // 1..5, .. is a Range {start 1, end: 5}

    // let mut sq = Square {width: 5, hight: 5};
    // println!("the area of the square = {}", sq.area());
    // println!("the width of the square = {}", sq.whats_my_width());
    // sq.change_width(7);
    // println!("the new width = {}", sq.whats_my_width());

    // let r;

    // {
    //     let x = 5;
    //     r = x;
    // }
    // println!("{}", r); //'a

    // &i32
    // &'a i32
    // &'a mut i32
    
}

// fn example<'a, 'b>(x: &'a mut str, y: &'b mut str) -> &'a str {
//     x
// }


// fn build_user(username: String) -> User {
//     User {
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }