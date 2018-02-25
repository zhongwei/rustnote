mod my {
    fn private_function() {
        println!("called `my::private_function()`");
    }

    pub fn function() {
        println!("called `my::function()`");
    }

    pub fn indirect_access() {
        print!("called `my::indirect_access()`, that\n> ");
        private_function();
    }

    pub mod nested {
        pub fn fucntion() {
            println!("called `my::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called `my::nested::private_funtion`");
        }
    }

    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called `my::private_nested::funtion()`");
        }
    }
}

fn function() {
    println!("called `fucntion()`");
}

fn main() {
    function();
    my::function();

    my::indirect_access();
    my::nested::fucntion();

    // my::private_function();
    // my::nested::private_funtion();
    // my::private_nested::funtion();
}
