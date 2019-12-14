
#![allow(unused_variables)]
pub fn impl_of_datatypes() {
    let bool_types: bool = true;
    let char_types = 'x';
    let integer_types: (i32, f32, u64, u8) = (2147483647, 21.4, 18446744073709551615, 255); // tuples
    let fixed_arr: [i32; 3] = [1, 2, 3];
    let string_types: &str = "Alfin";
    let mut arr_heap: Vec<i32> = Vec::new();

    // Tupples Array & Descturturing
    let tupple_step = (20, 30, 40, (1, 2, 3));

    // Ignore values - _name_func  => option var, no unused variables
    let (a, _, _, (x, _, _)) = tupple_step;
    
    fn get_return(a: i32) -> bool { // fn name() -> types of return
        a % 2 == 0
    }

    arr_heap.push(5);
    println!("Arr Heap: {:?}", arr_heap);
    
    println!(
        "
        Boolean types: {},
        Char Types: {},
        Integer Types: {:?},
        Fixed Array: {:?},
        String Types: {},
        Tupple: {}
        ",
        bool_types, char_types, integer_types, fixed_arr, string_types, a
    );
    
    println!("{}", get_return(2));
}

pub extern fn add_one(){
    println!("Hello World");
}
