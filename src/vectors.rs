fn main() {
  let vec: Vec<i32> = Vec::new();

  let even_vectors = filter_even(&vec);
  println!("{:?}", even_vectors);
}

fn filter_even(vector: &Vec<i32>) -> Vec<i32> {
  let mut even_numbers = Vec::new();
  for (_, &val) in vector.iter().enumerate() {
    if val % 2 == 0 {
      even_numbers.push(val);
    }
  }
  return even_numbers;
}