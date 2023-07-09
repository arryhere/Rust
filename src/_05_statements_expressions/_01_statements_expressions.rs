pub fn statemets_expressions() {
    let x: u32 = 5_u32;

    let y: u32 = {
        let x_squared: u32 = x * x;
        let x_cube: u32 = x * x * x;

        x_squared + x_cube
    };

    let z: () = {
        let x_squared: u32 = x * x;
        let x_cube: u32 = x * x * x;

        // this semicolon supresses this expression and () is assigned to z
        x_squared + x_cube;
    };

    println!("x: {x}"); // x: 5
    println!("y: {y}"); // y: 150
    println!("z: {:?}", z); // z: ()
}
