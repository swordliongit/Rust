//
//
//
//  Box Smart Pointer
//
//

//      Simple Pointer          ||        Smart Pointer
// ---------------------------------------------------------------------
//  Just stores memory address  ||  Special capabilities
//  Indicated by &              ||  Not just simple references
//  Also called references      ||
//  No special capabilities     ||

// Box is similar to unique_ptr in C++

/*
Idea is to have lists that contain other lists.
Error : recursive type `List` has infinite size. Problem with recursive types is that,
Rust compiler is unable to know the exact size of the instance of such type at compile time.
Rust has to know the sizes of variables at compile time.

* How does Rust calculate the memory size of a variable?:
For example:
    enum Conveyance {
        Car(i32),
        Train(i32),
        Air(i32),
        Walk
    }

    - Rust goes through the variants to determine how much memory to allocate.
    - Car, Train and Air require a fixed size memory that is i32 and Walk doesn't take any mem space
    - Rust allocates an i32 sized memory space for the enum. So the Conveyance enum takes i32 space
    - in the memory.

    - In contrast to the List enum below, the compiler knows Nil won't take any space.
    - But it can't determine how much space the Cons variant will take because the second value of
    - Cons is a List in itself, that is recursive in nature.
    - It's unknown how many times it will recurse.
    - Solution to this is to use the Box pointer.
*/

// #[derive(Debug)]
// enum List {
//     Cons(i32, List),
//     Nil,
// }
// fn main() {
//     let x = 0.600;
//     let y = Box::new(x); // points to a heap memory containing the value of x
//     let z = &x; // stack memory pointer

//     let list = List::Cons(1, List::Cons(2, List::Cons(3, List::Nil)));
// }
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
fn main() {
    let x = 0.600;
    let y = Box::new(x); // points to a heap memory containing the value of x
    let z = &x; // stack memory pointer

    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("{:?}", list);
}
