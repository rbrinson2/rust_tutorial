
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someuser123"),
        email: String::from("someone@email.com"),
        sign_in_count: 1
    };

    user1.email = String::from("anotheremail@email.com");

    let mut user2 = build_user(String::from("user2@email.com"), String::from("user2"));

    let mut user3 = User {
        email: String::from("user3@email.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}

fn build_user (email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1
    }
}