fn main()  {
  never_return();
  // make it unreachable
  println!("Failed!");
}

fn never_return()  {
  panic!("Thread Panicked");
}