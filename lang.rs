//--- lang.rs
pub fn print_hello() {
    println!("Hello, world!");
}

mod english {
    mod greetings {
          fn hello() -> String {
          "Hello!".to_string()
      }
    }

    mod farewells {
        fn goodby() -> String {
          "bye.".to_string()
      }
    }
}

mod japanese {
    mod greetings {
        fn hello() -> String {
        "Moshi moshi!".to_string()
      }
    }
    mod farewells {
       fn goodby() -> String {
          "sayonara.".to_string()
      }   
    }
}
