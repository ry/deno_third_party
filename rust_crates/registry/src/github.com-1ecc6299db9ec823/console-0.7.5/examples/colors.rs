extern crate console;

use console::style;

fn main() {
    println!(
        "This is red on black: {:010x}",
        style(42).red().on_black().bold()
    );
    println!("This is reversed: [{}]", style("whatever").reverse());
    println!("This is cyan: {}", style("whatever").cyan());
}
