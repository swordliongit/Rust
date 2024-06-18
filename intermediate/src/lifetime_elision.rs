//
//
//  Lifetime Elision
//
//
//

/*
1. Each parameter that is a reference, gets its own lifetime parameter.
2. If there is exactly one input lifetime parameter, that lifetime is assigned to
all output lifetime parameters.
3. If there are multiple input lifetime parameters, but one of them is &self or &mut self,
the lifetime of self is assigned to all output lifetime parameters.
*/

fn main() {
    let some_str = "Some str";
    let received_str = return_str(&some_str);
}

fn return_str(str: &str) -> &str {
    str
}
