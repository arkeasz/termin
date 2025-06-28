pub struct ListBox {
    items: Vec<String>,
    selected: usize,
}

impl Component for ListBox {
    fn render(&self) {
        for (i, item) in self.items.iter().enumerate() {
            if i == self.selected {
                println!("\x1B[7m> {}\x1B[0m", item);
            } else {
                println!("  {}", item);
            }
        }
    }

    fn update(&mut self, input: KeyEvent) {
        match input {
            KeyEvent::Up => {
                if self.selected > 0 {
                    self.selected -= 1;
                }
            }
            KeyEvent::Down => {
                if self.selected + 1 < self.items.len() {
                    self.selected += 1;
                }
            }
            KeyEvent::Enter => {
                // Handle selection
            }
            KeyEvent::Escape => {}
            KeyEvent::Other(_) => {}
        }
    }

    fn height(&self) -> usize {
        self.items.len()
    }

    fn width(&self) -> usize {
        self.items.iter().map(|s| s.len()).max().unwrap_or(0)
    }
}