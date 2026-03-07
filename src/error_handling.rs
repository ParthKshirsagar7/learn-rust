use std::fs;

fn main() {
  let res = fs::read_to_string("examples.txt");

  match res {
      Ok(content) => {
        println!("File content: {}", content)
      },
      Err(error) => {
        println!("Error: {}", error);
      }
  }
  println!("Hello");
}