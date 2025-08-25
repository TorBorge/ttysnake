#![allow(dead_code)]
use std::collections::{HashSet, VecDeque};

use crate::{
    renderer::{DrawCmd, Renderable},
    util::{Direction, Position, pos_add, pos_neg},
};
use crossterm::style::{Color, Stylize};

#[derive(Debug, Clone)]
pub struct SnakeStep {
    pub freed: Option<SnakeSegment>, // tail cell that became empty
    pub occupied: SnakeSegment,      // new head cell
}

impl Renderable for SnakeStep {
    fn render(&self) -> Vec<DrawCmd> {
        if let Some(segment) = &self.freed {
            vec![self.occupied.render(), segment.erase()].concat()
        } else {
            self.occupied.render()
        }
    }
    fn erase(&self) -> Vec<DrawCmd> {
        if let Some(segment) = &self.freed {
            vec![self.occupied.erase(), segment.render()].concat()
        } else {
            self.occupied.erase()
        }
    }
}

#[derive(Clone, Debug)]
pub struct SnakeSegment {
    pub pos: Position,
    dir: Direction,
    color: Color,
}

impl Renderable for SnakeSegment {
    fn render(&self) -> Vec<DrawCmd> {
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

pub struct Snake {
    body: VecDeque<SnakeSegment>, // head at front
    cells: HashSet<Position>,     // mirrors body
    dir: Direction,
    pending_growth: usize,
    pub speed: i32,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: VecDeque::new(),
            cells: HashSet::new(),
            dir: Direction::East,
            pending_growth: 0,
            speed: 10,
        }
    }

    pub fn grow_snake(&mut self) {
        let (pos, dir, color) = if let Some(last) = self.body.back() {
            let pos = pos_add(last.pos, pos_neg(last.dir.delta()));
            (pos, last.dir, last.color)
        } else {
            ((-4, 0), self.dir, Color::Blue)
        };
        let new_seg = SnakeSegment { pos, dir, color };
        self.cells.insert(new_seg.pos);
        self.body.push_back(new_seg);
    }

    pub fn grow_by(&mut self, n: usize) {
        self.pending_growth += n;
    }

    pub fn next_step(&self) -> SnakeStep {
        let freed = if self.pending_growth == 0 {
            Some(self.body.back().unwrap().clone())
        } else {
            None
        };

        let cur_head = self.body.front().unwrap();
        let new_head_pos = pos_add(cur_head.pos, self.dir.delta());

        let new_head = SnakeSegment {
            pos: new_head_pos,
            dir: cur_head.dir,
            color: cur_head.color,
        };

        SnakeStep {
            freed,
            occupied: new_head,
        }
    }

    pub fn move_snake(&mut self, next_step: SnakeStep) {
        if let Some(tail) = next_step.freed {
            self.body.pop_back();
            self.cells.remove(&tail.pos);
        } else {
            self.pending_growth -= 1;
            self.grow_snake();
        }

        self.cells.insert(next_step.occupied.pos);
        self.body.push_front(next_step.occupied);

        assert_eq!(self.body.len(), self.cells.len());
    }
    pub fn head(&self) -> &SnakeSegment {
        self.body.front().unwrap()
    }

    pub fn render_snake(&self) -> Vec<DrawCmd> {
        self.body
            .iter()
            .map(|segment| segment.render())
            .collect::<Vec<Vec<DrawCmd>>>()
            .concat()
    }

    pub fn erase_tail(&self) -> Vec<DrawCmd> {
        if let Some(last) = self.body.back() {
            last.erase()
        } else {
            panic!("ahhh")
        }
    }

    pub fn render_head(&self) -> Vec<DrawCmd> {
        if let Some(last) = self.body.front() {
            last.render()
        } else {
            panic!("ahhh")
        }
    }

    pub fn turn(&mut self, dir: Direction) {
        if dir.delta() == pos_neg(self.dir.delta()) {
            return;
        }
        self.dir = dir
    }
}
