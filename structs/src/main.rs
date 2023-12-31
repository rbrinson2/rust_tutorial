
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

// struct AlwaysEqual;

// ----- Exapmle Program Structs ----- //

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
} 

fn main() {
    // ----- Defining Structs ----- //
    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someone@email.com"),
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@email.com");

    let user2 = build_user(String::from("user2@email.com"), String::from("user2"));
    println!("user2 info{{ active:{}, username: {}, email: {}, sign in count: {} }}", user2.active, user2.username, user2.email, user2.sign_in_count);

    let user3 = User {
        email: String::from("user3@email.com"),
        ..user1
    };
    println!("user3 info{{ active:{}, username: {}, email: {}, sign in count: {} }}", user3.active, user3.username, user3.email, user3.sign_in_count);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black: {}, origin {}", black.0, origin.0);

    //let subject = AlwaysEqual;

    // ----- An Example Program ----- //

    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    println!("The area of the {:?} is {}.", rect1, area(&rect1));

    dbg!(&rect1);

}

fn build_user (email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}

// ----- An Exapmle Program Functions ----- //

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}