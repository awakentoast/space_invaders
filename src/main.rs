use bevy::core::FrameCount;
use bevy::prelude::*;
use bevy::window::WindowMode;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resizable: false,
                visible: false,
                mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                make_window_visible,
                setup_entities,
                setup_camera,
            )
        )
        .add_systems(Update, (
                close_game,
                move_aliens,
            )
        )
        .insert_resource(AlienDirection { direction: Directions::Right })
        .run();
}


fn make_window_visible(mut window: Single<&mut Window>) {
    window.visible = true;
}

fn close_game(keys: Res<ButtonInput<KeyCode>>, mut app_exit_events: ResMut<Events<bevy::app::AppExit>>) {
    if keys.just_pressed(KeyCode::Escape) {
        app_exit_events.send(AppExit::Success);
    }
}

#[derive(Component)]
#[require(Sprite, Transform, Visibility)]
struct Alien;

#[derive(Resource)]
struct AlienDirection {
    direction: Directions,
}

fn setup_camera(
    mut commands: Commands,
    window: Single<&mut Window>,
) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(window.width() / 2. - 50.0, window.height() / 2. * -1. - 36.5, 0.),
    ));
}

fn setup_entities(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    const PADDING_ALIEN: f32 = 50.;

    let alien = Sprite {
        image: asset_server.load("alien.png"),
        ..default()
    };

    for y in 0..2 {
        for x in 0..11 {
            commands.spawn((
                alien.clone(),
                Transform::from_xyz(x as f32 * (100. + PADDING_ALIEN) , y as f32 * (73. + 50.), 1.),
                Alien
            ));
        }
    }
}

enum Directions {
    Left,
    Right,
}

fn move_aliens(
    mut query: Query<&mut Transform>,
    frame_count: Res<FrameCount>,
    mut alien_direction: ResMut<AlienDirection>,
) {
    if frame_count.0 % 600 != 0 {
        return
    } else {
        let min_x: f32 = query.iter()
            .map(|transform| transform.translation.x as i32)
            .min().expect("REASON") as f32;

        let max_x: f32 = query.iter()
            .map(|transform| transform.translation.x as i32)
            .max().expect("REASON") as f32;

        const STEP_WIDTH: f32 = 80.;

        println!("{} {}", min_x, max_x);
        if max_x + STEP_WIDTH > 1980. {
            alien_direction.direction = Directions::Left;
        }

        if min_x - STEP_WIDTH < 0. {
            alien_direction.direction = Directions::Right;
        }

        match alien_direction.direction {
            Directions::Right => {
                for mut transform in &mut query {
                    println!("{}", transform.translation.x);
                    transform.translation.x += STEP_WIDTH;
                    println!("{}", transform.translation.x);
                    println!("a");
                    alien_direction.direction = Directions::Right;
                };
            },
            Directions::Left => {
                for mut transform in &mut query {
                    transform.translation.x -= STEP_WIDTH;
                    alien_direction.direction = Directions::Left;
                };
            },
        }
    }
}





