fn main(){
    let x = String::from("Hello World");
    // * Borowing
    // let y = &x; 
    // * Cloning
    let y = x.clone();
    println!("{}, {}", x, y);
}