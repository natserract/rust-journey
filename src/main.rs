#![allow(dead_code)]

mod ex_async_await;
mod ex_condition;
mod ex_ownership_borrowing;
mod ex_enums;
mod ex_lifetime;
mod ex_loops;
mod ex_method;
mod ex_pattern;
mod ex_structs;
mod ex_types;
mod ex_option;
mod ex_traits;
mod ex_macro;

use futures::executor::block_on;

fn main() {
    print!("-------------Condition–---------------\n");
    ex_condition::impl_of_cond(6);

    print!("-------------Data Types---------------\n");
    ex_types::impl_of_datatypes();

    print!("-------------Loop---------------\n");
    ex_loops::impl_of_loop();

    print!("-------------Enum---------------\n");
    ex_enums::impl_of_enum();

    print!("-------------Method---------------\n");
    ex_method::use_method();

    print!("-------------Data Structure---------------\n");
    ex_ownership_borrowing::borrowing();
    ex_pattern::rust_pattern();

    ex_lifetime::lifetime();

    ex_structs::impl_of_struct();

    print!("-------------Async Await ---------------\n");
    block_on(ex_async_await::impl_of_async());

    ex_option::impl_of_option();

    ex_traits::impl_of_traits();
    ex_macro::impl_of_macro();
}
