pub mod advanced_functions_and_closures;
pub mod advanced_types;
pub mod macros;
pub mod unsafe_rust;

fn main() {
    unsafe_rust::entry();
    advanced_types::entry();
    advanced_functions_and_closures::entry();
    macros::entry();
}
