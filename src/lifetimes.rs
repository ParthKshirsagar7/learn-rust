// fn main() {
//   let longest_str;
//   let str1 = String::from("small");
//   {
//     let str2 = String::from("longer");
//     longest_str = longest(str1, str2);
//   }

//   println!("{}", longest_str);
// }

// fn longest(a: String, b: String) -> String {
//   if a.len() > b.len() {
//     a
//   } else {
//     b
//   }
// }

// fn main() {
//   let longest_str;
//   let str1 = String::from("small");
//   {
//     let str2 = String::from("longer");
//     longest_str = longest(&str1, &str2);
//   }
//   println!("{}", longest_str);
// }

// fn longest<'l>(a: &'l str, b: &'l str) -> &'l str {
//   if a.len() > b.len() {
//     a
//   } else {
//     b
//   }
// }

// struct User<'a, 'b> {
//   first_name: &'a str,
//   last_name: &'b str
// }

// fn main() {
//   let user;
//   let first_name = String::from("Parth");
//   let extracted_first_name;
//   {
//     let last_name =  String::from("Kshirsagar");
//     user = User { first_name: &first_name, last_name: &last_name };
//     extracted_first_name = user.first_name;
//   }
//   println!("The name of the user is {}", extracted_first_name);
// }

// use std::fmt::Display;
// fn longest_with_an_announcement<'a, T: Display>(
//   x: &'a str,
//   y: &'a str,
//   ann: T
// ) -> &'a str {
//   println!("Announcement! {}", ann);
//   if x.len() > y.len() {
//     x
//   } else {
//     y
//   }
// }

// fn main() {
//   let longest_str;
//   let str1 = String::from("Hello");
//   {
//     let str2 = String::from("World");
//     longest_str = longest_with_an_announcement(&str1, &str2, String::from("We won!"));
//     println!("{}", longest_str);
//   }
// }