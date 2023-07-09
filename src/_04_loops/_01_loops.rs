use std::ops::{Range, RangeInclusive};

pub fn range() {
    for y in -3..2 {
        print!("{y} "); // -3 -2 -1 0 1
    }
    println!("");

    for z in 'a'..='z' {
        print!("{z} ") // a b c d e f g h i j k l m n o p q r s t u v w x y z
    }
    println!("");

    for z in 'a'..='z' {
        print!("{} ", z as u8) // 97 98 99 100 101 102 103 104 105 106 107 108 109 110 111 112 113 114 115 116 117 118 119 120 121 122
    }
    println!("");

    println!("---------------------------------------------------------------------------------------------------------------------------");
}

pub fn ops_range() {
    let x: Range<i32> = Range { start: 1, end: 5 };
    let y: RangeInclusive<i32> = RangeInclusive::new(1, 5);

    println!("{:?}", x); // 1..5
    println!("{:?}", y); // 1..=5
}
