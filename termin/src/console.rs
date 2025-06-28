use std::io::{self, Write};

use windows::Win32::{
    Foundation::HANDLE, 
    System::Console::{
        GetConsoleMode, 
        GetStdHandle, 
        SetConsoleMode, 
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