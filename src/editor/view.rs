use super::terminal::{Position, Size, Terminal};
use std::io::Error;
const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");
mod buffer;
use buffer::Buffer;
pub struct View {
    buffer: Buffer,
    need_redraw: bool,
    size: Size,
}

impl View {
    pub fn resize(&mut self, to: Size) {
        self.size = to;
        self.need_redraw = true;
    }
    fn render_line(at: usize, line_text: &str) -> Result<(), Error> {
        Terminal::move_caret_to(Position { row: at, col: 0 })?;
        Terminal::clean_line()?;
        Terminal::print(line_text)?;
        Ok(())
    }
    pub fn render(&mut self) -> Result<(), Error> {
        if !self.need_redraw {
            return Ok(());
        }
        let Size { height, width } = self.size;
        if height == 0 || width == 0 {
            return Ok(());
        }
        #[allow(clippy::integer_division)]
        let vertical_center = height / 3;

        for current_row in 0..height {
            if let Some(line) = self.buffer.lines.get(current_row) {
                let truncated_line = if line.len() >= width {
                    &line[0..width]
                } else {
                    line
                };
                Self::render_line(current_row, truncated_line)?;
            } else if current_row == vertical_center && self.buffer.is_empty() {
                Self::render_line(current_row, &Self::build_welcome_message(width))?;
            } else {
                Self::render_line(current_row, "~")?;
            }
        }
        self.need_redraw = false;
        Ok(())
    }
    fn build_welcome_message(width: usize) -> String {
        if width == 0 {
            return " ".to_string();
        }
        let welcome_message = format!("{NAME} Editor -- version {VERSION}");
        let len = welcome_message.len();
        if width <= len {
            return "~".to_string();
        }
        #[allow(clippy::integer_division)]
        let padding = (width.saturating_sub(len).saturating_sub(1)) / 2;

        let mut full_message = format!("~{}{}"," ".repeat(padding), welcome_message);
        full_message.truncate(width);
        full_message
    }
    pub fn load( & mut self, file_name: &str) {
        if let Ok(buffer) = Buffer::load(file_name) {
            self.buffer = buffer;
            self.need_redraw = true;
        }
    }
}
impl Default for View {
    fn default() -> Self {
        Self {
            buffer: Buffer::default(),
            need_redraw: true,
            size: Terminal::size().unwrap_or_default(),
        }
    }
}