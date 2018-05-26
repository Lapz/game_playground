extern crate ggez;
extern crate rand;
extern crate perlin_noise as perlin;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use perlin::PerlinNoise;
use rand::distributions::StandardNormal;
use rand::prelude::*;
use std::cell::RefCell;


struct Walker {
    x: f32,
    y: f32,
    tx:f64,
    ty:f64
}

impl Walker {
    pub fn new() -> Self {
        Walker { x: 250.0, y: 250.0,tx:0.0,ty:10000.0 }
    }

    fn display(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_color(ctx, graphics::Color::from_rgb(0,0,0))?;
        graphics::circle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Point2::new(self.x, self.y),
            3.0,
            1.0,
        )?;
        Ok(())
    }

    fn walk(&mut self, ctx: &mut Context,perlin:&PerlinNoise) -> GameResult<()> {
        let x_step_size = map(perlin.get(self.tx), 0.0,1.0,-12.0,12.0);

        let y_step_size = map(perlin.get(self.ty),0.0,1.0,-1.0,1.0);

        self.x += x_step_size as f32;
        self.y += y_step_size as f32;

        self.tx += 0.0001;
        self.ty += 1.0;

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context,perlin:&PerlinNoise) -> GameResult<()> {
        self.walk(ctx,perlin)?;
        self.display(ctx)?;
        Ok(())
    }
}

struct MainState {
    t:f64,
    walker:Walker,
    perlin: PerlinNoise,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState {
            t: 0.0,
            perlin: PerlinNoise::new(),
            walker:Walker::new(),
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

        self.walker.draw(ctx,&self.perlin)?;

        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();

    let ctx = &mut Context::load_from_conf("Splat", "Lapz", c).unwrap();

    graphics::set_background_color(ctx, graphics::Color::from_rgb(255,255,255));

    let state = &mut MainState::new().unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }

    println!("Hello, world!");
}
