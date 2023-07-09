use std::mem::size_of_val;

pub fn unit() {
    let _v: () = ();

    let v: (i32, i32) = (2, 3);

    println!("{:?}", _v); // ()
    println!("{:?}", v); // (2, 3)

    println!("{}", size_of_val(&_v)); // 0 byte or 0 bits
}
