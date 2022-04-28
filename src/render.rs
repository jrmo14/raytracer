use crate::framebuffer::FrameBuffer;
use crate::geometry::{Object, Sphere};
use crate::lighting::{Light, Material};
use crate::vector::{Vec3, Vec3f, Vec4f};
use std::f32::consts::PI;

pub const WIDTH: usize = 1024;
pub const HEIGHT: usize = 768;
const WH_RATIO: f32 = (WIDTH as f32) / (HEIGHT as f32);
const FOV: f32 = PI / 2.0;

const CAST_DEPTH_MAX: usize = 4;
const BACKGROUND: Vec3f = Vec3f {
    x: 0.2,
    y: 0.7,
    z: 0.8,
};

pub fn scene_intersect(
    orig: Vec3f,
    dir: Vec3f,
    objects: &[Box<dyn Object>],
) -> Option<(Material, Vec3f, Vec3f)> {
    let mut spheres_dist = f32::MAX;
    let mut mat = None;
    let mut n = None;
    let mut hit = None;

    for object in objects {
        let dist_i = object.ray_intersect(orig, dir);
        if let Some(dist_i) = dist_i {
            if dist_i < spheres_dist {
                spheres_dist = dist_i;
                hit = Some(orig + dir * dist_i);
                n = Some(object.compute_normal(hit.unwrap()));
                mat = Some(object.get_material());
            }
        }
    }
    // Just need to check one of them
    hit.map(|hit| (mat.unwrap(), n.unwrap(), hit))
}

fn reflect(i: Vec3f, n: Vec3f) -> Vec3f {
    i - n * 2.0 * (i * n)
}

fn refract(i: Vec3f, mut n: Vec3f, refractive_idx: f32) -> Vec3f {
    let mut cosi = -(-1.0_f32).max((1.0_f32).min(i * n));
    let mut etai = 1.0;
    let mut etat = refractive_idx;
    if cosi < 0.0 {
        cosi = -cosi;
        n = -n;
        std::mem::swap(&mut etai, &mut etat);
    }
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0 {
        Vec3f::id()
    } else {
        i * eta + n * (eta * cosi - k.sqrt())
    }
}

pub fn cast_ray(
    orig: Vec3f,
    dir: Vec3f,
    objects: &[Box<dyn Object>],
    lights: &[Light],
    depth: usize,
) -> Vec3f {
    if depth > CAST_DEPTH_MAX {
        return BACKGROUND;
    }
    if let Some((material, norm, point)) = scene_intersect(orig, dir, objects) {
        let mut diffuse_intensity = 0.0;
        let mut specular_intensity = 0.0;
        let reflect_dir = reflect(dir, norm).normalize();
        let refract_dir = refract(dir, norm, material.refractive_idx);
        let reflect_orig = if reflect_dir * norm < 0.0 {
            point - norm * 1e-3
        } else {
            point + norm * 1e-3
        };

        let refract_orig = if refract_dir * norm < 0.0 {
            point - norm * 1e-3
        } else {
            point + norm * 1e-3
        };

        let reflect_color = cast_ray(reflect_orig, reflect_dir, objects, lights, depth + 1);
        let refract_color = cast_ray(refract_orig, refract_dir, objects, lights, depth + 1);

        for light in lights {
            let light_dist = (light.position - point).norm();
            let light_dir = (light.position - point) / light_dist;
            let shadow_orig = if light_dir * norm < 0.0 {
                point - norm * 1e-3
            } else {
                point + norm * 1e-3
            };
            if let Some((_, _shadow_norm, shadow_pt)) =
                scene_intersect(shadow_orig, light_dir, objects)
            {
                if (shadow_pt - shadow_orig).norm() < light_dist {
                    continue;
                }
            }
            diffuse_intensity += light.intensity * (0.0_f32).max(light_dir * norm);
            specular_intensity += (0.0_f32)
                .max(reflect(light_dir, norm) * dir)
                .powf(material.specular_exp)
                * light.intensity;
        }
        material.diffuse_color * diffuse_intensity * material.albedo.w
            + specular_intensity * material.albedo.x
            + reflect_color * material.albedo.y
            + refract_color * material.albedo.z
    } else {
        BACKGROUND
    }
}

pub fn render(objects: &[Box<dyn Object>], lights: &[Light], framebuffer: &mut FrameBuffer) {
    let scale = |v: f32| (255.0 * (0.0_f32).max((1.0_f32).min(v))) as u8;
    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2.0 * (i as f32 + 0.5) / (WIDTH as f32) - 1.0) * (FOV / 2.0).tan() * WH_RATIO;
            let y = -(2.0 * (j as f32 + 0.5) / (HEIGHT as f32) - 1.0) * (FOV / 2.0).tan();
            let dir = Vec3f::new(x, y, -1.0).normalize();
            let el = cast_ray(Vec3f::id(), dir, objects, lights, 0);
            let x = scale(el.x);
            let y = scale(el.y);
            let z = scale(el.z);
            framebuffer.data[i + j * WIDTH] = Vec3::<u8> { x, y, z };
        }
    }
}
