

pub fn impl_of_cond(param_cond: i64) {

    fn age_range() -> u32 {
        10
    };

    match age_range() {
        0 => println!("Return 0"),
        n @ 1..=10 => println!("Age: {:?}", n),
        n => println!("NAN: {:?}", n),
    };

    match param_cond {
        6 | 7 | 8 => println!("This number from 6..8"),
        param_cond if param_cond < 5 => println!("This number less than 5"),
        _ => println!("Return false"), // Default case
    };

    if param_cond == 6 | 7 | 8 {
        println!("Your Range 6..8");
    } else if param_cond != 5 {
        println!("Not 5");
    } else {
        println!("Your Range Not in 6..8");
    }
}
