// * Shadowing: Declare a new variable with the same name as previous variable
fn main() {
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 12);
    }

    assert_eq!(x, 5);
    let x: i32 = 42;
    println!("{}", x);
}