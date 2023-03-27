use cgmath::Vector3;

use crate::{element::sphere::Sphere, material::{Material, surface_type::Surface}, ray::{Ray, Record}};

pub struct Scene {
    pub spheres: Vec<Sphere>
}

impl Scene {
    pub fn new() -> Scene {
        let mut spheres = Vec::with_capacity(30);

        spheres.push(Sphere {
            material: Material {
                surface: Surface::Lambertian,
                albedo: Vector3 { x: 0.5, y: 0.5, z: 0.5 }
            },
            center: Vector3 { x: 0.0, y: -1000.0, z: 0.0 },
            radius: 1000.
        });
        spheres.push(Sphere {
            material: Material {
                surface: Surface::Metallic,
                albedo: Vector3 { x: 0.4, y: 0.2, z: 0.6 }
            },
            center: Vector3 { x: 0.0, y: 1.0, z: 0.0 },
            radius: 1.
        });
        spheres.push(Sphere {
            material: Material {
                surface: Surface::Dielectric,
                albedo: Vector3 { x: 1.0, y: 1.0, z: 1.0 }
            },
            center: Vector3 { x: -0.5, y: 1.0, z: 6.5 },
            radius: 1.
        });
        spheres.push(Sphere {
            material: Material {
                surface: Surface::Lambertian,
                albedo: Vector3 { x: 0.8, y: 0.4, z: 0.2 }
            },
            center: Vector3 { x: 3.5, y: 1.0, z: 8.5 },
            radius: 1.
        });

        for i in 4..30 {
            let grid_x = i / 10;
            let grid_y = (i as f32) - (10. * grid_x as f32);
            let pos = Vector3::new(grid_x as f32, 0.2, grid_y)
                + Vector3::new(rand::random(), 0., rand::random());
            let size = 0.2;
            let color = Vector3::new(rand::random(), rand::random(), rand::random());

            spheres.push(Sphere {
                material: Material {
                    surface: Surface::Lambertian,
                    albedo: color
                },
                center: pos,
                radius: size
            });
        }

        Scene {
            spheres
        }
    }

    pub fn traverse(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut Record) -> Option<&Material> {
        rec.t = t_max;

        let mut mat: Option<&Material> = None;

        for sphere in self.spheres.as_slice() {
            let ret = sphere.hit(ray, t_min, rec);
            if ret.is_some() {
                mat = ret;
            }
        }

        mat
    }
}
