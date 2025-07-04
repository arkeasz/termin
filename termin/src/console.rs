use std::io::{self, Error, Write};
use std::mem;
use windows::Win32::System::Console::{GetConsoleScreenBufferInfoEx, CONSOLE_SCREEN_BUFFER_INFOEX};
use windows::Win32::{
    Foundation::HANDLE, 
    System::Console::{
        self, 
        COORD,
        GetConsoleMode, 
        GetStdHandle, 
        SetConsoleMode, 
        SetConsoleCursorPosition,
        CONSOLE_MODE, 
        DISABLE_NEWLINE_AUTO_RETURN, 
        ENABLE_ECHO_INPUT, 
        ENABLE_LINE_INPUT, 
        ENABLE_PROCESSED_INPUT, 
        ENABLE_PROCESSED_OUTPUT, 
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, 
        ENABLE_WRAP_AT_EOL_OUTPUT, 
        STD_INPUT_HANDLE, 
        STD_OUTPUT_HANDLE
    }
};


pub fn configure_console() -> io::Result<(HANDLE, HANDLE)> {
     let stdin = unsafe { GetStdHandle(STD_INPUT_HANDLE) }
        .map_err(|_| io::Error::last_os_error())?;
    let stdout = unsafe { GetStdHandle(STD_OUTPUT_HANDLE) }
        .map_err(|_| io::Error::last_os_error())?;

    let mut out_mode = 0;
    unsafe { GetConsoleMode(stdout, &mut CONSOLE_MODE(out_mode))?; }
    out_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING.0
              | ENABLE_PROCESSED_OUTPUT.0
              | ENABLE_WRAP_AT_EOL_OUTPUT.0
              | DISABLE_NEWLINE_AUTO_RETURN.0;
    unsafe { SetConsoleMode(stdout, CONSOLE_MODE(out_mode))?; }

    let mut in_mode = 0;
    unsafe { GetConsoleMode(stdin, &mut CONSOLE_MODE(in_mode))?; }
    in_mode &= !(ENABLE_LINE_INPUT.0 | ENABLE_ECHO_INPUT.0);
    in_mode |= ENABLE_PROCESSED_INPUT.0;
    unsafe { SetConsoleMode(stdin, CONSOLE_MODE(in_mode))?; }

    Ok((stdin, stdout))
}

pub fn clear_console() {
    print!("\x1B[2J\x1B[H"); // Clear screen
    io::stdout().flush().unwrap();
}

pub fn get_terminal_size() -> Result<(u16, u16), Error> {
    unsafe {
        // Get the standard output handle
        let h_out: HANDLE = GetStdHandle(STD_OUTPUT_HANDLE)
            .map_err(|_| Error::last_os_error())?;
        
        // Get the console screen buffer info
        let mut info: Console::CONSOLE_SCREEN_BUFFER_INFOEX = mem::zeroed();
        info.cbSize = mem::size_of::<CONSOLE_SCREEN_BUFFER_INFOEX>() as u32;
        
        if GetConsoleScreenBufferInfoEx(h_out, &mut info).is_err() {
            return Err(Error::last_os_error());
        }

        let w = (info.srWindow.Right - info.srWindow.Left + 1) as u16;
        let h = (info.srWindow.Bottom - info.srWindow.Top + 1) as u16;

        Ok((w, h))
    }
}

// \n or \r are not used in the terminal, so we use ANSI escape codes to set the cursor position
// because they are more efficient and work across different platforms
// This function sets the cursor position to (x, y) in the terminal
pub fn set_cursor_position(x: i16, y: i16) {
    print!("\x1B[{};{}H", y + 1, x + 1);
    io::stdout().flush().unwrap();
}