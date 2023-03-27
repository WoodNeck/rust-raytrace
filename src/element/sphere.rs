use cgmath::{Vector3, InnerSpace};

use crate::{material::Material, ray::{Ray, Record}};

pub struct Sphere {
    pub material: Material,
    pub center: Vector3<f32>,
    pub radius: f32
}

impl Sphere {
    pub fn hit(&self, r: &Ray, t_min: f32, hr: &mut Record) -> Option<&Material> {
        let oc = r.origin - self.center;
        let a = r.direction.dot(r.direction);
        let b = oc.dot(r.direction);
        let c = oc.dot(oc) - self.radius * self.radius;

        let d = b * b - a * c;
        if d < 0. {
            return None;
        }

        let s = d.sqrt();

        let val = (-b - s) / a;
        if val > t_min && val < hr.t {
            hr.p = r.origin + val * r.direction;
            hr.n = (hr.p - self.center) / self.radius;
            hr.t = val;
            let mat = &self.material;
            return Some(mat);
        }

        let val = (-b + s) / a;
        if val > t_min && val < hr.t {
            hr.p = r.origin + val * r.direction;
            hr.n = (hr.p - self.center) / self.radius;
            hr.t = val;
            let mat = &self.material;
            return Some(mat);
        }

        None
    }
}
