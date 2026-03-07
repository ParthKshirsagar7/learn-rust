fn find_first_a(s: String) -> Option<usize> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index);
        }
    }
    return None;
}

fn main() {
    let my_string = String::from("a is present");
    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
}