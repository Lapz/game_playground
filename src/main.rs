extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, GameResult};
use std::cell::RefCell;


const WIDTH:f32 = 400.0;
const HEIGHT:f32 = 300.0;

use rand::prelude::*;

thread_local!(static GENERATOR: RefCell<ThreadRng>= RefCell::new(thread_rng()));

struct Star {
    pos: na::Vector3<f32>,
    pz: f32,
}

impl Star {
    pub fn new() -> Self {
        let z = GENERATOR.with(|cell| cell.borrow_mut().gen_range(0.0, WIDTH));
        

        Star {
            pos: na::Vector3::new(
                GENERATOR.with(|cell| cell.borrow_mut().gen_range(-WIDTH, WIDTH)),
                GENERATOR.with(|cell| cell.borrow_mut().gen_range(-HEIGHT,HEIGHT)),
                z,
            ),
            pz:z
        }
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        // // self.pos.x += 3.0;
        self.pos.y -= 0.1;
          self.pos.x -= 0.4;
        // self.pos.y -= 4.0;
       
        self.pos.z -= 5.0;


        if self.pos.z < 1.0 {
            self.pos.x = GENERATOR.with(|cell| cell.borrow_mut().gen_range(-WIDTH,WIDTH));
            self.pos.y = GENERATOR.with(|cell| cell.borrow_mut().gen_range(-HEIGHT,HEIGHT));
            self.pos.z += WIDTH;
            self.pz = self.pos.z;
        }
      
        Ok(())
    }

    fn draw(&self, ctx: &mut Context) -> GameResult<()> {

       
        let sx = map(self.pos.x/self.pos.z, 0.0, 1.0, 0.0, WIDTH);
        let sy = map(self.pos.y/self.pos.z, 0.0, 1.0, 0.0, HEIGHT);
        let r = map(self.pos.z, 0.0, WIDTH, 6.0 ,0.0);
        
        graphics::circle(ctx, graphics::DrawMode::Fill, graphics::Point2::new(sx,sy), r, 1.0)?;

        let px =  map(self.pos.x/self.pz, 0.0, 1.0, 0.0, WIDTH);
        let py = map(self.pos.y/self.pz, 0.0, 1.0, 0.0, HEIGHT);

        // graphics::line(ctx, &[graphics::Point2::new(sx,sy),graphics::Point2::new(px,py)], 1.0)?;
        
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
            stars.push(Star::new());
        }
        Ok(MainState { stars })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
          

       
        graphics::set_background_color(ctx, graphics::Color::from_rgb(0, 0, 0));

        for star in self.stars.iter_mut() {
            star.update(ctx)?;
            star.draw(ctx)?;
            
        }

        graphics::present(ctx);
        
        graphics::clear(ctx);
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
