use std::io::{self, Write};
use windows::Win32::{
    Foundation::HANDLE,
    System::Console::{
        GetConsoleMode, GetStdHandle, ReadConsoleInputW, SetConsoleMode, CONSOLE_MODE,
        ENABLE_ECHO_INPUT, ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT,
        ENABLE_VIRTUAL_TERMINAL_PROCESSING, INPUT_RECORD, KEY_EVENT, STD_INPUT_HANDLE,
        STD_OUTPUT_HANDLE,
    },
};

// Virtual key codes en snake_case
const VK_UP: u16 = 0x26;
const VK_DOWN: u16 = 0x28;
const VK_RETURN: u16 = 0x0D;
const VK_ESCAPE: u16 = 0x1B; // Tecla ESC añadida

fn main() -> io::Result<()> {
    // Obtener handles de entrada/salida
    let stdin_handle = unsafe { GetStdHandle(STD_INPUT_HANDLE)? };
    let stdout_handle = unsafe { GetStdHandle(STD_OUTPUT_HANDLE)? };

    // Configurar modo de consola para SALIDA
    let mut out_mode: u32 = 0;
    unsafe {
        GetConsoleMode(stdout_handle, &mut CONSOLE_MODE(out_mode))?;
    }
    out_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING.0;
    unsafe {
        SetConsoleMode(stdout_handle, CONSOLE_MODE(out_mode))?;
    }

    // Configurar modo de consola para ENTRADA (¡CORREGIDO!)
    let mut in_mode: u32 = 0;
    unsafe {
        GetConsoleMode(stdin_handle, &mut CONSOLE_MODE(in_mode))?;
    }
    // Deshabilitar eco y entrada por línea
    in_mode &= !(ENABLE_ECHO_INPUT.0 | ENABLE_LINE_INPUT.0);
    // Mantener procesamiento y añadir VT
    in_mode |= ENABLE_PROCESSED_INPUT.0 | ENABLE_VIRTUAL_TERMINAL_PROCESSING.0;
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
        // Limpiar consola (secuencia corregida)
        print!("\x1B[2J\x1B[H");
        io::stdout().flush()?;

        println!("Select an option:");

        // Imprimir items del menú
        for (i, item) in items.iter().enumerate() {
            if i == selected {
                println!("\x1B[7m> {}\x1B[0m", item); // Texto corregido
            } else {
                println!("  {}", item);
            }
        }

        let mut record = [unsafe { std::mem::zeroed::<INPUT_RECORD>() }];
        let mut events_read = 0;

        unsafe {
            ReadConsoleInputW(
                stdin_handle,
                &mut record,
                &mut events_read,
            )?;
        }

        if events_read > 0 && record[0].EventType == KEY_EVENT as u16 {
            
            let key_event = unsafe { record[0].Event.KeyEvent };
            if key_event.bKeyDown == true {

                match key_event.wVirtualKeyCode {
                    VK_UP if selected > 0 => selected -= 1,
                    VK_DOWN if selected < items.len() - 1 => selected += 1,
                    VK_RETURN => {
                        if selected == items.len() - 1 {
                            break;
                        }
                        // Acciones para otras opciones
                    }
                    VK_ESCAPE => break, // Salir con ESC
                    _ => (),
                }
            }
        }
    }

    Ok(())
}