pub mod vec3;

fn main() {

    let nx : u8 = 200;
    let ny : u8 = 100;

    println!("P3");
    println!("{0} {1}",nx,ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            //let r : f32 = i as f32 / nx as f32;
            //let g : f32 = j as f32 / ny as f32;
            //let b : f32 = 0.2;
            let color = vec3::Vec3::new(i as f32 / nx as f32,j as f32 / ny as f32, 0.2);

            let ir : u8 = (255.99 * color[0]) as u8;
            let ig : u8 = (255.99 * color[1]) as u8;
            let ib : u8 = (255.99 * color[2]) as u8;

            println!("{0} {1} {2}",ir,ig,ib);
        }
    }
}

