use crate::{ray::Ray, vector::Vector3};

pub fn write_color(pixel:Vector3) -> String{
    let r=pixel.x*255.0;
    let g=pixel.y*255.0;
    let b=pixel.z*255.0;
    format!("{} {} {}",r as usize,g as usize,b as usize)
}
