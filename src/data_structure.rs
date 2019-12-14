
// Borrowing
pub fn borrowing() {
    let mut empty_arr = Vec::new();
    // Or you can use this => let mut empty_arr = vec![];
    let u = 10;
    let a = &u;
    let ref z = u;

    if z == a {
        println!("They are equal");
    }

    let mut x = 5;
    {
        let y = &mut x;
        *y += 1;
    }
    println!("Count {}", x);

    // Scope
    {
        for i in 1..12 {
            empty_arr.push(i)
        }
    }

    println!("Count: {:?}", empty_arr);
    print!("Array length: {:?}", empty_arr.len());
}

