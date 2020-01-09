/* Documentation:
- https://medium.com/@phoomparin/a-beginners-guide-to-rust-macros-5c75594498f1 */

macro_rules! fmt {
    ($name:expr) => {
        println!("My name is: {:?}", $name);
    };
}
pub fn impl_of_macro() {
    fmt!("Alfin")
}
