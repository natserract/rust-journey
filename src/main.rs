#![allow(dead_code)]

mod async_await;
mod condition;
mod data_structure;
mod enums;
mod lifetime;
mod loops;
mod method;
mod pattern;
mod structs;
mod types;
mod option;

use futures::executor::block_on;

fn main() {
    print!("-------------Condition–---------------\n");
    condition::impl_of_cond(6);

    print!("-------------Data Types---------------\n");
    types::impl_of_datatypes();

    print!("-------------Loop---------------\n");
    loops::impl_of_loop();

    print!("-------------Enum---------------\n");
    enums::impl_of_enum();

    print!("-------------Method---------------\n");
    method::use_method();

    print!("-------------Data Structure---------------\n");
    data_structure::borrowing();
    pattern::rust_pattern();

    lifetime::lifetime();

    structs::impl_of_struct();

    print!("-------------Async Await ---------------\n");
    // block_on(async_await::impl_of_async());

    option::impl_of_option();
}
