fn main() {
    let (mut x, mut y) = (5, 55); // tuple

    x += 5;
    y += 10;

    println!("x: {x}, y: {y}"); // x: 10, y:

    destructure();
}

fn destructure() {
    let (x, y);

    (x, ..) = (1, 2, 3);
    [.., y] = [1, 2, 3];

    println!("x: {x}, y: {y}"); // x: 1, y: 3
}
