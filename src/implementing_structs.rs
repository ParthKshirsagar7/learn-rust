struct Rect {
  width: f32,
  height: f32
}

impl Rect {
  fn area(&self) -> f32 {
    return self.width * self.height;
  }
  fn increment_width_and_height(&mut self) {
    self.width += 1.0;
    self.height += 1.0;
  }
}

fn main() {
  let mut square = Rect{
    width: 4.0,
    height: 4.0
  };

  square.increment_width_and_height();
  println!("{}", square.area());
}