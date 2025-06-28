
use std::io::{self, Write};
use termin::console::{clear_console, configure_console};
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
    let (stdin_handle, stdout_handle) = configure_console()?;

    // Menu items
    let items = [
        "op 1",
        "op 2",
        "op 3",
    ];
    let mut selected = 0;

    // Initial render
    print!("\x1B[?1049h"); // Enable alternate buffer
    clear_console();

    println!("Select an option:");
    
    for (i, item) in items.iter().enumerate() {
            print!("\r\x1B[2K"); // Limpia y vuelve a inicio de línea

        if i == selected {
            println!("\x1B[7m> {}\x1B[0m", item);
        } else {
            println!("  {}", item);
        }
    }
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
                    // move cursor to top left
                    print!("\x1B[{}A", items.len()); // Move cursor up
                    
                    for (i, item) in items.iter().enumerate() {
                        print!("\r\x1B[2K"); // Clear current line

                        if i == selected {
                            println!("\x1B[7m> {}\x1B[0m\r", item);   
                        } else {
                            println!("  {}\r", item);
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