use bevy::prelude::*;
use bevy::math::bounding::Aabb2d;

// Hitbox component: Represents an area that can cause interactions
#[derive(Component)]
pub struct Hitbox {
    pub aabb: Aabb2d,
    pub lifetime: f32, //time the hitbox is present
}

// Hurtbox component: Represents an area that can receive interactions
#[derive(Component)]
pub struct Hurtbox {
    pub aabb: Aabb2d,
}

// Colliding component: Added to entities when a collision is detected
#[derive(Component)]
pub struct Colliding;
