#![allow(dead_code)]

use std::{
    error::Error,
    io::{Stdout, Write, stdout},
};

use crossterm::{
    ExecutableCommand, QueueableCommand,
    cursor::MoveTo,
    style::{self, Color, PrintStyledContent, StyledContent},
    terminal,
};

use crate::util::Position;

pub trait Renderable {
    fn render(&self) -> Vec<DrawCmd>;
    fn erase(&self) -> Vec<DrawCmd>;
}
#[derive(Clone)]
pub struct DrawCmd {
    pub pos: Position,
    pub content: StyledContent<char>,
}
impl DrawCmd {
    pub fn new(chr: StyledContent<char>, pos: Position) -> Self {
        Self { pos, content: chr }
    }
}

pub struct Renderer {
    output: Stdout,
    pub size: (u16, u16),
    origin: (i16, i16),
    draw_cmds: Vec<DrawCmd>,
}

impl Renderer {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let size = terminal::size()?;
        Ok(Self {
            output: stdout(),
            size,
            origin: ((size.0 / 2) as i16, (size.1 / 2) as i16),
            draw_cmds: Vec::new(),
        })
    }
    pub fn init() -> Result<Self, Box<dyn Error>> {
        let mut r = Renderer::new()?;
        terminal::enable_raw_mode()?;
        r.output
            .execute(terminal::EnterAlternateScreen)?
            .execute(style::SetBackgroundColor(Color::DarkGreen))?
            .execute(terminal::Clear(terminal::ClearType::All))?
            .execute(crossterm::cursor::Hide)?; // Hide the cursor
        Ok(r)
    }

    pub fn translate_pos(&self, pos: Position) -> (u16, u16) {
        let (w, h) = self.size;

        let origin_x = w as i16 / 2;
        let origin_y = h as i16 / 2;

        let x = origin_x + pos.0;
        let y = origin_y - pos.1;

        if x >= 0 && y >= 0 && x < w as i16 && y < h as i16 {
            (x as u16, y as u16)
        } else {
            panic!("position out of bounds: ({x}, {y}) for screen {w}x{h}");
        }
    }

    pub fn put(&mut self) -> Result<(), Box<(dyn Error + 'static)>> {
        let cmds = std::mem::take(&mut self.draw_cmds);

        for cmd in cmds {
            let (x, y) = self.translate_pos(cmd.pos);
            self.output
                .queue(MoveTo(x, y))?
                .queue(PrintStyledContent(cmd.content))?;
        }
        Ok(())
    }

    pub fn end_frame(&mut self) -> Result<(), Box<(dyn Error + 'static)>> {
        self.output.flush()?;
        Ok(())
    }
    pub fn render_frame(&mut self) -> Result<(), Box<(dyn Error + 'static)>> {
        self.put()?;
        self.end_frame()?;
        Ok(())
    }
    pub fn queue(&mut self, cmds: Vec<DrawCmd>) {
        self.draw_cmds.extend(cmds);
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        self.output.flush().unwrap();
        self.output.execute(terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
