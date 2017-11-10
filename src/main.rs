pub mod vec3;
pub mod ray;


fn hit_sphere(center :vec3::Vec3, radius: f32, r :ray::Ray) -> bool {
    let oc : vec3::Vec3 = r.origin() - center;
    let a : f32 = vec3::dot(r.direction(), r.direction());
    let b : f32 = 2.0 * vec3::dot(oc,r.direction());
    let c : f32 = vec3::dot(oc,oc) - radius * radius;
    let discriminent = b * b - 4.0 * a * c;
    discriminent > 0.0
}

fn color(r :ray::Ray) -> vec3::Vec3 {
    if hit_sphere(vec3::Vec3 ::new(0.0,0.0,-1.0), 0.5, r) {
        return vec3::Vec3::new(1.0,0.0,0.0);
    }
    let unit_direction : vec3::Vec3 = vec3::unit_vector(r.direction());
    let t: f32 = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t)*vec3::Vec3::new(1.0,1.0,1.0) + t*vec3::Vec3::new(0.5,0.7,1.0)
}

fn main() {

    let nx : u8 = 200;
    let ny : u8 = 100;

    println!("P3");
    println!("{0} {1}",nx,ny);
    println!("255");
    let lower_left_corner: vec3::Vec3 = vec3::Vec3::new(-2.0,-1.0,-1.0);
    let horizontal: vec3::Vec3 = vec3::Vec3::new(4.0,0.0,0.0);
    let vertical: vec3::Vec3 = vec3::Vec3::new(0.0,2.0,0.0);
    let origin : vec3::Vec3 = vec3::Vec3::new(0.0,0.0,0.0);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u : f32 = i as f32 / nx as f32;
            let v : f32 = j as f32 / ny as f32;
            let r : ray::Ray = ray::Ray::new(origin,lower_left_corner + u * horizontal + v * vertical);
            let col = color(r);

            let ir : u8 = (255.99 * col[0]) as u8;
            let ig : u8 = (255.99 * col[1]) as u8;
            let ib : u8 = (255.99 * col[2]) as u8;

            println!("{0} {1} {2}",ir,ig,ib);
        }
    }
}
