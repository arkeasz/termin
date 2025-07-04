use windows::Win32::{Foundation::HANDLE, Security::WinAccountCertAdminsSid};

use crate::{box_chars::BoxDrawing, console::{clear_console, set_cursor_position}};

pub mod console;
pub mod box_chars;
pub mod utils;

pub enum KeyEvent {
    Up,
    Down,
    Enter,
    Escape,
    Left,
    Right,
    Other(char),
}

impl KeyEvent {
    pub fn get_code(code: u16) -> Self {
        match code {
            // VK_LEFT
            0x25 => KeyEvent::Left,    // VK_LEFT
            0x27 => KeyEvent::Right,   // VK_RIGHT
            0x26 => KeyEvent::Up,      // VK_UP
            0x28 => KeyEvent::Down,    // VK_DOWN
            0x0D => KeyEvent::Enter,   // VK_RETURN
            0x1B => KeyEvent::Escape,  // VK_ESCAPE
            // Quit
            0x51 => KeyEvent::Other('Q'),
            0x71 => KeyEvent::Other('q'), // 'q' or 'Q' to quit
            _ => KeyEvent::Other(char::from_u32(code as u32).unwrap_or('?')),
        }
    }
}

pub trait Component {
    fn render(&self);
    fn update(&mut self, input: KeyEvent);
    fn height(&self) -> usize;
    fn width(&self) -> usize;
    fn clear(&self) {
        print!("\r"); // Move cursor to the beginning of the line
        print!("\x1B[{}A", self.height()); // Move cursor up
        for _ in 0..self.height() {
            print!("\x1B[2K\r\n"); // Clear the current line
        }

        print!("\x1B[{}A\r", self.height()); // Move cursor down to the original position
    }
}


enum Style {
    Bold,
    Italic,
    Underline,
}

enum View {
    Border,
    Padding,
    Margin,
}

impl View {
    fn render(&self, content: String) -> String {
        return content;
    }
}
pub struct TUI {
    style: Vec<Style>,
    views: Vec<Box<dyn Component>>,
    size: (u16, u16),
    coords: (u16, u16),
    border: bool,
}

impl TUI {
    pub fn new(stdin: HANDLE, stdout: HANDLE) -> Self {
        let (width, height) = console::get_terminal_size().unwrap();
        
        TUI {
            style: Vec::new(),
            views: Vec::new(),
            size: (width, height),
            coords: (0, 0),
            border: false,
        }
    }

    pub fn render(&self) {
        for view in &self.views {
            view.render();
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        self.size = (width, height);
        // Optionally, you can clear the console and redraw the border
        clear_console();
    }

    pub fn draw(&mut self) {
        clear_console();
        self.render();   
    }

    pub fn border(&mut self) -> &mut Self {
        self.border = true;
        
        set_cursor_position(0, 0);
        print!("┌");
        for _ in 1..self.size.0-1 {
            print!("─");
        }
        println!("┐");
        set_cursor_position(0, self.size.1 as i16 - 1);
        print!("{}", BoxDrawing::UpRight.as_char());
        for _ in 1..self.size.0-1 {
            print!("─"); // Fill the screen with empty lines
        }
        print!("{}", BoxDrawing::UpLeft.as_char());
        set_cursor_position(0, 1);
        for i in 1..self.size.1-1 {
            set_cursor_position(0, i as i16);
            println!("│"); // Fill the screen with empty lines
        }
        set_cursor_position(0, self.size.0 as i16 - 1);
        for i in 1..self.size.1-1 {
            set_cursor_position(self.size.0 as i16 - 1, i as i16);
            println!("│"); // Fill the screen with empty lines
        }
        self
    }
    // pub fn add_view(&mut self, view: Box<dyn Component>) {
    //     self.views.push(view);
    // }

    // pub fn render(&self) {
    //     for view in &self.views {
    //         view.render();
    //     }
    // }

    // pub fn update(&mut self, input: KeyEvent) {
    //     for view in &mut self.views {
    //         view.update(input);
    //     }
    // }
}