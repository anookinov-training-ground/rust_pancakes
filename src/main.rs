use hello_macro::HelloMacro;
// use hello_macro_derive::HelloMacro;

// #[derive(HelloMacro)]
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

fn main() {
    Pancakes::hello_macro();
}

// Attribute-like macros
// #[route(GET, "/")]
// fn index() {}

// #[proc_macro_attribute]
// pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {}

// Function-like macros
// let sql = sql!(SELECT * FROM posts WHERE id=1);

// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {}