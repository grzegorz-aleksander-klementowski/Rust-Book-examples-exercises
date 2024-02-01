struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: usize,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("grzegorz_a.klementowski"),
        email: String::from("grzegorz_a.klementowski@outlook.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("another_gregorio_email@klementowski.pl");
}
