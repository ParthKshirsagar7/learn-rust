fn main () {
  // let mut v = vec![1, 2, 3];

  // // Iter
  // let mut iter = v.iter();
  // while let Some(val) = iter.next() {
  //   println!("{}", val);
  // }

  // // Mut Iter
  // let iter_mut = v.iter_mut();
  // for val in iter_mut {
  //   *val *= 10;
  // }

  // println!("{:?}", v);

  // // Into Iter
  // let into_iter = v.into_iter();
  // for val in into_iter {
  //   println!("{}", val);
  // }

  // let v = vec![1, 2, 3, 4, 5, 6];
  
  // // consuming adapters (consumes the iterators its called upon)
  // let v_iter = v.iter();
  // let sum: i32 = v_iter.sum();
  // println!("{}", sum);

  // // iterator adapters (returns another iterator)
  // let iter = v.iter();
  // let iter2 = iter.map(|x| x*10);
  // for val in iter2 {
  //   println!("{}", val);
  // }
  // let iter3 = iter.filter(|x| *x%2 == 0);
  // for val in iter3 {
  //   println!("{}", val);
  // }

  // assignment
  let v = vec![1, 2, 3, 4, 5, 6, 7];
  let new_vec: Vec<i32> = v.into_iter().filter(|x| *x % 2 == 1).map(|x| x*2).collect();
  for val in new_vec {
    println!("{}", val);
  }
}