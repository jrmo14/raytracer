use crate::vector::Vec3;
pub struct FrameBuffer {
    pub data: Box<[Vec3<u8>]>,
    width: usize,
    height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let data = vec![Vec3::<u8>::default(); width * height].into_boxed_slice();
        Self {
            data,
            width,
            height,
        }
    }
    pub fn save(self, fname: &str) -> io::Result<()> {
        let mut file = File::create(fname)?;
        let hdr = format!("P6\n{} {}\n255\n", self.width, self.height);
        file.write_all(hdr.as_bytes())?;

        for el in self.data.iter() {
            let data = [el.x, el.y, el.z];
            file.write_all(&data)?;
        }
        Ok(())
    }
}

use std::{
    fs::File,
    io::{self, Write},
};
