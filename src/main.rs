extern crate ggez;
extern crate rand;
#[macro_use]
extern crate lazy_static;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use rand::distributions::StandardNormal;
use rand::FromEntropy;
use std::env;
use std::path;

use rand::prelude::*;
use std::cell::RefCell;

thread_local!(static GENERATOR: RefCell<ThreadRng>= RefCell::new(thread_rng()));
const STANDARD_DEVIATION: f64 = 2.0;
const MEAN: f64 = 0.0;

struct Walker {
    x: f32,
    y: f32,
}

impl Walker {
    pub fn new() -> Self {
        Walker { x: 50.0, y: 50.0 }
    }

    fn display(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, [1.0, 0.0, 0.0, 1.0].into())?;
        graphics::circle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Point2::new(self.x, self.y),
            3.0,
            1.0,
        )?;
        Ok(())
    }

    fn walk(&mut self, ctx: &mut Context) -> GameResult<()> {
        let x_step_size = GENERATOR.with(|rng| {
            (rng.borrow_mut().sample(StandardNormal) * STANDARD_DEVIATION + MEAN) as f32
        });

        let y_step_size = GENERATOR.with(|rng| {
            (rng.borrow_mut().sample(StandardNormal) * STANDARD_DEVIATION + MEAN) as f32
        });

        self.x += x_step_size;
        self.y += y_step_size;
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.walk(ctx)?;
        self.display(ctx)?;
        Ok(())
    }
}

struct MainState {
    walker: Walker,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            walker: Walker::new(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.walker.draw(ctx)?;
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
