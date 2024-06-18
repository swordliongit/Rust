//
//
//
// Closures
//
//
//

struct User {
    name: String,
    age: u8,
    salary: u32,
}

// fn validate_user(name: &str) -> bool {
//     name.len() != 0
// }

fn is_valid_user<V1, V2>(name: &str, age: u8, simple_validator: V1, advanced_validator: V2) -> bool
where
    V1: FnOnce(&str) -> bool, // because it moves ownership
    V2: Fn(u8) -> bool,
{
    simple_validator(name) && advanced_validator(age)
}

fn main() {
    let person1 = User {
        name: String::from("someone"),
        age: 32,
        salary: 40_000,
    };
    // 3 types of traits that get implemented for the closure
    // 1. Immutable borrow -> Fn
    // 2. Mutable borrow -> FnMut
    // 3. Change of ownership -> FnOnce
    let banned_user = String::from("Banned User");
    let validate_user_simple = move |name: &str| {
        let banned_user_inner = banned_user;
        name.len() != 0 && name != banned_user_inner
    };
    let validate_user_advanced = |age: u8| age >= 30;

    // println!("{banned_user}"); // ERROR

    println!(
        "User validity: {}",
        is_valid_user(
            &person1.name,
            person1.age,
            validate_user_simple,
            validate_user_advanced
        )
    );
}
