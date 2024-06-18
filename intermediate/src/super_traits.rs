//
//
//
//  SUPER TRAITS
//
//
//

struct Square {
    side: f32,
    line_width: u8,
    color: String,
}

struct Rectangle {
    length: f32,
    width: f32,
    line_width: u8,
    color: String,
}
// super trait
trait Draw {
    fn draw_object(&self);
}

trait OtherTrait {} // marker trait, empty
impl OtherTrait for Rectangle {}
impl OtherTrait for Square {}

trait SomeOtherTrait {}
impl SomeOtherTrait for Rectangle {}
impl SomeOtherTrait for Square {}

// struct DrawInfo {
// line_width: u8,
// color: String,
// }

trait Shape: Draw + OtherTrait + SomeOtherTrait {
    fn area(&self) -> f32;

    // Default implementation
    fn perimeter(&self) -> f32 {
        println!("Perimeter not implemented. Returning empty value.");
        0.0
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        let area_of_rect = self.length * self.width;
        println!("Rectangle area: {}", area_of_rect);
        area_of_rect
    }

    // Overrides the default impl
    fn perimeter(&self) -> f32 {
        let perimeter_of_rect = 2.0 * (self.length + self.width);
        println!("Perimeter of rect: {}", perimeter_of_rect);
        perimeter_of_rect
    }
}

impl Shape for Square {
    fn area(&self) -> f32 {
        let area_of_square = self.side * self.side;
        println!("Square area: {}", area_of_square);
        area_of_square
    }
}

impl Draw for Rectangle {
    fn draw_object(&self) {
        println!("Drawing Rectangle");
    }
}

impl Draw for Square {
    fn draw_object(&self) {
        println!("Drawing Square");
    }
}

// Different implementations of the same idea
// impl Square {
//     fn calculate_area(&self) {
//         println!("The area is: {}", self.side * self.side);
//     }
// }

// impl Rectangle {
//     fn area(&self) -> f32 {
//         self.length * self.width
//     }
// }

// T is a type that implements the Shape trait, T is bounded by Shape
// fn shape_properties<T: Shape>(object: T) {
//     object.area();
//     object.perimeter();
// }

// // Same meaning:
// fn shape_properties<T>(object: impl Shape) {
//     object.area();
//     object.perimeter();
// }

fn shape_properties<T>(object: T)
where
    T: Shape + OtherTrait + SomeOtherTrait,
{
    object.area();
    object.perimeter();
}

// Returns an object that implements Shape trait
fn returns_shape() -> impl Shape {
    let sq = Square {
        side: 5.0,
        line_width: 5,
        color: String::from("Red"),
    };
    sq
    // let rect = Rectangle {
    //     length: 5.0,
    //     width: 10.0,
    //     line_width: 5,
    //     color: String::from("Green"),
    // };
    // let chck = false;
    // if chck {
    //     sq
    // } else {
    //     rect
    // }
}

struct Circle {
    radius: f32,
}

fn main() {
    let rect = Rectangle {
        length: 2.3,
        width: 5.0,
        line_width: 1,
        color: String::from("Blue"),
    };

    let square = Square {
        side: 4.1,
        line_width: 1,
        color: String::from("Turqoise"),
    };

    // rect.area();
    // square.area();
    // rect.perimeter();
    // square.perimeter();

    let circle = Circle { radius: 10.0 };

    shape_properties(rect);
    shape_properties(square);
    // shape_properties(circle);
}
