
fn main() {
    let x = Box::new(5);
    
    let mut y = Box::new(3);      // update this line, don't change other lines!
    
    *y = 4;


    println!("x, y: {}, {}", *x, *y);
    
    assert_eq!(*x, 5);

    println!("Success!");
}