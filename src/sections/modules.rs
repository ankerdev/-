use super::functions::create_heading;

pub fn run() {
    println!("\n{}", create_heading("Modules"));
    greetings::hello();
    greetings::polite::hello();
}

// A module does not need to be public to be accessed by the parent scope,
// but it needs to be public if it will be used by other modules.
mod greetings {
    // Everything inside a module is private by default,
    // whether that be a function or a nested module.
    // To make it accessible from outside of the scope,
    // the members need to be made public.
    pub fn hello() {
        println!("Hello!");
    }

    pub mod polite {
        pub fn hello() {
            println!("Hello, sir.");
        }
    }
}
