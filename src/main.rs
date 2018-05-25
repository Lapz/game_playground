extern crate ggez;
extern crate rand;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use rand::prelude::*;
use std::cell::RefCell;

thread_local!(static GENERATOR: RefCell<ThreadRng>= RefCell::new(thread_rng()));

fn monte_carlo() -> f32 {
    loop {
        let r1 = GENERATOR.with(|cell| cell.borrow_mut().gen_range(0.0, 1.0));
        let probability = r1;
        let r2 = GENERATOR.with(|cell| cell.borrow_mut().gen_range(0.0, 1.0));

        if r2 < probability {
            return r1;
        }
    }
}

struct Walker {
    x: f32,
    y: f32,
}

impl Walker {
    pub fn new() -> Self {
        Walker { x: 200.0, y: 200.0 }
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

    fn walk(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let step_size = monte_carlo() * 10.0;

        let x_step_size = GENERATOR.with(|cell| cell.borrow_mut().gen_range(-step_size, step_size));

        let y_step_size = GENERATOR.with(|cell| cell.borrow_mut().gen_range(-step_size, step_size));

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
    fn new() -> GameResult<MainState> {
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

    let state = &mut MainState::new().unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }

    println!("Hello, world!");
}
