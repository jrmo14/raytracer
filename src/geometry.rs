use crate::{lighting::Material, vector::{Vec3f, Vec4f}};

pub trait Object {
    fn ray_intersect(&self, orig: Vec3f, dir: Vec3f) -> Option<f32>;
    fn get_material(&self) -> Material;
    fn compute_normal(&self, hit: Vec3f) -> Vec3f;
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub center: Vec3f,
    pub radius: f32,
    pub material: Material,
}

pub struct Plane {
    pub center: Vec3f,
    pub material: Material,
}

impl Object for Plane {
    fn ray_intersect(&self, orig: Vec3f, dir: Vec3f) -> Option<f32> {
        if dir.y.abs() > 1e-3 {
            let d = -(orig.y + 4.0) / dir.y;
            let pt = orig + dir * d;
            if d > 0.0 && pt.x.abs() < 10.0 && pt.z < -10. && pt.z > -30. {
               Some(d)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn compute_normal(&self, _: Vec3f) -> Vec3f {
        Vec3f::new(0., 1., 0.)
    }
}

impl Object for Sphere {
    fn ray_intersect(&self, orig: Vec3f, dir: Vec3f) -> Option<f32> {
        let l = self.center - orig;
        let tca = l * dir;
        let d2 = l * l - tca * tca;
        let r2 = self.radius.powi(2);
        if d2 > r2 {
            return None;
        }
        let thc = (r2 - d2).sqrt();
        let mut t0 = tca - thc;
        if t0 < 0.0 {
            t0 = tca + thc;
        }
        if t0 < 0.0 {
            return None;
        }
        Some(t0)
    }

    fn get_material(&self) -> Material {
        self.material
    }

    fn compute_normal(&self, hit: Vec3f) -> Vec3f {
        (hit - self.center).normalize()
    }
}
