

#[allow(dead_code)]
pub fn impl_of_enum() {
    #[derive(Debug)]
    enum Theme {
        Dark(Point),
    }

    #[derive(Debug)]
    enum Keys {
        Upkeys(String)
    }

    #[derive(Debug)]
    struct Point {
        color: u32
    }

    impl Theme {
        fn main_dir(&self) -> Keys {
            match *self {
                Theme::Dark(_) => Keys::Upkeys(String::from("Hello"))
            }
        }
    };


    let u = Theme::Dark(Point{
        color: 0
    });
    let k = u.main_dir();

    print!("K is: {:?}", k)
}