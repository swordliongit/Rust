//
//
//
// Function Pointers
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

// function pointers implement all the 3 closure traits
// functions can be passed to anywhere where a closure is expected
fn is_valid_user(
    name: &str,
    age: u8,
    banned_user_name: &str,
    simple_validator: fn(&str, &str) -> bool,
    advanced_validator: fn(u8) -> bool,
) -> bool {
    simple_validator(name, banned_user_name) && advanced_validator(age)
}

fn validate_user_simple(name: &str, banned_user_name: &str) -> bool {
    name.len() != 0 && name != banned_user_name
}

fn validate_user_advanced(age: u8) -> bool {
    age >= 30
}

fn wrapper_func(text: &str, fp: fn(&str) -> usize) {
    fp(text);
}

fn text_length(text: &str) -> usize {
    text.len()
}

fn main() {
    let person1 = User {
        name: String::from("someone"),
        age: 32,
        salary: 40_000,
    };
    let banned_user_name = "banned user";
    // let validate_user_simple = move |name: &str| name.len() != 0;
    // let validate_user_advanced = |age: u8| age >= 30;

    wrapper_func("example", text_length);

    println!(
        "User validity: {}",
        is_valid_user(
            &person1.name,
            person1.age,
            banned_user_name,
            validate_user_simple,
            validate_user_advanced
        )
    );
}
