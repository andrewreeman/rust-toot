struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

/*
struct InvalidUserIfDeclaredWithoutLifetime {
    active: bool,
    username: &str,
    email: &str,
    sign_in_count: u64,
}
*/

fn main() {
    let user1 = User {
        active: true,
        username: String::from("myusername"),
        email: String::from("iamuser@userville.com"),
        sign_in_count: 666,
    };

    println!(
        "User is active {}, name is {}, email is {} and sign in count is {}",
        user1.active, user1.username, user1.email, user1.sign_in_count
    );

    let mut user2 = User {
        active: true,
        username: String::from("Admin"),
        email: String::from("admin@bossland.com"),
        sign_in_count: 42,
    };

    user2.sign_in_count = 45;

    println!("User 2 sign in count {}", user2.sign_in_count);

    let user3 = build_user(String::from("luser3@luserland.com"), String::from("Luser"));
    println!("User 3 is active: {}", user3.active);

    let user2 = User {
        active: false,
        ..user1
    };

    println!("Updated user 2 now immutable and active: {}", user2.active);
    /*println!(
        "Cannot access user1 now though as it's non-copy values have been moved to user2: {}",
        user1.username
    );*/

    let user1 = build_user(user2.email.clone(), user3.username.clone());

    let user2 = User {
        username: String::from("New username for user2"),
        email: String::from("newemailforuser2@user2rules.com"),
        ..user1
    };

    println!(
        "Now I can access user1 and user2 because user2 is using new values for the non-copy values so everything from user1 has been copied. {} and {}",
        user1.username,
        user2.username
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        sign_in_count: 0,
        email,
    }
}
