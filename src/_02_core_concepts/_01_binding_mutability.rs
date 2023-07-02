pub fn binding_mutability() {
    let mut x: i32 = 5;
    // let y: i32 = 55;

    x += 5;
    // y += 55; // cannot mutate immutable variable

    assert_eq!(x, 10);
    println!("Success !")
    
}
