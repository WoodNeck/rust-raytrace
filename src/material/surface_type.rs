use cgmath::{Vector3, InnerSpace};

use crate::ray::{Ray, Record};

#[derive(Copy, Clone)]
pub enum Surface {
    Lambertian,
    Metallic,
    Dielectric,
}

impl Surface {
    pub fn scat(self, ray: &mut Ray, hr: &mut Record) -> bool {
        match self {
            Surface::Lambertian => self.scat_lamb(ray, hr),
            Surface::Metallic => self.scat_metal(ray, hr),
            Surface::Dielectric => self.scat_dielectric(ray, hr),
        }
    }

    fn scat_lamb(self, ray: &mut Ray, hr: &mut Record) -> bool {
        let t = (hr.n + random_in_unit_sphere()).normalize();
        ray.origin = hr.p;
        ray.direction = t;

        true
    }

    fn scat_metal(self, ray: &mut Ray, hr: &mut Record) -> bool {
        let reflected = reflect(ray.direction.normalize(), hr.n);
        ray.origin = hr.p;
        ray.direction = reflected;

        reflected.dot(hr.n) > 0.
    }

    fn scat_dielectric(self, ray: &mut Ray, hr: &mut Record) -> bool {
        let ri = 1.5;
        let mut ior = ri;

        let reflected = reflect(ray.direction.normalize(), hr.n);
        let mut outward_normal = -hr.n;
        let reflect_prob: f32;
        let mut cosine = ray.direction.dot(hr.n);
        cosine = (1. - ri*ri*(1.0-cosine*cosine)).sqrt();

        if ray.direction.dot(hr.n) <= 0. {
            outward_normal = hr.n;
            ior = 1. / ior;
            cosine = ri * -ray.direction.dot(hr.n);
        }

        let refracted = refract(ray, outward_normal, ior);

        if refracted.is_some() {
            reflect_prob = schlick(cosine, ri);
        } else {
            reflect_prob = 1.0;
        }

        if rand::random::<f32>() < reflect_prob {
            ray.origin = hr.p;
            ray.direction = reflected;
        } else {
            ray.origin = hr.p;
            ray.direction = refracted.unwrap();
        }

        true
    }
}

fn random_in_unit_sphere() -> Vector3<f32> {
    Vector3 {
        x: rand::random(),
        y: rand::random(),
        z: rand::random()
    }
}

fn reflect(in_dir: Vector3<f32>, in_normal: Vector3<f32>) -> Vector3<f32> {
    return -2.0 * in_normal.dot(in_dir) * in_normal + in_dir;
}

fn refract(r: &mut Ray, n: Vector3<f32>, ior: f32) -> Option<Vector3<f32>> {
    let dt = r.direction.dot(n);
    let d = 1.0 - ior * ior * (1.0 - dt * dt);
    if d > 0. {
        Some(ior * (r.direction - n * dt) - n * d.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let r0 = (1. - ref_idx) / (1. + ref_idx);
    let r0 = r0 * r0;
    return r0 + (1. - r0) * (1. - cosine).powf(5.);
}
