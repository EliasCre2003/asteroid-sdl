use std::{
    f64::consts::PI,
    time::{Duration, Instant},
};

use asteroid_sdl::{
    game::{
        graphics::Drawable,
        objects::{polygons, Asteroid},
        physics::PhysicsSpace,
    },
    sdl_setup,
};
use rapier2d_f64::prelude::*;
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, rect::Rect, render::Canvas, surface::{self, Surface}
};

fn main() {
    let (sdl_context, video_subsystem, mut window_canvas, mut event_pump) =
        sdl_setup::create_window(1920, 1080);

    let mut physics_space = PhysicsSpace::new();
    physics_space.set_gravity(vector![0.0, 20.0]);

    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(200.0, 10.0)
        .position(Isometry::new(vector![0.0, 10.0], PI / 4.0))
        .build();
    physics_space.add_collider(collider);

    /* Create an Asteroid */
    let mut asteroids = Vec::new();

    for i in 0..2 {
        asteroids.push(Asteroid::new(
            vector![40.0 + 5.0 * i as f64, 20.0 - 20.0 * i as f64],
            // polygons::equilateral(8, 60),
            polygons::random(8..15, (10.0, 25.0)),
            &mut physics_space,
        ));
    }
    let mut duration: Duration = Duration::new(0, 0);

    /* Run the game loop, stepping the simulation once per frame. */
    'running: loop {
        let start = Instant::now();
        let delta_time = duration.as_secs_f64();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => (),
            }
        }

        physics_space.step(delta_time);

        window_canvas.set_draw_color(Color::BLACK);
        window_canvas.clear();

        let mut main_canvas = Canvas::from_surface(
            Surface::new(
                192, 
                108, 
                sdl2::pixels::PixelFormatEnum::RGB24
            )
            .unwrap()
        )
        .unwrap();

        for asteroid in &mut asteroids {
            asteroid.draw(&mut main_canvas, &mut physics_space);
        }

        let binding = window_canvas.texture_creator();
        let texture =  main_canvas.into_surface()
            .as_texture(&binding)
            .unwrap();
        window_canvas.copy_ex(
            &texture, 
            Rect::new(0, 0, 192, 108), 
            Rect::new(0, 0, 1920, 1080), 
            0.0, 
            sdl2::rect::Point::new(86, 53), 
            false, 
            false
        )
        .unwrap();
        window_canvas.present();

        println!("{}", 1.0 / delta_time);
        duration = start.elapsed();
    }
}
