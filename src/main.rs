extern crate ggez;
extern crate rand;
extern crate noise;
use ggez::conf;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};

use rand::distributions::StandardNormal;
use rand::prelude::*;
use std::cell::RefCell;
use noise::{NoiseFn, Perlin};


thread_local!(static GENERATOR: RefCell<ThreadRng>= RefCell::new(thread_rng()));


struct MainState {
    t:f64,
    perlin:Perlin
}

impl MainState {
    fn new() -> GameResult<MainState> {
        Ok(MainState {t: 0.0,perlin:Perlin::new()})
    }
}


fn map(value:f32,istart:f32,istop:f32,ostart:f32,ostop:f32) -> f32 {
    ostart + (ostop - ostart) * ((value - istart) / (istop - istart))
}


impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
       
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
      
        
        let n = self.perlin.get([self.t,random(),0.0]);
        println!("n:{}",n);

        let x = map(n as f32, 0.0, 1.0, 0.0, 200.0);

        println!("x:{}",x );


    


      
     
        graphics::set_color(ctx, [0.0;4].into())?;
        graphics::rectangle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Rect::new(0.0, (self.t*100.0) as f32, n as f32, 4.0),
        )?;
        graphics::present(ctx);
          self.t += 0.1;
        Ok(())
    }
}

fn main() {
    let c = conf::Conf::new();

    let ctx = &mut Context::load_from_conf("Splat", "Lapz", c).unwrap();

    graphics::set_background_color(ctx, [1.0;4].into());

   
    let state = &mut MainState::new().unwrap();

    if let Err(e) = event::run(ctx, state) {
        println!("Error encountered: {}", e);
    } else {
        println!("Game exited cleanly.");
    }

    println!("Hello, world!");
}
