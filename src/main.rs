extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::cell::RefCell;

use rand::prelude::*;

thread_local!(static GENERATOR: RefCell<ThreadRng>= RefCell::new(thread_rng()));

struct Star {
    pos: na::Vector3<f32>,
}

impl Star {
    pub fn new(ctx: &mut Context) -> Self {
        let (width, height) = graphics::get_window(ctx).size();

        Star {
            pos: na::Vector3::new(
                GENERATOR.with(|cell| cell.borrow_mut().gen_range(-(width as f32), width as f32)),
                GENERATOR.with(|cell| cell.borrow_mut().gen_range(-(width as f32), height as f32)),
                width as f32,
            ),
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.pos.x += 3.0;
        self.pos.y += 3.0;
        self.pos.z += self.pos.z / 2.0;
        // self.walker.walk(ctx)?;
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::ellipse(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Point2::new(self.pos.x, self.pos.y),
            2.0,
            2.0,
            1.0,
        )?;
        Ok(())
    }
}

struct MainState {
    stars: Vec<Star>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut stars = Vec::with_capacity(400);

        for _ in 0..400 {
            stars.push(Star::new(ctx));
        }
        Ok(MainState { stars })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for star in self.stars.iter_mut() {
            star.update(ctx)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
          graphics::clear(ctx);
        let (width, height) = graphics::get_window(ctx).size();
        let transform = graphics::Matrix4::new_rotation_wrt_point(
            na::Vector3::new(width as f32, height as f32, width as f32),
            na::Point3::new(
                (width as f32) / 2.0,
                (height as f32) / 2.0,
                (width as f32) / 2.0,
            ),
        );
        graphics::transform(ctx, transform);
        graphics::apply_transformations(ctx)?;
        graphics::set_background_color(ctx, graphics::Color::from_rgb(0, 0, 0));

        for star in self.stars.iter() {
            star.draw(ctx)?;
        }

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

fn map(value: f32, istart: f32, istop: f32, ostart: f32, ostop: f32) -> f32 {
    ostart + (ostop - ostart) * ((value - istart) / (istop - istart))
}
