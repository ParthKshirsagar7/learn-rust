use std::collections::HashMap;

fn main() {
  let pairs = vec![
    (String::from("parth"), 18),
    (String::from("someone"), 100)
  ];

  let hashmap = create_hashmap_from_vector(pairs);

  println!("{:?}", hashmap);
}

fn create_hashmap_from_vector(vector: Vec<(String, i32)>) -> HashMap<String, i32> {
  let mut hm: HashMap<String, i32> = HashMap::new();

  for (key, value) in vector {
    hm.insert(key, value);
  }

  return hm;
}