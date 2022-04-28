mod framebuffer;
mod geometry;
mod lighting;
mod render;
mod vector;

use std::io;

use framebuffer::FrameBuffer;
use geometry::{Object, Sphere, Plane};
use lighting::{Light, Material};
use render::{render, HEIGHT, WIDTH};
use vector::{Vec3f, Vec4f};

fn main() -> io::Result<()> {
    let ivory = Material::new(
        Vec3f::new(0.4, 0.4, 0.3),
        Vec4f::new(0.6, 0.3, 0.4, 0.0),
        50.0,
        1.0,
    );
    let red_rubber = Material::new(
        Vec3f::new(0.3, 0.1, 0.1),
        Vec4f::new(0.9, 0.1, 0.0, 0.0),
        10.0,
        1.0,
    );
    let mirror = Material::new(
        Vec3f::new(1.0, 1.0, 1.0),
        Vec4f::new(0.0, 10.0, 0.8, 0.0),
        1425.0,
        1.0,
    );
    let glass = Material::new(
        Vec3f::new(0.6, 0.7, 0.8),
        Vec4f::new(0.0, 0.5, 0.1, 0.8),
        125.,
        1.5,
    );
    let objects: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere {
            center: Vec3f::new(-3.0, 0.0, -16.0),
            radius: 2.0,
            material: ivory,
        }),
        Box::new(Sphere {
            center: Vec3f::new(-1.0, -1.5, -12.0),
            radius: 2.0,
            material: glass,
        }),
        Box::new(Sphere {
            center: Vec3f::new(-1.0, -1.5, -12.0),
            radius: 2.0,
            material: mirror,
        }),
        Box::new(Sphere {
            center: Vec3f::new(1.5, -0.5, -18.0),
            radius: 3.0,
            material: red_rubber,
        }),
        Box::new(Sphere {
            center: Vec3f::new(7.0, 5.0, -18.0),
            radius: 4.0,
            material: mirror,
        }),
        Box::new(Plane{
            center: Vec3f::new(0.0, 0.0, 0.0),
            material: red_rubber,
        })
    ];
    let lights = [
        Light {
            position: Vec3f::new(-20.0, 20.0, 20.0),
            intensity: 1.5,
        },
        Light {
            position: Vec3f::new(30.0, 50., -25.),
            intensity: 1.8,
        },
        //Light {
        //    position: Vec3f::new(30., 20., 30.),
        //    intensity: 1.7,
        //},
    ];
    let mut framebuffer = FrameBuffer::new(WIDTH, HEIGHT);
    render(&*objects, &lights, &mut framebuffer);
    framebuffer.save("./out.ppm")
}
