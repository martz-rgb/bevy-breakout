use bevy::prelude::*;

use crate::gameplay;

pub const WINDOW_WIDTH: f32 = 400.;
pub const WINDOW_HEIGHT: f32 = 600.;

const WINDOW_TRANSLATION: Vec3 = Vec3 {
    x: -WINDOW_WIDTH / 2.,
    y: WINDOW_HEIGHT / 2.,
    z: 0.,
};

const TILE_SPRITE: &str = "tile.png";
const TILE_WIDTH: u32 = 128;
const TILE_HEIGHT: u32 = 32;

const TILE_TRANSLATION: Vec3 = Vec3 {
    x: TILE_WIDTH as f32 / 2.,
    y: TILE_HEIGHT as f32 / -2.,
    z: 0.,
};

const GRID_COLS: u32 = WINDOW_WIDTH as u32 / TILE_WIDTH;
const GRID_ROWS: u32 = WINDOW_HEIGHT as u32 / TILE_HEIGHT;

// and move grid center to window center
const GRID_TRANSLATION: Vec3 = Vec3 {
    x: (WINDOW_WIDTH - (GRID_COLS * TILE_WIDTH) as f32) / 2.,
    y: -(WINDOW_HEIGHT - (GRID_ROWS * TILE_HEIGHT) as f32) / 2.,
    z: 0.,
};

const PLAYER_SPRITE: &str = "tile.png";
const PLAYER_WIDTH: u32 = 128;
const PLAYER_HEIGHT: u32 = 32;

const PLAYER_PADDING: u32 = 2 * PLAYER_HEIGHT;

#[derive(Component)]
pub struct MainCamera;

pub fn player_sprite(asset_server: &Res<AssetServer>) -> SpriteBundle {
    SpriteBundle {
        texture: asset_server.load(PLAYER_SPRITE),
        transform: Transform {
            translation: Vec3 {
                x: 0., // center
                y: -WINDOW_HEIGHT / 2. + PLAYER_PADDING as f32,
                z: 0.,
            },
            ..Default::default()
        },
        ..Default::default()
    }
}

pub fn tile_sprite(
    position: gameplay::GridPosition,
    asset_server: &Res<AssetServer>,
) -> SpriteBundle {
    SpriteBundle {
        texture: asset_server.load(TILE_SPRITE),
        transform: Transform {
            translation: position_tile(position),
            ..Default::default()
        },
        ..Default::default()
    }
}

pub struct Graphics;

impl Plugin for Graphics {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init.in_set(super::GraphicSet));

        app.add_systems(Update, position_player.in_set(super::GraphicSet));
    }
}

fn init(mut commands: Commands) {
    commands.spawn((MainCamera, Camera2dBundle::default()));
}

fn position_tile(position: gameplay::GridPosition) -> Vec3 {
    if position.col >= GRID_COLS || position.row >= GRID_ROWS {
        // error actually
        return Vec3 {
            ..Default::default()
        };
    }

    let mut translation = Vec3 {
        x: (position.col * TILE_WIDTH) as f32,
        y: -1. * (position.row * TILE_HEIGHT) as f32,
        z: 0.,
    };

    translation += WINDOW_TRANSLATION + TILE_TRANSLATION + GRID_TRANSLATION;

    return translation;
}

fn position_player() {}

// fn position_tiles(
//     mut tiles: Query<(&gameplay::GridPosition, &mut Transform), With<gameplay::Tile>>,
// ) {
//     for (position, mut transform) in &mut tiles {
//         if position.col >= GRID_COLS || position.row >= GRID_ROWS {
//             // delete maybe
//             continue;
//         }

//         let translation = Vec3 {
//             x: (position.col * TILE_WIDTH) as f32,
//             y: -1. * (position.row * TILE_HEIGHT) as f32,
//             z: 0.,
//         };

//         transform.translation =
//             WINDOW_TRANSLATION + translation + TILE_TRANSLATION + GRID_TRANSLATION;
//     }
// }
