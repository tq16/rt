use crate::math::vec3::Vec3;

pub fn write_color(color: Vec3) {
    let red = (color.x.clamp(0.0, 1.0) * 255.999) as u8;
    let green = (color.y.clamp(0.0, 1.0) * 255.999) as u8;
    let blue = (color.z.clamp(0.0, 1.0) * 255.999) as u8;

    println!("{} {} {}", red, green, blue);
}
