use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat, Extent3d};

#[derive(Resource, Default)]
pub struct Sprites {
    pub player: Handle<Image>,
    pub bullet: Handle<Image>,
    pub enemy: Handle<Image>,
}

const HEIGHT : usize = 8;
const WIDTH : usize = 16;


const O: [f32; 4] = [0.0, 0.0, 0.0, 0.0];
const I: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
const M: [f32; 4] = [0.6, 0.6, 0.6, 1.0];
const X: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

fn make_image(
    data: &[[f32;4]; WIDTH*HEIGHT],
) -> Image {
    let data = bytemuck::cast_slice(data);

    Image::new_fill(
        Extent3d { 
            width: WIDTH as u32,
            height: HEIGHT as u32,
            depth_or_array_layers: 1
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba32Float
    )
}

pub fn make_sprites(
    mut images: ResMut<Assets<Image>>,
) -> Sprites {
    // Thank you GPT-4 for the sprites!
    let player = [
        O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O,
        O, O, O, O, O, I, O, O, I, O, O, O, O, O, O, O,
        O, O, O, O, M, I, I, I, I, M, O, O, O, O, O, O,
        O, O, O, M, I, I, I, I, I, I, M, O, O, O, O, O,
        O, O, M, I, I, I, I, I, I, I, I, M, O, O, O, O,
        O, M, I, I, I, I, I, X, X, I, I, I, I, M, O, O,
        M, I, I, I, I, M, O, O, O, O, M, I, I, I, I, M,
        O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O,
    ];
    let bullet = [
        O, O, O, O, O, X, O, O, O, O, O, O, O, O, O, O,
        O, O, O, O, O, X, O, O, O, O, O, O, O, O, O, O,
        O, O, O, O, O, X, O, O, O, O, O, O, O, O, O, O,
        O, O, O, O, O, X, O, O, O, O, O, O, O, O, O, O,
        O, O, O, O, I, M, I, O, O, O, O, O, O, O, O, O,
        O, O, O, O, O, I, O, O, O, O, O, O, O, O, O, O,
        O, O, O, O, O, X, O, O, O, O, O, O, O, O, O, O,
        O, O, O, O, O, X, O, O, O, O, O, O, O, O, O, O,
    ];
    let enemy = [
        O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O,
        O, O, O, O, O, O, O, O, O, O, O, O, O, O, O, O,
        O, O, O, O, O, I, O, O, O, I, O, O, O, O, O, O,
        O, O, O, O, I, M, I, O, O, I, M, I, O, O, O, O,
        O, O, O, I, M, X, M, I, I, M, X, M, I, O, O, O,
        O, O, I, M, X, M, X, M, M, X, M, X, M, I, O, O,
        O, I, M, X, M, M, X, X, X, X, M, M, X, M, I, O,
        I, M, X, M, M, X, M, O, O, M, X, M, M, X, M, I,
    ];

    Sprites {
        player: images.add(make_image(&player)),
        bullet: images.add(make_image(&bullet)),
        enemy: images.add(make_image(&enemy)),
    }
}

