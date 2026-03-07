fn main () { 
  // Conditionals
  // let num = 45;
  // let is_even = num % 2 == 0;

  // if is_even {
  //   println!("{} is Even", num);
  // } else {
  //   println!("{} is Odd", num);
  // }

  // Loops
  // for i in 0..11 {
  //   println!("{}", i)
  // }

  // Functions
  let msg = String::from("Hey! I'm good, how are you?");
  let first_word = get_first_word(msg);
  println!("{}", first_word);
}

fn get_first_word (sentence: String) -> String {
  let mut ans = String::new();
  for ch in sentence.chars() {
    ans.push(ch);
    if ch == ' ' {
      break;
    }
  }

  return ans;
}