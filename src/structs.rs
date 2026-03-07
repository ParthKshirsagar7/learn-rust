struct User {
  active: bool,
  username: String,
  email: String,
  password: String,
  age: u8
}

fn main() {
  let mut user = User {
    active: true,
    username: String::from("john_doe"),
    email: String::from("xyz@example.com"),
    password: String::from("test_password"),
    age: 18
  };
  user.age = 100;
  println!("{}", user.active);
  println!("{}", user.username);
  println!("{}", user.email);
  println!("{}", user.password);
  println!("{}", user.age);
}
