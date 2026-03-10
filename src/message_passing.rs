use std::{sync::mpsc, thread};

fn main() {
  let (tx, rx) = mpsc::channel();
  for i in 0..10 {
    let transmitter = tx.clone();
    thread::spawn(move || {
      let mut ans: u64 = 0;
      for j in 1..10000001 {
        ans += i * 10000000 + j;
      }
      transmitter.send(ans).unwrap();
    }) ;
  }
  drop(tx);

  let mut ans: u64 = 0;
  for val in rx {
    ans += val;
  }
  println!("Got {}", ans);
}