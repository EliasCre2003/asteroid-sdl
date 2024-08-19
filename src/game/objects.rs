use polygons::triangulate_polygon;
use rapier2d_f64::prelude::*;

use super::physics::{ColumnVector, PhysicsSpace};

pub struct Asteroid {
    shape: Vec<Point<Real>>,
    body_handle: RigidBodyHandle,
}

impl Asteroid {
    pub fn new(position: ColumnVector, shape: Vec<Point<Real>>, space: &mut PhysicsSpace) -> Self {
        let rigid_body = RigidBodyBuilder::dynamic()
            .translation(position)
            .build();
        let indices = triangulate_polygon(&shape);
        let indices = indices.as_slice();
        let shapes = Vec::from_iter((0..indices.len() / 3)
            .map(|i| {(
                    Isometry::translation(0.0, 0.0),
                    SharedShape::convex_polyline(vec![
                        *shape.get(indices[3*i+0] as usize).unwrap(),
                        *shape.get(indices[3*i+1] as usize).unwrap(),
                        *shape.get(indices[3*i+2] as usize).unwrap(),
                    ]).unwrap()
                )
            }
        ));
        let collider = ColliderBuilder::compound(shapes).build();
        let body_handle: RigidBodyHandle = space.add_rigid_body(rigid_body, collider);
        Self { shape, body_handle }
    }

    pub fn get_rigid_body_handle(&self) -> RigidBodyHandle {
        self.body_handle
    }

    pub fn get_shape(&self) -> &Vec<Point<Real>> {
        &self.shape
    }
}


pub struct SpaceShip {
    
}


pub mod polygons {
    use itertools::Itertools;
    use rand::Rng;
    use rapier2d_f64::{
        na::{DimSum, Point2, Vector2},
        parry::query::point,
        prelude::{ColliderBuilder, ConvexPolygon},
    };
    use std::{f64::consts::PI, ops::Range};
    use triangulate::{formats, Polygon};
    use triangulate::{ListFormat, PolygonList};

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

    pub fn random(num_sides_range: Range<u32>, bounding_box: (f64, f64)) -> Vec<Point2<f64>> {
        let mut rng = rand::thread_rng();
        let num_sides = rng.gen_range(num_sides_range);
        let mut points: Vec<Point2<f64>> = Vec::from_iter((0..num_sides).map(|_| {
            let mut coord = || {
                rng.gen::<f64>() * (bounding_box.1 - bounding_box.0) + bounding_box.0
                    - bounding_box.1 / 2.0
            };
            Point2::new(coord(), coord())
        }));
        let center_point = {
            let mut center_point: Point2<f64> = Point2::new(0.0, 0.0);
            for point in &points {
                center_point.x += point.x;
                center_point.y += point.y;
            }
            center_point / num_sides as f64;
        };
        points.sort_by(|a, b| {
            let p1 = a - center_point;
            let p2 = b - center_point;
            let angle1 = p1.y.atan2(p1.x);
            let angle2 = p2.y.atan2(p2.x);
            angle1.partial_cmp(&angle2).unwrap()
        });
        points
    }

    fn is_concave(p0: &Point2<f64>, p1: &Point2<f64>, p2: &Point2<f64>) -> bool {
        let v1 = p0 - p1;
        let v2 = p2 - p1;
        v1.x * v2.y - v1.y * v2.x < 0.0
    }

    fn is_convex(p0: &Point2<f64>, p1: &Point2<f64>, p2: &Point2<f64>) -> bool {
        !is_concave(p0, p1, p2)
    }

    pub fn triangulate_polygon(points: &Vec<Point2<f64>>) -> Vec<u32> {
        let formated_points = vec![Vec::from_iter(
            points.iter().map(|point| [point.x, point.y]),
        )];
        let mut triangulated_indices = Vec::<[usize; 2]>::new();
        formated_points
            .triangulate(
                formats::IndexedListFormat::new(&mut triangulated_indices)
                .into_fan_format(),
            )
            .unwrap();
        Vec::from_iter(triangulated_indices.iter().map(
            |pair| pair[1] as u32
        ))


        
    }
}
