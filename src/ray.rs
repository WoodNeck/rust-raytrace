use cgmath::{Vector3, InnerSpace, VectorSpace};

use crate::{camera::Camera, raycaster::Raycaster, scene::Scene};

const MAX_RECURSION: u32 = 100;

pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, camera: &Camera, caster: &Raycaster) -> Ray {
        let fov_adjustment = (camera.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (caster.width as f32) / (caster.height as f32);
        let sensor_x = ((((x as f32 + 0.5) / caster.width as f32) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f32 + 0.5) / caster.height as f32) * 2.0) * fov_adjustment;

        Ray {
            origin: camera.pos,
            direction: Vector3::new(
                sensor_x,
                sensor_y,
                -1.0
            ).normalize()
        }
    }

    pub fn color(&mut self, scene: &Scene) -> Vector3<f32> {
        let mut col = Vector3::new(1.0, 1.0, 1.0);

        let mut hr = Record {
            t: 0.0,
            p: Vector3 { x: 0.0, y: 0.0, z: 0.0 },
            n: Vector3 { x: 0.0, y: 0.0, z: 0.0 }
        };

        for _ in 0..MAX_RECURSION {
            let mat = scene.traverse(self, 0.001, f32::MAX, &mut hr);

            if mat.is_some() {
                let mat = mat.unwrap();
                let ret = mat.surface.scat(self, &mut hr);
                if ret {
                    col.x *= mat.albedo.x;
                    col.y *= mat.albedo.y;
                    col.z *= mat.albedo.z;
                } else {
                    return Vector3::new(0.0, 0.0, 0.0);
                }
            } else {
                let t = 0.5 * self.direction.y + 0.5;
                let a = Vector3::new(1.0, 1.0, 1.0);
                let b = Vector3::new(0.5, 0.7, 1.0);
                let lerped = a.lerp(b, t);

                col.x *= lerped.x;
                col.y *= lerped.y;
                col.z *= lerped.z;

                return col;
            }
        }

        col
    }
}

pub struct Record {
    pub t: f32,
    pub p: Vector3<f32>,
    pub n: Vector3<f32>
}

