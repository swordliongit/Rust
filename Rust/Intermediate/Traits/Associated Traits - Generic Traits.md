- Lets us have a placeholder type to use in trait definitions that can be defined by the implementers


```rust
trait DistanceThreeHours {
    type Distance; // Associated Type
    fn distance_in_three_hours(&self) -> Self::Distance;
}

impl DistanceThreeHours for Kmh {
    type Distance = Km;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}

impl DistanceThreeHours for Mph {
    type Distance = Miles;
    fn distance_in_three_hours(&self) -> Self::Distance {
        Self::Distance {
            value: self.value * 3,
        }
    }
}
```


# Associated vs Generic

- Use ==associated types== when there's only a single implementation of a trait per type.
- If there are multiple implementations of a trait per type, use ==generics==.
## Problem

```rust
trait Addition {
    type Rhs;
    type Output;
    fn add(self, rhs: Self::Rhs) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Addition for Point {
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

// conflicting implementations of trait `Addition` for type `Point`  
// conflicting implementation for `Point`rustc
impl Addition for Point {
    type Rhs = Point;
    type Output = Point;
    fn add(self, rhs: Self::Rhs) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    let p3 = p1.add(p2);

    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 6);
}

```


## Solution with Generics

```rust
trait Addition<Rhs> {
    type Output;
    fn add(self, rhs: Rhs) -> Self::Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Addition<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32> for Point {
    type Output = Point;
    fn add(self, rhs: i32) -> Self::Output {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    let p3 = p1.add(p2);

    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 6);
    
	let p1 = Point { x: 1, y: 1 };
    let p3 = p1.add(2);
    
    assert_eq!(p3.x, 3);
}
```


# Example:

```rust
//
//
//
// Assocaited Traits vs Generic Types
//
//
trait Addition<Rhs, Output> {
    fn add(self, rhs: Rhs) -> Output;
}

struct Point {
    x: i32,
    y: i32,
}

impl Addition<Point, Point> for Point {
    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Addition<i32, Point> for Point {
    fn add(self, rhs: i32) -> Point {
        Point {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}

struct Line {
    start: Point,
    end: Point,
}

impl Addition<Point, Line> for Point {
    fn add(self, rhs: Point) -> Line {
        Line {
            start: self,
            end: rhs,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };

    let p3: Point = p1.add(p2); // must add the explicit type annotation

    assert_eq!(p3.x, 4);
    assert_eq!(p3.y, 6);

    let p1 = Point { x: 1, y: 1 };
    let p3 = p1.add(2);
    assert_eq!(p3.x, 3);

    let p1 = Point { x: 1, y: 1 };
    let p2 = Point { x: 2, y: 2 };
    let line: Line = p1.add(p2);

    assert!(line.start.x == 1 && line.start.y == 1 && line.end.x == 2 && line.end.y == 2);
}

```