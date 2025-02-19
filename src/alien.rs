use bevy::core::FrameCount;
use bevy::prelude::*;

pub const HEIGHT_SPRITES: f32 = 73.;
pub const PADDING_VERTICAL: f32 = 17.5;
pub const HEIGHT_BLOCK: f32 = HEIGHT_SPRITES + PADDING_VERTICAL * 2.;
pub const NUMBER_OF_VERTICAL_BLOCKS: f32 = 1080. / HEIGHT_BLOCK;

pub const WIDTH_SPRITES: f32 = 100.;
pub const PADDING_HORIZONTAL: f32 = 30.;
pub const WIDTH_BLOCK: f32 = WIDTH_SPRITES + PADDING_HORIZONTAL * 2.;
pub const NUMBER_OF_HORIZONTAL_BLOCKS: f32 = 1920. / WIDTH_BLOCK;

pub const MOVE_FREQUENCY_ALIEN: u32 = 40;
pub const MOVE_ROW_DOWN_FREQUENCY: u32 = MOVE_FREQUENCY_ALIEN * MAX_ROWS * 4;

const MAX_ROWS: u32 = 6;

pub struct AlienPlugin;

impl Plugin for AlienPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_aliens)
            .add_systems(Update, (move_aliens_row_down, move_aliens_left_or_right))
            .insert_resource(RowIndex { row: 0 });
    }
}

#[derive(Component)]
#[require(Sprite, Direction, Row)]
pub struct Alien;

#[derive(Default, Component, Clone)]
pub enum Direction {
    #[default]
    Right,
    Left,
}

#[derive(Resource)]
struct RowIndex {
    pub row: u32,
}

#[derive(Component, Default)]
pub struct Row {
    pub row: u32,
}

fn setup_aliens(mut commands: Commands, asset_server: Res<AssetServer>) {
    let alien = Sprite {
        image: asset_server.load("alien.png"),
        ..default()
    };

    for y in 0..MAX_ROWS {
        for x in 0..11 {
            commands.spawn((
                alien.clone(),
                Transform::from_xyz(
                    x as f32 * WIDTH_BLOCK + PADDING_HORIZONTAL + WIDTH_SPRITES / 2.,
                    -1. * (y as f32 * HEIGHT_BLOCK) - PADDING_VERTICAL - HEIGHT_SPRITES / 2.,
                    1.,
                ),
                Alien,
                Row { row: y },
            ));
        }
    }
}

fn move_aliens_row_down(
    mut query: Query<&mut Transform, With<Alien>>,
    frame_count: Res<FrameCount>,
) {
    //we add MOVE_FREQUENCY_ALIEN because we skip moving left or right on frame 0
    if frame_count.0 % (MOVE_ROW_DOWN_FREQUENCY + MOVE_FREQUENCY_ALIEN) != 0 || frame_count.0 == 0 {
        return;
    }

    println!("down");
    query.iter_mut().for_each(|mut transform| {
        transform.translation.y -= HEIGHT_BLOCK;
    })
}

fn move_aliens_left_or_right(
    mut query: Query<(&mut Transform, &mut Direction, &Row)>,
    frame_count: Res<FrameCount>,
    mut row_index: ResMut<RowIndex>,
) {
    if frame_count.0 % MOVE_FREQUENCY_ALIEN != 0
        || frame_count.0 % (MOVE_ROW_DOWN_FREQUENCY + MOVE_FREQUENCY_ALIEN) == 0
        || frame_count.0 == 0
    {
        return;
    }

    println!("horizontal");
    let min_x: f32 = query
        .iter()
        .filter(|(_, _, row)| row.row == row_index.row)
        .map(|(transform, _direction, _row)| transform.translation.x as i32)
        .min()
        .expect("there should be a min") as f32;

    let max_x: f32 = query
        .iter()
        .filter(|(_, _, row)| row.row == row_index.row)
        .map(|(transform, _direction, _row)| transform.translation.x as i32)
        .max()
        .expect("there should be a max") as f32;

    let mut direction = query
        .iter()
        .find(|(_, _, row)| row.row == row_index.row)
        .map(|(_, direction, _)| direction)
        .cloned()
        .expect("there should be a direction");

    if max_x + WIDTH_BLOCK / 2. >= 1920. {
        direction = Direction::Left;
    }

    if min_x - WIDTH_BLOCK / 2. <= 0. {
        direction = Direction::Right;
    }

    match direction {
        Direction::Right => {
            query
                .iter_mut()
                .for_each(|(mut transform, mut direction, row)| {
                    if row.row == row_index.row {
                        transform.translation.x += WIDTH_BLOCK / 2.;
                        *direction = Direction::Right;
                    }
                });
        }
        Direction::Left => {
            query
                .iter_mut()
                .for_each(|(mut transform, mut direction, row)| {
                    if row.row == row_index.row {
                        transform.translation.x -= WIDTH_BLOCK / 2.;
                        *direction = Direction::Left;
                    }
                });
        }
    }
    row_index.row = if row_index.row + 1 < MAX_ROWS {
        row_index.row + 1
    } else {
        0
    };
}
