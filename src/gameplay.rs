use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
struct Player;

#[derive(Component)]
pub struct GridPosition {
    pub row: u32,
    pub col: u32,
}

// from 0.00 to 1.00
#[derive(Component)]
struct HorizontalMovement {
    x: f64,
    speed: f64,
}

#[derive(Component)]
struct HP {
    max: i32,
    current: i32,
}

#[derive(Bundle)]
struct TileBundle {
    tile: Tile,
    position: GridPosition,
    sprite: SpriteBundle,
    hp: HP,
}

impl TileBundle {
    fn new(row: u32, col: u32, hp: Option<i32>, asset_server: &Res<AssetServer>) -> Self {
        TileBundle {
            tile: Tile,
            position: GridPosition { row: row, col: col },
            sprite: crate::graphics::tile_sprite(asset_server),
            hp: HP {
                max: hp.unwrap_or(1),
                current: hp.unwrap_or(1),
            },
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    position: HorizontalMovement,
    sprite: SpriteBundle,
    hp: HP,
}

impl PlayerBundle {
    fn new(hp: Option<i32>, asset_server: &Res<AssetServer>) -> Self {
        PlayerBundle {
            player: Player,
            position: HorizontalMovement { x: 0., speed: 0. },
            sprite: crate::graphics::player_sprite(asset_server),
            hp: HP {
                max: hp.unwrap_or(3),
                current: hp.unwrap_or(3),
            },
        }
    }
}

pub struct Gameplay;

impl Plugin for Gameplay {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init);
    }
}

fn init(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(PlayerBundle::new(None, &asset_server));

    commands.spawn(TileBundle::new(0, 0, None, &asset_server));
    commands.spawn(TileBundle::new(1, 0, None, &asset_server));
    commands.spawn(TileBundle::new(0, 1, None, &asset_server));
}
