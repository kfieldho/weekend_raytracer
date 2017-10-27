fn main() {

    let nx : u8 = 200;
    let ny : u8 = 100;

    println!("P3");
    println!("{0} {1}",nx,ny);
    println!("255");

    for j in (0..ny).rev() {
        for i in 0..nx {
            let r : f32 = i as f32 / nx as f32;
            let g : f32 = j as f32 / ny as f32;
            let b : f32 = 0.2;

            // println!("{0} {1} {2} ({3} {4})",r,g,b,j,i);

            let ir : u8 = (255.99 * r) as u8;
            let ig : u8 = (255.99 * g) as u8;
            let ib : u8 = (255.99 * b) as u8;

            println!("{0} {1} {2}",ir,ig,ib);
        }
    }
}

