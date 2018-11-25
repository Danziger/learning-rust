fn main() {
    // This is not a String. The String type is a Struct in
    // Rust's standard library.

    // This is a string slice (&str):
    let a = "Dani";

    // This is the same:
    // let target: &str = "Dani";

    println!("Hello {}", a);

    // A String is growable, mutable and UTF-8 encoded:

    let mut b = String::from("Da");

    b += "ni";

    println!("Bye {}", b);
}