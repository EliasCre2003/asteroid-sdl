use rapier2d_f64::prelude::*;

use super::physics::{ColumnVector, PhysicsSpace};

pub struct Asteroid {
    shape: ConvexPolygon,
    body_handle: RigidBodyHandle,
}

impl Asteroid {
    pub fn new(position: ColumnVector, shape: ConvexPolygon, space: &mut PhysicsSpace) -> Self {
        let rigid_body = RigidBodyBuilder::dynamic().translation(position).build();
        let collider = ColliderBuilder::convex_polyline(shape.points().to_vec())
            .unwrap()
            .build();
        let body_handle = space.add_rigid_body(rigid_body, collider);
        Self { shape, body_handle }
    }

    pub fn get_rigid_body_handle(&self) -> RigidBodyHandle {
        self.body_handle
    }

    pub fn get_shape(&self) -> &ConvexPolygon {
        &self.shape
    }
}

pub mod polygons {
    use std::{f64::consts::PI, ops::Range};
    use rapier2d_f64::{na::{Point2, Vector2}, prelude::ConvexPolygon};
    use rand::Rng;
    use itertools::Itertools;

    pub fn equilateral(num_sides: u32, side_length: u32) -> ConvexPolygon {
        let delta_angle = 2.0 * PI / num_sides as f64;
        let hypotenuse = delta_angle.sin() / (side_length as f64 / 2.0);
        let mut points = Vec::with_capacity(num_sides as usize);
        let mut prev_point: Point2<f64> = Point2::new(0.0, 0.0);
        for i in 0..num_sides {
            let point = Point2::new(
                prev_point.x + (delta_angle * i as f64).cos() * side_length as f64,
                prev_point.y + (delta_angle * i as f64).sin() * side_length as f64 - hypotenuse,
            );
            prev_point = point.clone();
            points.push(point);
        }
        ConvexPolygon::from_convex_polyline(points).unwrap()
    }

    pub fn randomized_convex(num_sides_range: Range<u32>, bounding_box: Vector2<f64>) -> ConvexPolygon {
        let mut rng = rand::thread_rng();
        let num_sides = rng.gen_range(num_sides_range);
        // let mut points: Vec<Point2<f64>> = Vec::from_iter((0..num_sides).map(
        //     |_| {
        //         Point2::new(
        //             rng.gen::<f64>() * bounding_box.x - bounding_box.x / 2.0,
        //             rng.gen::<f64>() * bounding_box.y - bounding_box.y / 2.0
        //         )
        //     }
        // ));
        // points.sort_by(|a, b|
        //     a.y.atan2(a.x).partial_cmp(&b.y.atan2(b.x)).unwrap()
        // );
        // let points: [Vec<f64>; 2] = [
            // Vec::from_iter((0..num_sides).map(
            //     |_| rng.gen::<f64>() * bounding_box.x - bounding_box.x / 2.0
            // )),
        //     Vec::from_iter((0..num_sides).map(
        //         |_| rng.gen::<f64>() * bounding_box.y - bounding_box.y / 2.0
        //     ))
        // ];
        // let points: [Vec<f64>; 2] = {
        //     let mut points: [Vec<f64>; 2];
        //     for (i, bound) in [bounding_box.x, bounding_box.y].iter().enumerate() {
        //         points[i] = Vec::from_iter((0..num_sides).map(
        //             |_| rng.gen::<f64>() * bound - bound / 2.0
        //         )
        //         .sorted_by(
        //             |a, b| a.partial_cmp(&b).unwrap()
        //         ));
        //     }
        //     points
        // geo_rand::GeoRand::rand(rng, geo_rand_parameters)
        // };



        // ConvexPolygon::from_convex_polyline(points).unwrap()
    }
}
