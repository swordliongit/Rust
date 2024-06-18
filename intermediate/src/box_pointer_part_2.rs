//
//
//
//  Box Smart Pointer
//
//

// A better optimized version using Option
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}
fn main() {
    let x = 0.600;
    let y = Box::new(x); // points to a heap memory containing the value of x
    let z = &x; // stack memory pointer

    // last variant doesn't need heap allocation, so it's just None
    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
    );

    println!("{:?}", list);
}
