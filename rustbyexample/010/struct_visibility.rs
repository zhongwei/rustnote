mod my {
    pub struct WitheBox<T> {
        pub contents: T,
    }

    #[allow(dead_code)]
    pub struct BlackBox<T> {
        contents: T,
    }

    impl<T> BlackBox<T> {
        pub fn new(contents: T) -> BlackBox<T> {
            BlackBox {
                contents: contents,
            }
        }
    }
}

fn main() {
    let white_box = my::WitheBox { contents: "public information" };

    println!("The white box contains: {}", white_box.contents);

    // let black_box = my::BlackBox { contents: "classified information" };

    let _balck_box = my::BlackBox::new("classified information");

    //println!("The black box contains: {}", _black_box.contents);
}
