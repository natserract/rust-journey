/**
    A trait is a collection of methods defined 
    for an unknown type: Self. They can access other 
    methods declared in the same trait.
**/

#[derive(Debug)]
struct Value {
    single: bool,
}

trait Random {
    fn check_single(&self) -> Self;
}

impl Random for Value {
    fn check_single(&self) -> Self {
        Value {
            single: self.single
        }
    }
}

pub fn impl_of_traits() {
    let my_gender = Value { single: true };

    println!("Are you single? {:?}", my_gender.check_single().single);

}
