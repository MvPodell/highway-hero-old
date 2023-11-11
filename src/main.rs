mod game;
mod input;

// get the width and height of the whole game screen
pub const  WINDOW_WIDTH: f32 = 90.0;
pub const  WINDOW_HEIGHT: f32 = 180.0;

pub const NUMBER_OF_CELLS_X: i32 = 3;
pub const NUMBER_OF_CELLS_Y: i32 = 6;

// here divide by a number to create the number of grids
pub const CELL_WIDTH: f32 = WINDOW_WIDTH / NUMBER_OF_CELLS_X as f32;
pub const CELL_HEIGHT: f32 = WINDOW_HEIGHT / NUMBER_OF_CELLS_Y as f32;

fn main() {
    // Initialize your game engine here
    // For example, create a game instance and start the game loop
    let mut game = game::Game::new();
    game.run_game_loop();
}

// main.rs
use ggez::{conf, event, Context, ContextBuilder, GameResult};
use ggez::event::KeyCode;

mod engine;

use engine::game::Game;

struct MainState {
    game: Game,
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.update(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.game.draw(ctx)?;
        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, keycode: KeyCode, keymods: ggez::input::keyboard::KeyMods, repeat: bool) {
        self.game.handle_key_down(ctx, keycode, keymods, repeat);
    }
}

fn main() -> GameResult {
    let (ctx, event_loop) = &mut ContextBuilder::new("subway_surfers_clone", "your_username")
        .window_setup(conf::WindowSetup::default().title("Subway Surfers Clone"))
        .window_mode(conf::WindowMode::default().dimensions(800.0, 600.0))
        .build()?;

    let state = &mut MainState {
        game: Game::new(),
    };

    event::run(ctx, event_loop, state)
}
