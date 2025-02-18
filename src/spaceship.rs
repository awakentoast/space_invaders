use crate::alien::*;
use crate::misc::WIDTH_BLOCK;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::time::Stopwatch;
use std::ptr::copy;

#[derive(Component)]
#[require(Sprite, MovementCooldown)]
pub struct Spaceship;

#[derive(Component)]
pub struct MovementCooldown {
    button: KeyCode,
    stopwatch: Stopwatch,
}

impl Default for MovementCooldown {
    fn default() -> Self {
        Self {
            button: KeyCode::KanaMode,
            stopwatch: Stopwatch::new(),
        }
    }
}

pub fn input_spaceship(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &MovementCooldown), With<Spaceship>>,
    time: Res<Time>,
) {
    let mut spaceship_translation =
        query
            .iter_mut()
            .for_each(|(mut transform, mut cd)| {
                if keys.pressed(KeyCode::ArrowLeft) && transform.translation.x - WIDTH_BLOCK >= 0. {
                    if cd.button == KeyCode::ArrowLeft {
                        if cd.stopwatch.elapsed_secs() >= 0.5 {
                            transform.translation.x -= WIDTH_BLOCK / 2.;
                            cd.stopwatch.reset();
                        }
                    }
                }

                if keys.pressed(KeyCode::ArrowRight)
                    && transform.translation.x + WIDTH_BLOCK / 2. < 1920.
                {
                    if cd.button == KeyCode::ArrowRight {
                        if cd.stopwatch.elapsed_secs() >= 0.5 {
                            transform.translation.x += WIDTH_BLOCK / 2.;
                            cd.stopwatch.reset();
                        }
                    }
                }
            });
}
