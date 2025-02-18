mod alien;
mod bullet;
mod misc;
mod spaceship;

use bevy::prelude::*;
use bevy::time::Stopwatch;
use bevy::window::{Window, WindowMode, WindowResolution};

use alien::*;
use misc::*;
use spaceship::*;

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
        .add_systems(
            Startup,
            (
                make_window_visible,
                setup_aliens,
                setup_camera,
                setup_spaceship,
            ),
        )
        .add_systems(Update, (close_game, move_aliens, input_spaceship))
        .insert_resource(RowIndex { row: 0 })
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

fn setup_aliens(mut commands: Commands, asset_server: Res<AssetServer>) {
    let alien = Sprite {
        image: asset_server.load("alien.png"),
        ..default()
    };

    for y in 0..2 {
        for x in 0..11 {
            commands.spawn((
                alien.clone(),
                Transform::from_xyz(
                    x as f32 * WIDTH_BLOCK + PADDING_HORIZONTAL + WIDTH_SPRITES / 2.,
                    y as f32 * HEIGHT_BLOCK - PADDING_VERTICAL - HEIGHT_SPRITES / 2.,
                    1.,
                ),
                Alien,
                Row { row: y },
            ));
        }
    }
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
        MovementCooldown {
            button: KeyCode::ArrowLeft,
            stopwatch: Stopwatch::new(),
        },
        MovementCooldown {
            button: KeyCode::ArrowLeft,
            stopwatch: Stopwatch::new(),
        },
    ));
}
