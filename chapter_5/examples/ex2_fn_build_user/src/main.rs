struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: usize,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    build_user(String::from("user1"), String::from("useremail@domain.pl"));

    let user1 = User {
        active: true,
        username: String::from("use2name"),
        email: String::from("user2email@domena.pl"),
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("user2@email.com"),
        ..user1
    };
}
