//
//
//
//      GENERICS
//

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

// Specialization
impl Point<i32, i32> {
    fn print_coord(&self) {
        println!("Point: {}, {}", self.x, self.y);
    }
}

impl Point<f64, f64> {
    // Duplicate name is allowed for different specializations
    fn print_coord(&self) {
        println!("Point: {}, {}", self.x, self.y);
    }
}

// Free Functions
fn add_points<T, U>(p1: &Point<T, U>, p2: &Point<T, U>) -> Point<T, U> {
    unimplemented!();
}

// Monomorphization
// fn add_points_i32<i32, i32>(p1: &Point<i32, i32>, p2: &Point<i32, i32>) -> Point<i32, i32> {
//     unimplemented!();
// }

// fn add_points_f64<f64, f64>(p1: &Point<f64, f64>, p2: &Point<f64, f64>) -> Point<f64, f64> {
//     unimplemented!();
// }

fn main() {
    let origin = Point::new(0, 0);
    let p1 = Point { x: 1.2, y: 3.5 };

    let p2 = Point { x: 12.0, y: 7 };

    origin.print_coord();
    p1.print_coord();

    add_points(&origin, &origin);
    add_points(&p1, &p1);
}
