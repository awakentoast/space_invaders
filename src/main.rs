mod alien;
mod bullet;
mod spaceship;

use crate::alien::{Alien, AlienPlugin, HEIGHT_BLOCK, HEIGHT_SPRITES, WIDTH_SPRITES};
use crate::bullet::{collide, BulletPlugin};
use crate::spaceship::{Spaceship, SpaceshipPlugin};
use bevy::math::bounding::Aabb2d;
use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::{Window, WindowMode, WindowResolution};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                visible: false,
                resolution: WindowResolution::new(1920., 1080.),
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (make_window_visible, close_game, game_over, win))
        .add_plugins(AlienPlugin)
        .add_plugins(SpaceshipPlugin)
        .add_plugins(BulletPlugin)
        .run();
}

fn make_window_visible(mut window: Single<&mut Window>) {
    window.visible = true;
}

fn close_game(
    keys: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

fn win(aliens: Query<&Alien>) {
    if aliens.is_empty() {
        println!("YOU WIN");
    }
}

fn setup_camera(mut commands: Commands, window: Single<&mut Window>) {
    println!(
        "{} {}",
        window.resolution.height(),
        window.resolution.width()
    );
    commands.spawn((
        Camera2d,
        Transform::from_xyz(
            window.resolution.width() / 2.,
            (window.resolution.height() / 2.) * -1. + HEIGHT_BLOCK,
            0.,
        ),
    ));
}

fn game_over(
    spaceship: Query<&Transform, (With<Spaceship>, Without<Alien>)>,
    aliens: Query<&Transform, (With<Alien>, Without<Spaceship>)>,
) {
    let spaceship = spaceship.get_single().expect("there should be a spaceship");
    for alien_center in &aliens {
        if collide(
            Aabb2d::new(
                vec2(alien_center.translation.x, alien_center.translation.y),
                vec2(WIDTH_SPRITES / 2., HEIGHT_SPRITES / 2.),
            ),
            Aabb2d::new(
                vec2(spaceship.translation.x, spaceship.translation.y),
                vec2(WIDTH_SPRITES / 2., HEIGHT_SPRITES / 2.),
            ),
        ) {
            println!("you lose");
        }
    }
}
