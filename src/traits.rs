trait Summarize {
  // default implementation
  fn summarize(&self) -> String {
    return String::from("Hi");
  }
}
trait Fix {
  fn fix(&self) {
    println!("Hello world");
  }
}

struct User {
  name: String,
  age: u32
}

struct Store {
  store_name: String,
  items: Vec<String>
}

impl Summarize for User {
  // this overwrites the default implementation
  fn summarize(&self) -> String {
      return format!("{} is {} years old", self.name, self.age);
  }
}
impl Fix for User {}

impl Summarize for Store {
    fn summarize(&self) -> String {
        let mut items = String::new();
        for (i, v) in self.items.iter().enumerate() {
          if i == self.items.len()-1 {
            items.push_str(&format!("and {}", v));
          } else {
            items.push_str(&format!("{}, ", v));
          }
        }
        return format!("{} sells {}", self.store_name, items);
    }
}

// Only types which implement the Summarize trait would be able to be get passed as a parameter to this function.
fn notify(item: &impl Summarize) {
  println!("You know what, {}!", item.summarize());
}

fn summarize_and_fix<T: Summarize + Fix>(item: T) {
  println!("{}", item.summarize());
  item.fix();
}

fn main() {
  let parth = User {
    age: 18,
    name: String::from("Parth")
  };

  notify(&parth);

  let general_store = Store {
    store_name: String::from("Food basket"),
    items: vec![String::from("handwash"), String::from("brush"), String::from("razor"), String::from("toothpaste")]
  };

  notify(&general_store);

  summarize_and_fix(parth);
}