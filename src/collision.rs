use bevy::prelude::*;

use crate::player::Player;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, ground_detection);
    }
}


#[derive(Component, Debug, Clone, Copy)]
pub struct HitBox(pub Vec2);


pub fn check_hit(
    hitbox: HitBox, 
    offset: Vec3, 
    other_hitbox: HitBox, 
    other_offset: Vec3,
) -> bool {
    let h_size  = hitbox.0.y /2.;
    let oh_size = other_hitbox.0.y /2.;
    let w_size  = hitbox.0.x /2.;
    let ow_size = other_hitbox.0.x /2.;

    offset.x + w_size > other_offset.x - ow_size &&
    offset.x - w_size < other_offset.x + ow_size &&
    offset.y + h_size > other_offset.y - oh_size &&
    offset.y - h_size < other_offset.y + oh_size 
}


#[derive(Component, Debug)]
pub struct Grounded(pub bool);

pub fn ground_detection(
    mut player: Query<(&Transform, &mut Grounded), With<Player>>,
    mut last: Local<Transform>,
) {
    let Ok((pos, mut on_ground)) = player.get_single_mut() else {return};
    let current = if pos.translation.y == last.translation.y {
        true
    } else {
        false
    };
    if current != on_ground.0 {
        on_ground.0 = current;
    }
    *last = *pos;
}