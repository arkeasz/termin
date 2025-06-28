use std::io::{self, Write};

fn main() {
    // clear the console
    print!("\x1B[2J\x1B[H");
    io::stdout().flush();
            println!("Select an option:");

    let items = [
        "Option 1: Do something",
        "Option 2: Do something else",
        "Option 3: Exit",
    ];
        // Print the menu items
    for (i, item) in items.iter().enumerate() {
        // if i == selected {
            print!("\x1B[7m> da {}\x1B[0m\n", item); // Highlight the selected item
        // } else {
            // println!("  {}", item);
        // }
    }
}