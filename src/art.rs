use bevy::prelude::*;
use bevy::render::render_resource::{TextureDimension, TextureFormat, Extent3d};

#[derive(Resource, Default)]
pub struct Sprites {
    pub player: Handle<Image>,
    pub bullet: Handle<Image>,
    pub enemy: Handle<Image>,
}

const SIZE : usize = 5;

const O : [f32;4] = [0.0, 0.0, 0.0, 0.0];
const X : [f32;4] = [1.0, 1.0, 1.0, 1.0];

fn make_image(
    data: &[[f32;4]; SIZE*SIZE],
) -> Image {
    let data = bytemuck::cast_slice(data);

    Image::new_fill(
        Extent3d { 
            width: SIZE as u32,
            height: SIZE as u32,
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
    let player = [
        O, O, X, O, O,
        O, X, X, X, O,
        O, X, X, X, O,
        X, X, X, X, X,
        X, X, X, X, X,
    ];
    let bullet = [
        O, O, O, O, O,
        O, O, X, O, O,
        O, O, X, O, O,
        O, O, X, O, O,
        O, O, O, O, O,
    ];
    let enemy = [
        O, O, X, O, O,
        O, X, X, X, O,
        O, X, O, X, O,
        O, X, X, X, O,
        O, O, X, O, O,
    ];

    Sprites {
        player: images.add(make_image(&player)),
        bullet: images.add(make_image(&bullet)),
        enemy: images.add(make_image(&enemy)),
    }
}

