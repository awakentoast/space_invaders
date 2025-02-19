use crate::alien::*;
use bevy::math::bounding::{Aabb2d, IntersectsVolume};
use bevy::math::vec2;
use bevy::prelude::*;

pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_bullet_pos, check_for_hit));
    }
}

#[derive(Component)]
#[require(Sprite)]
pub struct Bullet;

const BULLET_VELOCITY: f32 = 1.;

pub fn update_bullet_pos(mut query: Query<(&Bullet, &mut Transform)>) {
    query.iter_mut().for_each(|(_, mut transform)| {
        transform.translation.y += BULLET_VELOCITY;
    })
}

pub fn check_for_hit(
    mut bullets: Query<(Entity, &mut Transform), (With<Bullet>, Without<Alien>)>,
    mut aliens: Query<(Entity, &mut Transform), (With<Alien>, Without<Bullet>)>,
    mut commands: Commands,
) {
    for (bullet_entity, bullet_center) in &mut bullets {
        for (alien_entity, alien_center) in &mut aliens {
            if collide(
                Aabb2d::new(
                    vec2(bullet_center.translation.x, bullet_center.translation.y),
                    vec2(1., 0.5),
                ),
                Aabb2d::new(
                    vec2(alien_center.translation.x, alien_center.translation.y),
                    vec2(WIDTH_SPRITES / 2., HEIGHT_SPRITES / 2.),
                ),
            ) {
                println!("intersects");
                commands.entity(bullet_entity).despawn();
                commands.entity(alien_entity).despawn();
            }
        }
    }
}

pub fn collide(a: Aabb2d, b: Aabb2d) -> bool {
    a.intersects(&b)
}
