//
//
//
// Combinators
//
//

// 1. Combinators methods on iterators that are compact pure functions, can be linked together to execute complex operations.
// 2. They are lazy methods, no work is done until their next() is called

fn main() {
    let words = vec!["apple", "banana", "grape", "orange", "pear"];
    // let mut result: Vec<String> = vec![];

    // for word in words {
    //     if word.starts_with("a") || word.starts_with("b") {
    //         let uppercase_word = word.to_uppercase();
    //         result.push(uppercase_word);
    //     }
    // }

    // println!("{:?}", result);

    let result: Vec<String> = words
        .into_iter()
        .filter(|&word| word.starts_with("a") || word.starts_with("b"))
        .map(|word| word.to_uppercase())
        .collect(); // collect::<Vec<String>> turbo fish syntax as alternative

    println!("{:?}", result);
}
