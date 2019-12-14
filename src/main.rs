
mod method;
mod condition;
mod types;
mod data_structure;
mod loops;
mod enums;
mod pattern;
mod lifetime;
mod structs;



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
    
}
