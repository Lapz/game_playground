extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use std::env;
use std::path;

use rand::prelude::*;

struct Walker {
    x: f32,
    y: f32,
}

struct Ball {
    pos:graphics::Vector2,
    vel:graphics::Vector2
}


impl Ball {
    pub fn new() -> Self {
        Ball {
            pos:graphics::Vector2::new(100.0,100.0),
            vel:graphics::Vector2::new(2.0,5.0),
        }
    }

    fn bounce(&mut self,ctx:&mut Context) -> GameResult<()> {
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
        graphics::ellipse(ctx, graphics::DrawMode::Fill, graphics::Point2::new(self.pos.x,self.pos.y),16.0,16.0,1.0)?;
        Ok(())
    }
}

impl Walker {
    pub fn new() -> Self {
        Walker { x: 12.0, y: 12.0 }
    }

    fn display(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, [1.0, 0.0, 0.0, 1.0].into())?;
        graphics::rectangle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Rect::new(self.x, self.y, 1.0, 1.0),
        )?;

        Ok(())
    }

    fn walk(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut rng = thread_rng();
        let choice: u16 = rng.gen_range(0, 4);

        if choice == 0 {
            self.x += 1.0;
        } else if choice == 1 {
            self.x -= 1.0;
        } else if choice == 2 {
            self.y += 1.0;
        } else {
            self.y -= 1.0;
        }

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.walk(ctx)?;
        self.display(ctx)?;
        Ok(())
    }
}

struct MainState {
    ball: Ball,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            ball: Ball::new(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.ball.bounce(ctx)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // self.walker.draw(ctx)?;
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
