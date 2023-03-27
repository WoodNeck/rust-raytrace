use cgmath::Vector3;

use self::surface_type::Surface;

pub mod surface_type;

pub struct Material {
    pub surface: Surface,
    pub albedo: Vector3<f32>
}
