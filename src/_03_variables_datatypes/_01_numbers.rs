/*
• Signed Integer    - can represent both positive and negative integers
• Unsigned Integer  - can represent only positive integers


•   length      Signed      Unsigned
    8-bit       i8          u8
    16-bit      i16         u16
    32-bit      i32         u32
    64-bit      i64         u64
    128-bit     i128        u128
    arch        isize       usize


• Default Types -
    Integers - i32
    Float    - f64


• Range -
    unsigned 8-bit   u8     -   [0, 255]
    unsigned 16-bit  u16    -   [0, 65,535]
    unsigned 32-bit  u32    -   [0, 4,294,967,295]
    unsigned 64-bit  u64    -   [0, 18,446,744,073,709,551,615]
    unsigned 128-bit u128   -   [0, 340,282,366,920,938,463,463,374,607,431,768,211,455]

    signed 8-bit   i8       -   [-128, 127]
    signed 16-bit  i16      -   [-32,768, 32,767]
    signed 32-bit  i32      -   [-2,147,483,648, 2,147,483,647]
    signed 64-bit  i64      -   [-9,223,372,036,854,775,808, 9,223,372,036,854,775,807]
    signed 128-bit i128     -   [-170,141,183,460,469,231,731,687,303,715,884,105,728, 170,141,183,460,469,231,731,687,303,715,884,105,727]


• Negative numbers -
    Negative numbers are stored in computers as 2's complement -
        signed 8-bit i8 system -
            0       1 1 1 1 1 1 1     (127)
            1       0 0 0 0 0 0 0     (1's complement)
            1       0 0 0 0 0 0 1    (-127, 2's complement)
            MSB     -----LSB-----


• Architecture Reference -
    In a 32-bit architecture, the system can access 32 bits at a time
    In a 64-bit architecture, the system can access 64 bits at a time

    1 word = 32 bits (4 bytes) in a 32-bit system
    1 word = 64 bits (8 bytes) in a 64-bit system
*/

pub fn integers() {
    let a: i64 = 5;
    let b: i32 = 10;
    let c = 15_u8;
    let d: u16 = 20_u8 as u16;

    println!("a: {a}, b: {b}, c: {c}, d: {d}"); // a: 5, b: 10, c: 15, d: 20

    println!("---------------------------------------------------------------------------------------------------------------------------");

    let max_u8: u8 = u8::MAX;
    let max_i8: i8 = i8::MAX;

    println!("max_u8: {max_u8}, max_i8: {max_i8}"); // max_u8: 255, max_i8: 127

    println!("---------------------------------------------------------------------------------------------------------------------------");

    // let v1: u8 = 250_u8 + 50; // arithmetic_overflow
    let v1: u16 = 250_u16 + 50;
    let v2: u16 = u16::checked_add(250, 50).unwrap();

    println!("v1: {v1}, v2: {v2}"); // v1: 300, v2: 300

    println!("---------------------------------------------------------------------------------------------------------------------------");

    let num = 1024 + 0xff + 0o77 + 0b11111111; // 1024 + 255 + 63 + 255

    println!("num: {num}"); // num: 1597
}

pub fn float() {
    
}