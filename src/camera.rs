use cgmath::{Vector3, InnerSpace, Quaternion};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct Camera {
    pub pos: Vector3<f32>,
    pub dir: Vector3<f32>,
    pub up: Vector3<f32>,
    pub fov: f32,
    pub rot: Quaternion<f32>,
    pub width: f32,
    pub height: f32,
    pub aspect: f32
}

impl Camera {
    pub fn new(pos: Vector3<f32>, dir: Vector3<f32>, width: f32, height: f32) -> Camera {
        let up = Vector3 { x: 0.0, y: 1.0, z: 0.0 };
        let aspect = width / height;
        let rot = Quaternion::from_arc(
            Vector3::new(0.0, 0.0, -1.0),
            dir,
            None
        );

        Camera {
            pos,
            dir,
            up,
            rot,
            fov: 90.0,
            width,
            height,
            aspect
        }
    }

    pub fn get_ray(self, x: f32, y: f32) -> Ray {
        let fov_adjustment = (self.fov.to_radians() / 2.0).tan();
        let aspect_ratio = self.aspect;
        let sensor_x = ((((x + 0.5) / self.width) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y + 0.5) / self.height) * 2.0) * fov_adjustment;
        let ray_dir = self.rot * Vector3::new(
            sensor_x,
            sensor_y,
            -1.0
        );

        Ray {
            origin: self.pos.clone(),
            direction: (self.dir + ray_dir).normalize()
        }
    }
}
