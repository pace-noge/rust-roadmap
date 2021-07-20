#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


pub fn run() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("{}", user1.email);

    // mutable

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someone"),
        active: true,
        sign_in_count: 2,
    };

    user1.email = String::from("new_mail@example.com");

    println!("{:?}", user1);

    let user2 = build_user(String::from("second@example.com"), String::from("2nduser"));
    println!("{:?}", user2);

    // struct update
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };
    println!("{:?}", user3);

    // or
    let user4 = User {
        email: String::from("another@example.com"),
        username: String::from("anotheruser567"),
        ..user1
    };

    println!("{:?}", user4);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}