
use std::io::{self, Write};
use termin::{box_chars::BoxDrawing, console::{clear_console, configure_console, set_cursor_position}};
use windows::Win32::{
    System::Console::{
        ReadConsoleInputW,
        INPUT_RECORD, KEY_EVENT,
    }
};

// Virtual key codes in snake_case
const VK_UP: u16 = 0x26; // Up arrow key
const VK_LEFT: u16 = 0x25; // Left arrow key
const VK_RIGHT: u16 = 0x27; // Right arrow key
const VK_DOWN: u16 = 0x28; // Down arrow key
const VK_RETURN: u16 = 0x0D; // Enter key
const VK_ESCAPE: u16 = 0x1B; // Escape key

fn main() -> io::Result<()> {
    // Configure console for raw input   
    let (
        stdin_handle, 
        stdout_handle
    ) = configure_console()?;

    // Menu items
    let items: [&'static str; 4] = [
        "Home",
        "About",
        "Settings",
        "Exit",
    ];
    let mut selected: usize = 0;

    // Initial render
    print!("\x1B[?1049h"); // Enable alternate buffer
    clear_console();

    let (width, height): (u16, u16) = termin::console::get_terminal_size()?;

    print!("\r\x1B[2K"); // Clean and clear line

    print!("┌");
    for _ in 1..width-1 {
        print!("─");
    }
    println!("┐");
    set_cursor_position(0, height as i16 - 1);
    print!("{}", BoxDrawing::UpRight.as_char());
    for _ in 1..width-1 {
        print!("─"); // Fill the screen with empty lines
    }
    print!("{}", BoxDrawing::UpLeft.as_char());
    set_cursor_position(0, 1);
    for i in 1..height-1 {
        set_cursor_position(0, i as i16);
        println!("│"); // Fill the screen with empty lines
    }
    set_cursor_position(0, width as i16 - 1);
    for i in 1..height-1 {
        set_cursor_position(width as i16 - 1, i as i16);
        println!("│"); // Fill the screen with empty lines
    }

    // coords
    // initial cords (0,0) (0, y_length) (x_length, 0) (x_length, y_length)
    // new coords (padding, margin or border) relative to the initial coords
    // we will call this var re
    // (0 + re, 0 + re) (0 + re, y_length - re) (x_length - re, 0 + re) (x_length - re, y_length - re)
    let mut re = 2u16; // Relative size for padding, margin or border
    let mut new_coords = (
        re as u16, 
        re/2 as u16, 
        width - re*2, 
        height - re*2
    );
    // Print menu items
    // the width of the menu is will be able for porcentage of the terminal size
    // the height of the menu is will be able for porcentage of the terminal size
    // like 20% of the terminal size 
    // by default the menu will be 100% of the relative size

    // set_cursor_position(new_coords.0 as i16, new_coords.1 as i16);
    set_cursor_position(new_coords.0 as i16, new_coords.1 as i16);
    // make a border for the menu
    print!("┌");
    for i in new_coords.0..new_coords.2 {
        print!("─");
        if i == new_coords.0 - 1 {
            print!("┐");
        } 
    }
    println!("┐");
    set_cursor_position(new_coords.0 as i16, new_coords.1 as i16 + 1);
    print!("│");
    set_cursor_position(new_coords.0 as i16 + re as i16, new_coords.1 as i16 + 1);
    let dyncom = (new_coords.0 as i16 + re as i16, new_coords.1 as i16 + 1);
    for (i, item) in items.iter().enumerate() {
        if i == selected {
            print!("  \x1B[7m> {}\x1B[0m", item); // Highlight selected item
        } else {
            print!("    {}", item); // Normal item
        }
    }
    set_cursor_position(
        new_coords.2 as i16 + 1,
        new_coords.1 as i16 + 1);
    println!("│");

    set_cursor_position(
        new_coords.0 as i16, 
        new_coords.1 as i16 + 2);
    print!("{}", BoxDrawing::UpRight.as_char());
    for i in new_coords.0..new_coords.2 {
        print!("─");
        if i == new_coords.2 - 1 {
            print!("{}", BoxDrawing::UpLeft.as_char());
            break;
        } 
    }
    // set_cursor_position(0, 0);
    // we take the las cursor position that is the last line of the menu
    set_cursor_position(
        new_coords.0 as i16, 
        new_coords.1 as i16 + 3
    );

    // let state =
    let mut view = "Home";
    new_coords.1 += 3; // Adjust the new coords for the view
    // new_coords.2 -= 1; // Adjust the new coords for the view
    // print borders
    print!("{}", BoxDrawing::DownRight.as_char());
    for i in new_coords.0..new_coords.2 {
        print!("─");
        if i == new_coords.2 - 1 {
            print!("{}\n", BoxDrawing::DownLeft.as_char());
            break;
        } 
    }

    // now we print the border laterals
    for i in new_coords.1..new_coords.3 + 1 {
        set_cursor_position(new_coords.0 as i16, i as i16 + 1);
        print!("│");
        set_cursor_position(new_coords.2 as i16 + 1, i as i16 + 1);
        print!("│");
    }

    set_cursor_position(
        new_coords.0 as i16,
        new_coords.3 as i16 + 2
    );
    print!("{}", BoxDrawing::UpRight.as_char());
    for i in new_coords.0..new_coords.2 {
        print!("─");
        if i == new_coords.2 - 1 {
            print!("{}", BoxDrawing::UpLeft.as_char());
            break;
        } 
    }

    new_coords.0 += 1; // Adjust the new coords for the view
    new_coords.1 += 1; // Adjust the new coords for the view
    new_coords.2 -= 1; // Adjust the new coords for the view
    new_coords.3 -= 1; // Adjust the new coords for the view

    set_cursor_position(
        new_coords.0 as i16, 
        new_coords.1 as i16 
    );
    print!("Home");
    io::stdout().flush()?; // Forzar impresión
    // loop
    loop {


        let mut record: [INPUT_RECORD; 1] = unsafe { std::mem::zeroed() };
        let mut read = 0u32;

        unsafe {
            ReadConsoleInputW(
                stdin_handle,
                &mut record,
                &mut read,
            )?;
        }

        if record[0].EventType == KEY_EVENT as u16 {
            let ke = unsafe { record[0].Event.KeyEvent };
            if ke.bKeyDown == true {
                let mut redraw = true;
                let mut change_view = false;
                match ke.wVirtualKeyCode {
                    VK_LEFT => {
                        if selected > 0 { selected -= 1; };
                        
                    }
                    VK_RIGHT => {
                        if selected + 1 < items.len() { selected += 1; }
                    }
                    VK_RETURN => {
                        if selected == items.len() - 1 {
                            break;
                        } else if selected == 0 {
                            view = "Home";
                            change_view = true;
                        } else if selected == 1 {
                            view = "About";
                            change_view = true;
                        } else if selected == 2 {
                            view = "Settings";
                            change_view = true;
                        }
                    }
                    0x51 /* 'Q' */ => break,
                    _ => redraw = false, // Ignore other keys
                }

                if redraw {
                    set_cursor_position(dyncom.0, dyncom.1);
                    
                    for (i, item) in items.iter().enumerate() {
                        if i == selected {
                            print!("  \x1B[7m> {}\x1B[0m", item);   
                        } else {
                            print!("    {}", item);
                        }
                    }

                    if change_view {
                        set_cursor_position(
                            new_coords.0 as i16, 
                            new_coords.1 as i16 
                        );
                        print!("{}    ", view);
                    }
                    io::stdout().flush()?;
                }
            }
        }
    }

    clear_console();
    Ok(())
}