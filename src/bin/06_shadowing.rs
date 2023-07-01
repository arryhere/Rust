fn main() {
    let x: i32 = 5;

    {
        let x: i32 = 55;
        println!("x: {x}") // x: 55
    }

    println!("x: {x}"); // x: 5

    let x = 555;
    println!("x: {x}"); // x: 555

    shadow();
}

fn shadow() {
    let mut x: i32 = 5;
    x = 55;

    let mut x: i32 = x;
    x += 555;

    println!("x: {x}"); // x: 610

    let mut y: i32 = 10;
    // y = "Hello Rust";
    let y: &str = "Hello Rust";

    println!("y: {y}") // y: Hello Rust
}
