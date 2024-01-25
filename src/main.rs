use std::env;
use rust_datastructures::Stack;

fn main() {
    let mut arg_stack = Stack::new();
    let mut args_it   = env::args();

    args_it.next(); // Discard program name from arg list.
    
    for arg in args_it {
        arg_stack.push(arg);
    }

    while let Some(arg) = arg_stack.pop() {
        println!("{arg}");
    }
}
