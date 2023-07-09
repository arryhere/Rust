use std::mem::size_of_val;

pub fn boolean() {
    let f: bool = true;

    if f {
        println!("Success"); // Success
        println!("{}", size_of_val(&f)); // 1 byte or 8 bits
    }
}
