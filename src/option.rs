
fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0i32 {
        None
    } else {
        Some(x/y)
    }
}

pub fn impl_of_option() {
    let result = divide(20, 0);
    
    match result {
        None => println!("Error occured"),
        Some(result)=> println!("The result is: {:?}", result)
    }
}