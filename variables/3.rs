// * Scope: A scope is the range within the program for which the item is valid
fn main() {
    let x: i32 = 10;
    let y: i32 = 5;

    {
        println!("The value of x is {} and value of y is {}", x, y);
    }

    println!("The value of x is {} amd value of y is {}",  x, y);
}