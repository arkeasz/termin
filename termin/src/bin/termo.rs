
use std::io::{self, Write};
use termin::{box_chars::BoxDrawing, console::{clear_console, configure_console, set_cursor_position}, TUI};
use windows::Win32::{
    System::Console::{
        ReadConsoleInputW,
        INPUT_RECORD, KEY_EVENT,
    }
};

fn main() -> io::Result<()> {
    let (stdin_handle, stdout_handle) = configure_console()?;
    clear_console();
    print!("\r\x1B[2K"); // Clean and clear line

    let tui = TUI::new(stdin_handle, stdout_handle).border();

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
            let key_event = record[0].Event.KeyEvent;
            if key_event.bKeyDown {
                match key_event.wVirtualKeyCode {
                    0x1B => { // Escape key
                        break; // Exit the loop
                    }
                    _ => {
                        // Handle other keys if needed
                    }
                }
            }
        }
    }

    clear_console();
    Ok(())
}