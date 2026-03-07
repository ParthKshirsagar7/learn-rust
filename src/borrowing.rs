fn main() {
  let mut s1 = String::from("Hello world");
  let s2 = &mut s1;

  s2.push('a');
  println!("{}", s2);
  let s3 = &s1;
  println!("{}", s3);
}

// As in the above example, s2 is considered as a mutable reference of s1 till it is last used in the code. After the last use of the mutable reference, s1 be comes free again and multiple immutable references or another single mutable reference of s1 can be created.