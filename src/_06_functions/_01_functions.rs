pub fn function() {
    let res = sum(4, 5);
    println!("res: {res}"); // res: 9

    let x = print(); // i love functions !

    let x = never_return();
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn print() -> () {
    println!("i love functions !")
}

// diverging functions - never return to the caller, so they may be used in places where a value of any type is expected
fn never_return() -> ! {
    panic!();
    // todo!();
    // unimplemented!();
}
