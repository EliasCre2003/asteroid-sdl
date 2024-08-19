use asteroid_sdl::{
    game::{
        graphics::Drawable,
        objects::{polygons, Asteroid},
        physics::{self, PhysicsSpace},
    },
    sdl_setup,
};
use rapier2d_f64::na::{Point2, Vector2};
use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::WindowCanvas,
};

fn to_points(polygon: &Vec<Point2<f64>>) -> Vec<Point> {
    let mut points = Vec::new();
    for vert in polygon {
        points.push(Point::new(vert.x as i32, vert.y as i32));
    }
    points
}

fn draw1(
    canvas: &mut WindowCanvas,
    polygon: &Vec<Point2<f64>>,
    indices: &Vec<[usize; 3]>,
    position: Point,
) {
    canvas.set_scale(1.01, 1.01).unwrap();
    let polygon = to_points(polygon);

    const COLORS: [Color; 3] = [Color::GREEN, Color::RED, Color::BLUE];
    for (i, triangle) in indices.iter().enumerate() {
        canvas.set_draw_color(COLORS[i % 3]);
        for j in 0..3 {
            canvas
                .draw_line(
                    *polygon.get(triangle[j] as usize).unwrap() + position,
                    *polygon.get(triangle[(j + 1) % 3] as usize).unwrap() + position,
                )
                .unwrap();
        }
    }
}

fn draw2(canvas: &mut WindowCanvas, polygon: &Vec<Point2<f64>>, position: Point) {
    let polygon = to_points(polygon);
    canvas.set_draw_color(Color::WHITE);
    for i in 0..polygon.len() {
        canvas
            .draw_line(
                *polygon.get(i).unwrap(),
                *polygon.get((i + 1) % polygon.len()).unwrap(),
            )
            .unwrap();
    }
}

fn main() {
    let (sdl_context, video_subsystem, mut window_canvas, mut event_pump) =
        sdl_setup::create_window(1920, 1080);

    let polygon = polygons::random(8..15, (400.0, 800.0));

    draw2(&mut window_canvas, &polygon, Point::new(0, 0));
    window_canvas.present();

    let indices = polygons::triangulate_polygon(polygon.clone());

    let mut draw_method1 = true;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::SPACE),
                    ..
                } => draw_method1 = !draw_method1,
                _ => (),
            }
        }

        window_canvas.set_draw_color(Color::BLACK);
        window_canvas.clear();
        if draw_method1 {
            draw1(&mut window_canvas, &polygon, &indices, Point::new(0, 0));
        } else {
            draw2(&mut window_canvas, &polygon, Point::new(0, 0))
        }
        // window_canvas.fill_rect(Rect::new(0, 0, 200, 200)).unwrap();
        window_canvas.present();
    }
}
