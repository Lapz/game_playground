extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::{Context,GameResult};
use ggez::graphics;
use std::env;
use std::path;


use rand::prelude::*;


struct Walker {
    x:f32,
    y:f32,
}

impl Walker {
    pub fn new() -> Self {
        Walker {
            x:12.0,
            y:12.0
        }
    }

    fn display(&self,ctx:&mut Context) -> GameResult<()> {
        graphics::set_color(ctx, [1.0, 0.0, 0.0, 1.0].into())?;
        graphics::rectangle(ctx, graphics::DrawMode::Fill, graphics::Rect::new(self.x, self.y, 1.0, 1.0))?;
        
        Ok(())
    }

    fn walk(&mut self,ctx:&mut Context) -> GameResult<()> {
        let mut rng = thread_rng();
        let choice:u16 = rng.gen_range(0,4);

        if choice == 0 {
            self.x += 1.0;
        }else if choice == 1 {
            self.x -= 1.0;
        }else if choice == 2 {
            self.y += 1.0;
        }else {
            self.y -= 1.0;
        }

        Ok(())
    }
    fn draw(&mut self,ctx:&mut Context)   -> GameResult<()> {
        self.walk(ctx)?;
        self.display(ctx)?;
        Ok(())
    }
}

struct MainState {
    walker:Walker
}


impl MainState {
    fn new(ctx:&mut Context) -> GameResult<MainState> {
        Ok(MainState {
            walker:Walker::new()
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self,ctx:&mut Context) -> GameResult<()> {
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
