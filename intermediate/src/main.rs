//
//
//
//  Reference Counting Smart Pointer
//
//

use std::rc::Rc;

enum List {
    Cons(i32, Option<Rc<List>>),
    Nil,
}

fn main() {
    let a = Rc::new(List::Cons(1, Some(Rc::new(List::Cons(2, None))))); // first reference to this heap location
    println!("Reference count after a: {}", Rc::strong_count(&a));
    {
        let b = List::Cons(3, Some(Rc::clone(&a))); // a is borrowed, this clone is not a deep copy,
                                                    // it increments the reference counts of a, now it's 2
        println!("Reference count after b: {}", Rc::strong_count(&a));
        let c = List::Cons(4, Some(Rc::clone(&a))); // a is borrowed again, count is 3
        println!("Reference count after c: {}", Rc::strong_count(&a));
    }
    println!(
        "Reference count after inner scope: {}",
        Rc::strong_count(&a)
    );
}
