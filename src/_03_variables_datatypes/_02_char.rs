use std::mem::size_of_val;

pub fn char() {
    let c1: char = 'a';
    println!("{}", size_of_val(&c1)); // 4 bytes or 32 bits
}
