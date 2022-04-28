use crate::vector::{Vec3f, Vec4f};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Material {
    pub diffuse_color: Vec3f,
    pub albedo: Vec4f,
    pub refractive_idx: f32,
    pub specular_exp: f32,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Light {
    pub position: Vec3f,
    pub intensity: f32,
}

impl Material {
    pub fn new(
        diffuse_color: Vec3f,
        albedo: Vec4f,
        specular_exp: f32,
        refractive_idx: f32,
    ) -> Self {
        Self {
            diffuse_color,
            albedo,
            specular_exp,
            refractive_idx,
        }
    }
}
