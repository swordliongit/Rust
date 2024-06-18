//
//
//
//  TRAITS
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

// struct DrawInfo {
// line_width: u8,
// color: String,
// }

trait Shape {
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

// Different implementations of the same idea
impl Square {
    fn calculate_area(&self) {
        println!("The area is: {}", self.side * self.side);
    }
}

impl Rectangle {
    fn area(&self) -> f32 {
        self.length * self.width
    }
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

    rect.area();
    square.area();
    rect.perimeter();
    square.perimeter();
}
