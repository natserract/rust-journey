

#[allow(unused_assignments)]
pub fn impl_of_loop() {
    let mut value_start = 2;
    let lists = vec!["Apples", "Bananas", "Bla"];

    /* Constant can't mutable */
    const MAX_NUMBER: i64 = 20; // Should be uppercase
    
    for x in 1..MAX_NUMBER {
        println!("Number is {}", x)
    }

    /* Conditional Loop */
    while value_start != 0 {
        println!("While Loop Number => {}!", value_start);

        value_start += 1;
        break; // if hasn't break has infinite loop
    }

    /* For Loop */
    for loop_param in lists.iter() {
        println!("For Loop => {}", loop_param);
    }
}