
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
    let items = [
        "Home",
        "About",
        "Settings",
    ];
    let mut selected = 0;

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
    let new_coords = (
        re as u16, 
        re as u16, 
        width - re*2, 
        height - re*2
    );
    // Print menu items
    // the width of the menu is will be able for porcentage of the terminal size
    // the height of the menu is will be able for porcentage of the terminal size
    // like 20% of the terminal size 
    // by default the menu will be 100% of the relative size

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
    set_cursor_position(new_coords.0 as i16 + re as i16, new_coords.1 as i16 + 1);
    let dyncom = (new_coords.0 as i16 + re as i16, new_coords.1 as i16 + 1);
    for (i, item) in items.iter().enumerate() {
        if i == selected {
            print!("  \x1B[7m> {}\x1B[0m", item); // Highlight selected item
        } else {
                print!("    {}", item); // Normal item
            }
    }
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

                match ke.wVirtualKeyCode {
                    VK_UP => {
                        if selected > 0 { selected -= 1; };
                        
                    }
                    VK_DOWN => {
                        if selected + 1 < items.len() { selected += 1; }
                    }
                    VK_RETURN => {
                        if selected == items.len() - 1 {
                            break;
                        } else {
                            // Aquí puedes manejar Option 1 o 2
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
                    io::stdout().flush()?;
                }
            }
        }
    }

    clear_console();
    Ok(())
}