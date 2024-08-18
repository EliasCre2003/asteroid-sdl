use std::{
    collections::HashMap,
    f64::consts::PI,
    time::{Duration, Instant},
};

use asteroid_sdl::{
    game::{
        graphics::Drawable,
        objects::{polygons, Asteroid},
        physics::{self, PhysicsSpace},
    },
    sdl_setup,
};
use nalgebra::{Point2, Vector2};
use rapier2d_f64::math;
use rapier2d_f64::prelude::*;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect,
    rect::Rect,
    render::CanvasBuilder,
    surface::{self, Surface},
};

fn main() {
    let (sdl_context, video_subsystem, mut window_canvas, mut event_pump) =
        sdl_setup::create_window(1920, 1080);

    window_canvas.set_logical_size(192, 108).unwrap();

    let mut physics_space = PhysicsSpace::new();
    physics_space.set_gravity(vector![0.0, 200.0]);

    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(2000.0, 100.0)
        .position(Isometry::new(vector![0.0, 100.0], PI / 4.0))
        .build();
    physics_space.add_collider(collider);

    /* Create an Asteroid */
    let mut asteroids = Vec::new();

    for i in 0..2 {
        asteroids.push(Asteroid::new(
            vector![400.0 + 50.0 * i as f64, 200.0 - 200.0 * i as f64],
            // polygons::equilateral(8, 60),
            polygons::randomized_convex(4..7, Vector2::new(50.0, 150.0)),
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

        for asteroid in &mut asteroids {
            asteroid.draw(&mut window_canvas, &mut physics_space);
        }

        window_canvas.present();

        println!("{}", 1.0 / delta_time);
        duration = start.elapsed();
    }
}
