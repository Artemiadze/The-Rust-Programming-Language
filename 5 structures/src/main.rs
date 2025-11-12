// Struct
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //Return himself (example: sq = Rectangle::square(3);)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

//Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    print_users();

   let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_users(){
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("User1 {}, {}, {}, {}", user1.email, user1.username, user1.active, user1.sign_in_count);

    let mut user2 = User {
        email: String::from("somewho@example.com"),
        username: String::from("someusername456"),
        active: true,
        sign_in_count: 44,
    };

    println!("User2 {}, {}, {}, {}", user2.email, user2.username, user2.active, user2.sign_in_count);
    user2.email = String::from("anotheremail@example.com");
    println!("Updated User2 {}, {}, {}, {}", user2.email, user2.username, user2.active, user2.sign_in_count);

    // creating user 3 using field from user1
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
    println!("User3 {}, {}, {}, {}", user3.email, user3.username, user3.active, user3.sign_in_count);

    let user4 = build_user(String::from("anotherusername789"), String::from("another@example.com"));
    println!("User4 {}, {}, {}, {}", user4.email, user4.username, user4.active, user4.sign_in_count);

    //Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("Color: {}, {}, {}", black.0, black.1, black.2);
    println!("Point: {}, {}, {}", origin.0, origin.1, origin.2);
}