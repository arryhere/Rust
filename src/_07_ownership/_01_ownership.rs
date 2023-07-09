/*
• Ownership

• Rust's ownership system is unique and sets it apart from other programming languages
• It implements some set of rules that govern memory management
• Rules are enforced at compile time. If any of the rules are violated the code won't compile

• Owner -
  - The owner of a value is the variable or the data structure that holds it and is responsible for allocating 
    and freeing the memory used to store that data

• Three Rules of Ownership
  - Each value in rust has a owner
  - There can only be one owner at a time
  - When owner goes out of scope the value will be dropped

• Scope -
  - Range within a program for which an item is valid
    - Global Scope  - Accessible throughout the entire program
    - Local Scope   - Accessible only within a particular code block or a function, not accessible outside of it

• Memory -
  - Memory is a component in a computer to store data and instructions for processes to execute
  - RAM - Random Access Memory is volatile, when power turned off all contents are lost
  - Two Types of region in RAM used by a program at runtime -
    - Stack Memory
    - Heap Memoryn
*/

pub fn ownership() {}
