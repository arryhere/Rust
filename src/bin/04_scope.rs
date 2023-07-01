fn main() {
    let x: i32 = 5;

    {
        let y: i32 = 10;
        println!("x: {x}, y: {y}"); // x: 5, y: 10
    }

    // println!("x: {x}, y: {y}");  // error
}
