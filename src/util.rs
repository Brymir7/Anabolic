use macroquad::math::Vec3;

pub fn vec3_no_y(vec: Vec3) -> Vec3 {
    return Vec3::new(vec.x, 0.0, vec.z);
}