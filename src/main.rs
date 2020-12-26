use raytracer::{ray::Ray, vector::Vector3};
use std::fs;
use raytracer::color;

fn main() {
    let aspect_ratio=16.0/9.0;
    
    let image_width = 256u32;
    let image_height = (image_width as f64/aspect_ratio)as u32;

    let viewport_height=2u32;
    let viewport_width= (viewport_height as f64 *aspect_ratio) as u32;
    let focal_length=1.0;

    let origin=Vector3::new(0.0,0.0,0.0);
    let horizontal=Vector3::new(viewport_width as f64,0.0,0.0);
    let vertical= Vector3::new(0.0,viewport_height as f64,0.0);
    let lower_left_corner:Vector3=origin-(horizontal/2.0)-vertical/2.0-Vector3::new(0.0,0.0,1.0)*focal_length;

    let mut contents = String::new();
    contents.push_str("P3\n");
    contents = format!("{}{} {}\n", contents, image_width, image_height);
    contents = format!("{}{}\n", contents, "255");

    for i in (0..image_height).rev() {
        println!("Lines remaining {}", i);
        for j in (0..image_width){
            let v = i as f64/(image_height-1) as f64;
            let h = j as f64/(image_width-1) as f64;
            let direction:Vector3=v*vertical+h*horizontal+lower_left_corner -origin;
            let ray=Ray::new(origin,direction);
            contents=format!("{}{}\n",contents,color::write_color(ray_color(ray)));
        }
    }

    fs::write("output.ppm", contents.as_bytes()).unwrap();
}

fn sphere_collided(radius:f64,ray:&Ray,center:Vector3)->bool{
    //𝑡^2𝐛⋅𝐛+2𝑡𝐛⋅(𝐀−𝐂)+(𝐀−𝐂)⋅(𝐀−𝐂)−𝑟2=0 solve with quadratic formula
    let a_c=ray.origin-center;
    
    let a:f64=ray.direction.length_squared(); //same as dot product
    let b=2.0*(ray.direction.dot(a_c));
    let c=a_c.length_squared() -radius*radius;
    
    let discriminant= b*b-4.0*a*c;

    if discriminant>0f64{
        return true;
    }
    else{
        false
    }
}


fn ray_color(ray:Ray)->Vector3{
    if sphere_collided(0.4, &ray, Vector3::new(0.0,0.0,-1.0)){
        return Vector3::new(1.0,0.2,0.0);
    }
    let unit_vector=ray.direction.unit_vector();
    let t=(unit_vector.y+1.0)*0.5;
    
    (1.0-t)*Vector3::new(1.0, 0.7, 0.7) + t*Vector3::new(0.5, 0.7, 1.0)
}
