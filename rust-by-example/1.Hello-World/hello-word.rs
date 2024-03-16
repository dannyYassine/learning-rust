// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.
    let full_name: String = String::from("Danny Yassine");
    // Print text to the console.
    println!("Hello World!");
    println!("{}", full_name);

    let name: &str = &full_name[0..5];
    println!("{}", name);
    println!("{name} {last_name}", name="Danny", last_name="Yassine");
}