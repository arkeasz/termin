pub mod console;

pub enum KeyEvent {
    Up,
    Down,
    Enter,
    Escape,
    Other(char),
}

impl KeyEvent {
    pub fn get_code(code: u16) -> Self {
        match code {
            0x26 => KeyEvent::Up,      // VK_UP
            0x28 => KeyEvent::Down,    // VK_DOWN
            0x0D => KeyEvent::Enter,   // VK_RETURN
            0x1B => KeyEvent::Escape,  // VK_ESCAPE
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