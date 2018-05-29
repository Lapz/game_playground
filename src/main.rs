extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use rand::prelude::*;

struct Ball {
    pos: graphics::Vector2,
    vel: graphics::Vector2,
}

impl Ball {
    pub fn new() -> Self {
        Ball {
            pos: graphics::Vector2::new(100.0, 100.0),
            vel: graphics::Vector2::new(2.0, 5.0),
        }
    }

    fn bounce(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.pos += self.vel;
        if self.pos.x > 500.0 || self.pos.x < 0.0 {
            self.vel.x = self.vel.x * -1.0;
        }
        if self.pos.y > 500.0 || self.pos.y < 0.0 {
            self.vel.y = self.vel.y * -1.0;
        }
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::ellipse(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Point2::new(self.pos.x, self.pos.y),
            16.0,
            16.0,
            1.0,
        )?;
        Ok(())
    }
}

struct MainState {
    ball: Ball,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState { ball: Ball::new() })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ball.bounce(ctx)?;
        // self.walker.walk(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ball.draw(ctx)?;
        graphics::present(ctx);
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();

    let ctx = &mut Context::load_from_conf("Walker", "Lapz", c).unwrap();

    let state = &mut MainState::new(ctx).unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }

    println!("Hello, world!");
}
