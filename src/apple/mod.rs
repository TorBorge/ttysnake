use crate::util::pos_add;
use crossterm::style::Stylize;
use rand::Rng;

use crate::{
    renderer::{self, DrawCmd, Renderable},
    util::Position,
};

pub struct Apple {
    pub pos: Position,
    color: crossterm::style::Color,
}

impl Apple {
    pub fn new(pos: Position) -> Self {
        Self {
            pos,
            color: crossterm::style::Color::Red,
        }
    }

    pub fn move_apple(&mut self, screen_size: (u16, u16)) {
        let mut limit_x = (screen_size.0 / 2) as i16;
        let limit_y = (screen_size.1 / 2) as i16;

        if limit_x.rem_euclid(2) == 0 {
            limit_x += 1;
        }

        let mut rng = rand::rng();
        self.pos = (
            rng.gen_range(-limit_x..limit_x),
            rng.gen_range(-limit_y..limit_y),
        )
    }
}

impl Renderable for Apple {
    fn render(&self) -> Vec<renderer::DrawCmd> {
        vec![
            DrawCmd::new('█'.with(self.color), self.pos),
            DrawCmd::new('█'.with(self.color), pos_add(self.pos, (1, 0))),
        ]
    }
    fn erase(&self) -> Vec<DrawCmd> {
        vec![
            DrawCmd::new(' '.with(self.color), self.pos),
            DrawCmd::new(' '.with(self.color), pos_add(self.pos, (1, 0))),
        ]
    }
}
