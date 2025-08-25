mod apple;
mod renderer;
mod snake;
mod util;
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal;
use std::time::{Duration, Instant};
//use stack::{Offset, Stack, StackMatrix, StackRenderer, Unselected};
use std::error::Error;

use crate::apple::Apple;
use crate::renderer::Renderable;
use crate::snake::{Snake, SnakeSegment};
use crate::{renderer::Renderer, util::Direction};

#[derive(Clone)]
enum GameState {
    Playing,
    Paused,
    GameOver,
}
struct Game {
    snake: Snake,
    apple: Apple,
    renderer: Renderer,
    state: GameState,
}

impl Game {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let renderer = Renderer::init()?;

        Ok(Self {
            apple: Apple::new((4, 0)),
            snake: Snake::new(),
            state: GameState::Playing,
            renderer,
        })
    }
    pub fn collsion(snake_head: &SnakeSegment, apple: &Apple) -> bool {
        snake_head.pos == apple.pos
    }

    pub fn run(mut self) -> Result<(), Box<dyn Error>> {
        let target_fps: i32 = 60;
        let target_frame_time = Duration::from_secs_f32(1.0 / target_fps as f32);

        let mut frames_elapsed: i32 = 0;
        self.snake.grow_snake();
        self.snake.grow_snake();
        self.snake.grow_snake();
        self.snake.grow_snake();
        self.snake.grow_snake();
        self.snake.grow_snake();
        self.snake.grow_snake();
        self.snake.grow_snake();

        self.renderer.queue(self.apple.render());
        self.renderer.queue(self.snake.render_snake());

        while matches!(self.state, GameState::Playing) {
            let frame_start = Instant::now();

            // Drain all pending events without blocking
            while event::poll(std::time::Duration::from_millis(0))? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('k') => self.snake.turn(Direction::North),
                        KeyCode::Char('j') => self.snake.turn(Direction::South),
                        KeyCode::Char('h') => self.snake.turn(Direction::West),
                        KeyCode::Char('l') => self.snake.turn(Direction::East),
                        KeyCode::Char('q') | KeyCode::Esc => self.state = GameState::GameOver,

                        _ => {}
                    }
                }
            }

            if frames_elapsed.rem_euclid(self.snake.speed) == 0 {
                let step = self.snake.next_step();
                self.renderer.queue(step.render());
                self.snake.move_snake(step);
            }

            if Self::collsion(self.snake.head(), &self.apple) {
                self.snake.grow_by(1);
                self.apple.move_apple(self.renderer.size);
                self.renderer.queue(self.apple.render());
            }

            self.renderer.render_frame()?;

            frames_elapsed += 1;

            if frames_elapsed == target_fps {
                frames_elapsed = 0
            }
            let elapsed = frame_start.elapsed();
            if elapsed < target_frame_time {
                std::thread::sleep(target_frame_time - elapsed);
            }
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let solitaire = Game::new()?;
    solitaire.run()?;
    Ok(())
}
