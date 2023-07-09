#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_must_use)]

mod _01_intro {
    pub mod _01_hello_world;
}

mod _02_core_concepts {
    pub mod _01_binding_mutability;
    pub mod _02_scope;
    pub mod _03_entrypoint;
    pub mod _04_shadowing;
    pub mod _05_compiler_warnings;
    pub mod _06_destructuring;
}

mod _03_variables_datatypes {
    pub mod _01_numbers;
    pub mod _02_char;
    pub mod _03_boolean;
    pub mod _4_unit;
}

mod _04_loops {
    pub mod _01_loops;
}

mod _05_statements_expressions {
    pub mod _01_statements_expressions;
}

mod _06_functions {
    pub mod _01_functions;
}

mod _07_ownership {
    pub mod _01_ownership;
}

// use crate::_01_intro::_01_hello_world;
// use crate::_02_core_concepts::{_01_binding_mutability, _02_scope, _03_entrypoint, _04_shadowing, _05_compiler_warnings, _06_destructuring};
// use crate::_03_variables_datatypes::{_01_numbers, _02_char, _03_boolean, _4_unit};
// use crate::_04_loops::_01_loops;
// use crate::_05_statements_expressions::_01_statements_expressions;
// use crate::_06_functions::_01_functions;
use crate::_07_ownership::_01_ownership;

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

    // _01_numbers::integers();
    // _01_numbers::float();
    // _01_numbers::number_ops();
    // _02_char::char();
    // _03_boolean::boolean();
    // _4_unit::unit();

    // _01_loops::range();
    // _01_loops::ops_range();

    // _01_statements_expressions::statemets_expressions();

    // _01_functions::function();

    _01_ownership::ownership();
}
