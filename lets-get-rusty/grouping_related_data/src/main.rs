#[derive(Debug)]
struct User {
    username: String,
    email: String,
    login_count: u32,
    is_online: bool,
}

impl User {
    // methods get passed "&self"
    fn logout(&mut self) {
        self.is_online = false;
    }

    // associated functions do not get passed "&self"
    fn print_status(user: &User) {
        println!(
            "{} is {} online",
            user.username,
            if user.is_online { "well and" } else { "not" }
        )
    }
}

fn main() {
    // create struct item/instance
    let mut user1 = User {
        username: String::from("jack"),
        email: String::from("jack@school.com"),
        login_count: 15,
        is_online: true,
    };

    // copy a compound value
    let username = user1.username.clone();

    // modify mutable struct attributes
    user1.is_online = false;

    let user2 = create_user(&"king", &"king@themail.com");
    let user3 = User { ..user2 };

    // tupe structs
    struct Color(u8, u8, u8);

    // printing structs: must inherit the derive debug trait
    println!("{:#?}", user3);

    // this will not work since "user2" is not a mutable owner or borrower
    // user2.logout()
    user1.logout();
    User::print_status(&user1);
}

fn create_user(username: &str, email: &str) -> User {
    let username = String::from(username);
    let email = String::from(email);

    User {
        username, // field init shorthand syntax
        email,

        login_count: 1,
        is_online: true,
    }
}
