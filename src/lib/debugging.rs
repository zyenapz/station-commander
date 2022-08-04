use bevy::{prelude::*, sprite::collide_aabb::collide};

use super::{asteroid::Asteroid, defines::WND_RES, player::Player};

pub fn setup_debug_cordon(mut commands: Commands) {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::BLACK,
            custom_size: Some(Vec2::new(WND_RES.0, WND_RES.1)),
            ..default()
        },
        transform: Transform::from_xyz(0., 0., 0.),
        ..default()
    });
}

pub fn c_bullet_asteroid(
    b_query: Query<(Entity, &Sprite, &Transform), With<Player>>,
    a_query: Query<(Entity, &Sprite, &Transform), With<Asteroid>>,
    mut commands: Commands,
) {
    for (bul_ent, bul_spr, bul_trans) in b_query.iter() {
        for (ast_ent, ast_spr, ast_trans) in a_query.iter() {
            let collision = collide(
                bul_trans.translation,
                bul_spr.custom_size.unwrap(),
                ast_trans.translation,
                ast_spr.custom_size.unwrap(),
            );

            if let Some(_) = collision {
                commands.entity(bul_ent).despawn();
                commands.entity(ast_ent).despawn();
            }
        }
    }
}
