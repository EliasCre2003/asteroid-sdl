use rapier2d_f64::prelude::{ConvexPolygon, RigidBody};
use sdl2::pixels::Color;
use sdl2::{rect::Point, render::WindowCanvas};

use super::objects::Asteroid;
use super::physics::{ColumnVector, PhysicsSpace};

pub trait Drawable {
    fn draw(&mut self, canvas: &mut WindowCanvas, space: &mut PhysicsSpace) {}
}


impl Drawable for Asteroid  {
    fn draw(&mut self, canvas: &mut WindowCanvas, space: &mut PhysicsSpace) {
        
        let body =  space.get_rigid_body(self.get_rigid_body_handle()).unwrap();
        let position: ColumnVector = body
            .position().
            translation.
            vector;
        let shape_points = self.get_shape().points();
        let mut draw_points: Vec<Point> = Vec::with_capacity(shape_points.len());
        let angle = body.rotation().angle();
        for shape_point in shape_points {
            draw_points.push(Point::new(
                (position.x + shape_point.x * angle.cos() - shape_point.y * angle.sin()) as i32,
                (position.y + shape_point.x * angle.sin() + shape_point.y * angle.cos()) as i32,
            ));
        }
        canvas.set_scale(1.001, 1.001).unwrap();
        canvas.set_draw_color(Color::WHITE);
        canvas.draw_lines(draw_points.as_slice()).unwrap();
        canvas.draw_line(
            *draw_points.get(0).unwrap(), 
            *draw_points.get(draw_points.len()-1).unwrap()
        )
        .unwrap();



    }
}
