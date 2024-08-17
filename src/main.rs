use std::{
    collections::HashMap, f64::consts::PI, time::{Duration, Instant}
};

use asteroid_sdl::sdl_setup;
use rapier2d_f64::prelude::*;
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::CanvasBuilder,
    surface::{self, Surface},
};

fn main() {
    let (sdl_context, video_subsystem, mut window_canvas, mut event_pump) =
        sdl_setup::create_window(1920, 1080);

    // window_canvas.set_logical_size(32, 16).unwrap();

    let mut rigid_body_set = RigidBodySet::new();
    let mut collider_set = ColliderSet::new();

    /* Create the ground. */
    let collider = ColliderBuilder::cuboid(2000.0, 100.0)
        .position(Isometry::new(vector![0.0, 100.0], PI / 4.0))
        .build();
    collider_set.insert(collider);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![500.0, 0.0])
        .build();
    let collider = ColliderBuilder::cuboid(50.0, 50.0).restitution(1.0).build();
    let cube_body_handle = rigid_body_set.insert(rigid_body);
    collider_set.insert_with_parent(collider, cube_body_handle, &mut rigid_body_set);

    /* Create other structures necessary for the simulation. */
    let gravity: nalgebra::Matrix<
        f64,
        nalgebra::Const<2>,
        nalgebra::Const<1>,
        nalgebra::ArrayStorage<f64, 2, 1>,
    > = vector![0.0, 200.0];
    let mut integration_parameters = IntegrationParameters::default();
    let mut physics_pipeline = PhysicsPipeline::new();
    let mut island_manager = IslandManager::new();
    let mut broad_phase = DefaultBroadPhase::new();
    let mut narrow_phase = NarrowPhase::new();
    let mut impulse_joint_set = ImpulseJointSet::new();
    let mut multibody_joint_set = MultibodyJointSet::new();
    let mut ccd_solver = CCDSolver::new();
    let mut query_pipeline = QueryPipeline::new();
    let physics_hooks = ();
    let event_handler = ();

    let mut block_canvas = Surface::new(50, 50, sdl2::pixels::PixelFormatEnum::RGB24)
        .unwrap()
        .into_canvas()
        .unwrap();

    block_canvas.set_draw_color(Color::WHITE);
    block_canvas.clear();

    let block_surface = block_canvas.into_surface();

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

        integration_parameters.dt = delta_time;

        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut query_pipeline),
            &physics_hooks,
            &event_handler,
        );

        let cube_body = &rigid_body_set[cube_body_handle];

        let x = cube_body.center_of_mass().x as i32;
        let y = cube_body.center_of_mass().y as i32;

        let cube_rect = Rect::from_center(sdl2::rect::Point::new(x, y), 50, 50);

        window_canvas.set_draw_color(Color::BLACK);
        window_canvas.clear();

        window_canvas.set_draw_color(Color::WHITE);

        let binding = window_canvas.texture_creator();
        let texture = block_surface.as_texture(&binding).unwrap();
        let angle = cube_body.rotation().angle();
        window_canvas
            .copy_ex(
                &texture,
                Rect::new(0, 0, 50, 50),
                cube_rect,
                angle.to_degrees(),
                Point::new(25, 25),
                false,
                false,
            )
            .unwrap();

        window_canvas.present();

        println!("{delta_time}");
        duration = start.elapsed();

    }
}
