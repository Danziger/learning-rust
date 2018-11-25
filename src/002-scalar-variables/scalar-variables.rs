
// This won't work as let can't be used in the global scope:
// let GLOBAL = 10;

// However, this will work:
const GLOBAL: i8 = 100;

fn main() {

    // LET:
    // Cannot be declared in the global scope:s

    // INTEGERS (SIGNED):
    let integer8: i8 = -1;
    let integer16: i16 = -2;
    let integer32: i32 = -3;
    let integer64: i64 = -4;
    // let integer128: i128 = -5;
    let integer: isize = -6;

    // INTEGERS (UNSIGNED):
    let uinteger8: u8 = 1;
    let uinteger16: u16 = 2;
    let uinteger32: u32 = 3;
    let uinteger64: u64 = 4;
    // let uinteger128: u128 = -5;
    let uinteger: usize = 6;

    // FLOAT:
    let float32: f32 = 1.0;
    let float64: f64 = 1.5;

    // BOOLEAN:
    let boolean: bool = true;

    // CHAR:
    let character: char = 'a';

    // MUTABILITY:
    // Variables are inmutable by default, this will fail:
    // integer8 = 0.0;

    let mut mutableInteger8: i8 = 1;

    println!("VALUE BEFORE = {}", mutableInteger8);

    // Max value, otherwise it wraps:
    mutableInteger8 = 127;

    println!("VALUE AFTER = {}", mutableInteger8);

    // CONSTANTS:
    // const do not accept mut, require a type and uppercase name,
    // can be declared in the global scope and its value must be
    // hardcoded:
    const CONSTANT: i8 = 10;

    println!("CONSTANT = {}", CONSTANT);

    println!("GLOBAL CONSTANT = {}", GLOBAL);
}