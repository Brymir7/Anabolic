use dot_vox::load;
use macroquad::math::Vec3;

pub fn vec3_no_y(vec: Vec3) -> Vec3 {
    return Vec3::new(vec.x, 0.0, vec.z);
}
use macroquad::{
    color::Color,
    prelude::ImageFormat,
    texture::{Image, Texture2D},
};
use shared::types::{Voxel, VoxelMesh};

pub fn is_white(color: Color) -> bool {
    color.r == 1.0 && color.g == 1.0 && color.b == 1.0
}

pub fn convert_white_to_transparent(image: &mut Image) {
    for pixel in image.get_image_data_mut().iter_mut() {
        if is_white((*pixel).into()) {
            *pixel = Color::new(0.0, 0.0, 0.0, 0.0).into(); // Transparent color
        }
    }
}

pub fn load_and_convert_texture(data: &[u8], format: ImageFormat) -> Texture2D {
    let texture = Texture2D::from_file_with_format(data, Some(format));
    let mut texture_data = texture.get_texture_data();
    convert_white_to_transparent(&mut texture_data);
    texture.update(&texture_data);
    texture
}

pub fn load_voxel_data(filename: &str) -> VoxelMesh {
    let vox_data = load(filename).expect("Faield ot load");

    // Extract voxels and their colors
    let mut voxel_mesh = VoxelMesh { voxels: Vec::new() };
    
    for model in &vox_data.models {
        for voxel in &model.voxels {
            let color_index = voxel.i as usize;
            let color = vox_data.palette[color_index];
            voxel_mesh.voxels.push(Voxel {
                position: Vec3::new(voxel.x as f32, voxel.z as f32, voxel.y as f32) *0.5,
                color: Color::new(
                    color.r as f32 / 255.0,
                    color.g as f32 / 255.0,
                    color.b as f32 / 255.0,
                    1.0,
                ),
            });
        }
    }
    voxel_mesh
}