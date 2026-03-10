fn main() {
  let s = String::from("Hello");

  let word1 = first_word(&s);
  
  println!("{}", word1);
}

fn first_word(s: &String) -> &str{

  for (i, v) in s.chars().enumerate(){
    if v == ' ' {
      return &s[0..i];
    }
  }
  return &s;
}