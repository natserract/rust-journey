

// initial struct / type
struct Color(i32, i32, i32);

struct State<T>{
    title: T
} 


// what the struct do
impl<T> State<T> { // generic type
    fn get_title(&self) -> &T {
        &self.title
    }
}


pub fn impl_of_struct(){
    // data in struct
    let find_title = State {
        title: "Alfin" 
    };

    let get_color = Color(255, 255, 90);

    println!("{}", find_title.get_title());
    println!("{}, {}, {}", get_color.0, get_color.1, get_color.2);
}