use crate::alien::*;
use crate::bullet::Bullet;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy::utils::HashMap;

const MOVEMENT_COOLDOWN: f32 = 0.166667;
const SHOOT_COOLDOWN: f32 = 0.5;

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_spaceship).add_systems(
            Update,
            (
                update_movement_timer,
                update_shoot_timer,
                input_spaceship,
                shoot,
            ),
        );
    }
}

#[derive(Component)]
#[require(Sprite, Movement, Shoot)]
pub struct Spaceship;

#[derive(Component, Default)]
pub struct Movement {
    pub cooldown: HashMap<KeyCode, Timer>,
}

#[derive(Component, Default)]
pub struct Shoot {
    pub cooldown: Timer,
}

fn update_movement_timer(time: Res<Time>, mut query: Query<&mut Movement>) {
    for mut timer in &mut query {
        timer.cooldown.iter_mut().for_each(|(_, timer)| {
            timer.tick(time.delta());
        });
    }
}

fn update_shoot_timer(time: Res<Time>, mut query: Query<&mut Shoot>) {
    let mut timer = query
        .get_single_mut()
        .expect("there should be a shoot-timer");
    timer.cooldown.tick(time.delta());
}

fn setup_spaceship(mut commands: Commands, asset_server: Res<AssetServer>) {
    let spaceship = Sprite {
        image: asset_server.load("spaceship.png"),
        ..default()
    };

    println!("aaa");
    commands.spawn((
        spaceship,
        Transform::from_xyz(
            1920. / 2. - WIDTH_BLOCK / 2.,
            -1080. + HEIGHT_BLOCK * 1.5,
            1.,
        ),
        Spaceship,
        Movement {
            cooldown: HashMap::from([
                (
                    KeyCode::ArrowLeft,
                    Timer::from_seconds(MOVEMENT_COOLDOWN, TimerMode::Once),
                ),
                (
                    KeyCode::ArrowRight,
                    Timer::from_seconds(MOVEMENT_COOLDOWN, TimerMode::Once),
                ),
            ]),
        },
        Shoot {
            cooldown: Timer::from_seconds(SHOOT_COOLDOWN, TimerMode::Once),
        },
    ));
}

pub fn input_spaceship(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut Movement), With<Spaceship>>,
) {
    let spaceship = query.get_single_mut().expect("not a single spaceship");
    let mut transform = spaceship.0;
    let mut timer = spaceship.1;

    let out_of_bounds_left = transform.translation.x - WIDTH_BLOCK / 2. - PADDING_HORIZONTAL < 0.;
    if keys.pressed(KeyCode::ArrowLeft) && !out_of_bounds_left {
        let cd = timer
            .cooldown
            .get_mut(&KeyCode::ArrowLeft)
            .expect("there should be a stopwatch for left arrow");

        if cd.finished() {
            transform.translation.x -= WIDTH_BLOCK / 3.;
            cd.reset();
        }
    }

    let out_of_bounds_right = transform.translation.x + WIDTH_BLOCK / 2. >= 1920.;
    if keys.pressed(KeyCode::ArrowRight) && !out_of_bounds_right {
        let cd = timer
            .cooldown
            .get_mut(&KeyCode::ArrowRight)
            .expect("there should be a stopwatch for right arrow");

        if cd.finished() {
            transform.translation.x += WIDTH_BLOCK / 3.;
            cd.reset();
        }
    }
}

pub fn shoot(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<(&Transform, &Shoot), With<Spaceship>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for (pos, shoot) in &query {
        let can_fire = shoot.cooldown.finished();
        if keys.just_pressed(KeyCode::KeyZ) && can_fire {
            commands.spawn((
                Sprite {
                    image: asset_server.load("bullet.png"),
                    ..default()
                },
                Bullet,
                Transform::from_xyz(
                    pos.translation.x,
                    pos.translation.y + HEIGHT_SPRITES / 2.,
                    1.,
                ),
            ));
        }
    }
}
