enum Shape {
    Square(f32),
    Circle(f32),
    Rectangle(f32, f32),
}

impl Shape {
    fn area(&self) -> f32 {
        match self {
            Shape::Square(side) => return side * side,
            Shape::Circle(r) => return 3.14 * r * r,
            Shape::Rectangle(x, y) => return 2.0 * (x + y),
        }
    }

    fn perimeter(&self) -> f32 {
        match self {
            Shape::Square(side) => return 4.0 * side,
            Shape::Circle(r) => return 2.0 * 3.14 * r,
            Shape::Rectangle(x, y) => return x * y,
        }
    }
}

fn main() {
    let shape = Shape::Square(10.0);
    let shape_circle = Shape::Circle(10.0);
    let shape_rect = Shape::Rectangle(10.0, 12.0);
    println!("{}", shape.area());
    println!("{}", shape_circle.area());
    println!("{}", shape_rect.area());
    println!("{}", shape.perimeter());
    println!("{}", shape_circle.perimeter());
    println!("{}", shape_rect.perimeter());
}
