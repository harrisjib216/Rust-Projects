use std::fmt::{Display, Formatter, Result};

struct PersonAge(u8);

impl Display for PersonAge {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "This person's age is {}", self.0)
    }
}

#[derive(Debug)]
struct MinMax(u64, u64);

impl Display for MinMax {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl Display for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 25);
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let point = Point {
        x: 2.5,
        y: 25.50,
    };

    println!("\nDisplay: {}", point);
    println!("Debug: {}", point);
}