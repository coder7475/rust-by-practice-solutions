fn main(){
  let s = sum(1, 2);
  
  println!("{}", s);
  assert_eq!(s, 3);
  println!("Success");
}

fn sum(x: i32, y:i32) -> i32 {
  return x + y;
}