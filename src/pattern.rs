// This is main features of Rust
pub fn rust_pattern() {
    fn desctruct() {
        struct Types {
            age: i32,
        }

        let person = Types { age: 0 };
        let Types { age, .. } = person; // .. it's mean like rest operator
        println!("{}", age);
    };

    fn reference() {
        let s = String::from("Hello");
        let y = &s;
        println!("Reference {}", y);
    };

    fn ownership() {
        // One owner
        let arr = vec![1, 2, 3, 4, 5];
        let mut clone_arr = arr; // arr move & borrow to clone_arr

        for i in 6..12 {
            clone_arr.push(i);
        }
        println!("Ownership {:?}", clone_arr);
    }


    desctruct();
    reference();
    ownership();
}
