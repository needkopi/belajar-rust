struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {

    let user = User{
        username: String::from("needkopi"),
        email: String::from("needkopi@gmail.com"),
        sign_in_count: 2,
        active: true,
    }

    /*
     * create instance from another instance
     *
     * */

    let user2 = User{
        username: String::from("samakayakatas"),
        email: String::from("samakayakatas@gmail.com"),
        ..user
    }
}
