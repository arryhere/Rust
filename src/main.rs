#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]

mod _01_intro {
    pub mod _01_hello_world;
}
// use crate::_01_intro::_01_hello_world;

mod _02_core_concepts {
    pub mod _01_binding_mutability;
    pub mod _02_scope;
    pub mod _03_entrypoint;
    pub mod _04_shadowing;
    pub mod _05_compiler_warnings;
    pub mod _06_destructuring;
}
// use crate::_02_core_concepts::_01_binding_mutability;
// use crate::_02_core_concepts::_02_scope;
// use crate::_02_core_concepts::_03_entrypoint;
// use crate::_02_core_concepts::_04_shadowing;
// use crate::_02_core_concepts::_05_compiler_warnings;
// use crate::_02_core_concepts::_06_destructuring;

mod _03_variables_datatypes {
    pub mod _01_numbers;
}
use crate::_03_variables_datatypes::_01_numbers;

fn main() {
    println!("Hello, Rust!");
    println!("---------------------------------------------------------------------------------------------------------------------------");

    // _01_hello_world::intro();

    // _01_binding_mutability::binding_mutability();
    // _02_scope::scope();
    // _03_entrypoint::entrypoint();
    // _04_shadowing::shadowing();
    // _05_compiler_warnings::compiler_warnings();
    // _06_destructuring::destructuring();

    _01_numbers::integers();
}
