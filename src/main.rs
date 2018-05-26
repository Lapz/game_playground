extern crate ggez;
extern crate rand;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use perlin::PerlinNoise;
use rand::distributions::StandardNormal;
use rand::prelude::*;
use std::cell::RefCell;

mod perlin;

struct MainState {
    t: f64,
    perlin: PerlinNoise,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState {
            t: 0.0,
            perlin: PerlinNoise::new(),
        })
    }
}

fn map(value: f64, istart: f64, istop: f64, ostart: f64, ostop: f64) -> f64 {
    ostart + (ostop - ostart) * ((value - istart) / (istop - istart))
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        // graphics::clear(ctx);
         
        self.t += 0.01;

        let n = self.perlin.get(self.t);

        let x = map(n, 0.0, 1.0, 0.0, 100.0);

    

        graphics::rectangle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Rect::new(0.0, (self.t * 100.0) as f32, x as f32, 50.0),
        )?;
        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();

    let ctx = &mut Context::load_from_conf("Splat", "Lapz", c).unwrap();

   graphics::set_background_color(ctx, [1.0; 4].into());

    let state = &mut MainState::new().unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }

    println!("Hello, world!");
}
