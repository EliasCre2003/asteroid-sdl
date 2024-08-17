use rapier2d_f64::prelude::RigidBody;
use sdl2::render::WindowCanvas;

pub trait Drawable {
    fn draw(&mut self, canvas: &mut WindowCanvas) {}
}


impl Drawable for RigidBody  {
    fn draw(&mut self, canvas: &mut WindowCanvas) {
        
    }
}