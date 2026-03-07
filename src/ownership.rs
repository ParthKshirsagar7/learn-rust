fn main() {
  // let s = String::from("Hello world");
  // let mut s1 = s;
  // s1.push('a');
  // // println!("{}", s); Will throw an error becuase s1 has taken over the pointer of heap memory address of "Hello world" from s and is the new owner of the string. The program doesn't compile if we try to use s.
  // println!("{}", s1); // Output: Hello worlda

  // let msg = String::from("Hello world");
  // print_str(msg);
  // println!("{}", msg); // Will throw an error since now "some_str" present inside the print_str function is the owner of "Hello world" string.

  // let msg = String::from("Hello world");
  // print_str(msg.clone());
  // println!("{}", msg); // Will print "Hello world" because we are passing a clone of msg in print_str function.

  let mut msg = String::from("Hello world");
  msg = print_str_2(msg);
  println!("{}", msg);
}

// fn print_str(some_str: String) {
//   println!("{}", some_str);
// }

fn print_str_2(some_str: String) -> String {
  println!("{}", some_str);
  return some_str
}