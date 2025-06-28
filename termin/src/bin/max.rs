use std::io::{self, Write};
use windows::Win32::System::Console::{
    GetConsoleMode, GetStdHandle, ReadConsoleInputW, SetConsoleMode, CONSOLE_MODE,
    ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT,
    ENABLE_VIRTUAL_TERMINAL_PROCESSING, INPUT_RECORD, KEY_EVENT, STD_INPUT_HANDLE,
    STD_OUTPUT_HANDLE,
};

// Virtual key codes in snake_case
const VK_UP: u16 = 0x26;
const VK_DOWN: u16 = 0x28;
const VK_RETURN: u16 = 0x0D;
const VK_ESCAPE: u16 = 0x1B;

fn main() -> io::Result<()> {
    // Get the standard input and output handles
    let stdin_handle = unsafe { GetStdHandle(STD_INPUT_HANDLE)? };
    let stdout_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE)? };

    // Configure console mode for OUTPUT
    let mut out_mode: u32 = 0;
    unsafe {
        GetConsoleMode(stdout_handle, &mut CONSOLE_MODE(out_mode))?;
    }
    out_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING.0;
    unsafe {
        SetConsoleMode(stdout_handle, CONSOLE_MODE(out_mode))?;
    }

    // Configure console mode for INPUT (CORRECTED)
    let mut in_mode: u32 = 0;
    unsafe {
        GetConsoleMode(stdin_handle, &mut CONSOLE_MODE(in_mode))?;
    }
    // CORRECCIÓN: Solo deshabilitar las banderas necesarias
    in_mode &= !(ENABLE_ECHO_INPUT.0 | ENABLE_LINE_INPUT.0);
    in_mode |= ENABLE_PROCESSED_INPUT.0; // Eliminado VT para input
    unsafe {
        SetConsoleMode(stdin_handle, CONSOLE_MODE(in_mode))?;
    }

    let items = [
        "Option 1: Do something",
        "Option 2: Do something else",
        "Option 3: Exit",
    ];
    let mut selected = 0;

    loop {
        // Clear console
        print!("\x1B[2J\x1B[H");
        io::stdout().flush()?;

        println!("Select an option:");

        // Print menu items
        for (i, item) in items.iter().enumerate() {
            if i == selected {
                println!("\x1B[7m> {}\x1B[0m", item);
            } else {
                println!("  {}", item);
            }
        }

        let mut records = [unsafe { std::mem::zeroed::<INPUT_RECORD>() }];
        let mut events_read = 0;

        // CORRECCIÓN: Uso correcto de ReadConsoleInputW
        unsafe {
            ReadConsoleInputW(
                stdin_handle,
                &mut records,
                &mut events_read,
            )?;
        }

        if events_read > 0 && records[0].EventType == KEY_EVENT as u16 {
            let key_event = unsafe { records[0].Event.KeyEvent };
            if key_event.bKeyDown == true {
                match key_event.wVirtualKeyCode {
                    VK_UP if selected > 0 => selected -= 1,
                    VK_DOWN if selected < items.len() - 1 => selected += 1,
                    VK_RETURN => {
                        if selected == items.len() - 1 {
                            break;
                        }
                        // Handle other options here
                    }
                    VK_ESCAPE => break,
                    _ => (),
                }
            }
        }
    }

    Ok(())
}