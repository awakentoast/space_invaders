use crate::misc::WIDTH_BLOCK;
use bevy::core::FrameCount;
use bevy::prelude::*;

#[derive(Component)]
#[require(Sprite, Direction, Row)]
pub struct Alien;

#[derive(Default, Clone)]
pub enum Directions {
    #[default]
    Right,
    Left,
}

#[derive(Resource)]
pub struct RowIndex {
    pub row: u8,
}

#[derive(Component, Default)]
pub struct Row {
    pub row: u8,
}

#[derive(Component, Default)]
pub struct Direction {
    pub direction: Directions,
}

const MAX_ROWS: u8 = 2;
pub const PADDING_ALIEN: f32 = 50.;

pub fn move_aliens(
    mut query: Query<(&mut Transform, &mut Direction, &Row)>,
    frame_count: Res<FrameCount>,
    mut row_index: ResMut<RowIndex>,
) {
    if frame_count.0 % 60 != 0 || frame_count.0 == 0 {
        return;
    }

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
        .filter(|(_, _, row)| row.row == row_index.row)
        .next()
        .map(|(_, direction, _)| direction)
        .map(|direction| direction.direction.clone())
        .expect("there should be a direction");

    if max_x + WIDTH_BLOCK / 2. >= 1920. {
        direction = Directions::Left;
    }

    if min_x - WIDTH_BLOCK / 2. <= 0. {
        direction = Directions::Right;
    }

    match direction {
        Directions::Right => {
            query
                .iter_mut()
                .for_each(|(mut transform, mut direction, row)| {
                    if row.row == row_index.row {
                        transform.translation.x += WIDTH_BLOCK / 2.;
                        *direction = Direction {
                            direction: Directions::Right,
                        };
                    }
                });
        }
        Directions::Left => {
            query
                .iter_mut()
                .for_each(|(mut transform, mut direction, row)| {
                    if row.row == row_index.row {
                        transform.translation.x -= WIDTH_BLOCK / 2.;
                        *direction = Direction {
                            direction: Directions::Left,
                        };
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
