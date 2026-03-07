// enum Direction {
//   North,
//   South,
//   East,
//   West
// }

enum Shapes {
  Circle(f64),
  Square(f64),
  Rectangle(f64, f64)
}

fn main() {
  // let curr_direction = Direction::North;
  // let new_direction = curr_direction;

  // print_direction(new_direction);

  let square = Shapes::Square(5.0);

  println!("Area of the square is {}", calculate_area(square))
}

fn calculate_area (shape: Shapes) -> f64 {
  match shape {
    Shapes::Circle(radius) => std::f64::consts::PI * radius * radius,
    Shapes::Rectangle(width, height) => width * height,
    Shapes::Square(side) => side * side
  }
}

// fn print_direction(direction: Direction) {
//   match direction {
//     Direction::North => { println!("North direction") }
//     Direction::South => { println!("South direction") }
//     Direction::East => { println!("East direction") }
//     Direction::West => { println!("West direction") }
//   }
// }