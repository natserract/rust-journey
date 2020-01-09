#[allow(unused_variables)]

fn lifetime_fn<'a>(a: &'a i32) -> &'a str {
    // *a (dereference), use to get var reference (a: &i32)
    if *a < 2 {
        "More than 2"
    } else {
        "NOT"
    }
}

pub fn lifetime() {
    let mut x;

    // ownership
    fn ownership(message: &str) {
        let r = message.clone(); // move / copying here
        let mut x = String::from(r); // heap allocated
        {
            // Mutated x
            let y = &mut x; // borrow y starts here
            y.push_str(" World");
            println!("{}", y);
            // borrow y end, memory is free.
        }
        println!("{}", r);
    };

    {
        let z = 10;
        x = z;
        println!("Lifetime B: {:?}", lifetime_fn(&x));
        
        {
            let z = 1;   
            x = z;
            println!("Lifetime C: {:?}", lifetime_fn(&x));

        }
    }
    println!("{:?}", lifetime_fn(&x));

    ownership("Hello");
}
