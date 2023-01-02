use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{Bullet, GameState, Health, Target};

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_update(GameState::Gameplay).with_system(bullet_collision_detection),
        );
    }
}

#[derive(Bundle)]
pub struct PhysicsBundle {
    flags: ActiveEvents,
    active_collition_types: ActiveCollisionTypes,
    collider: Collider,
    colliding_entities: CollidingEntities,
    rigid_body: RigidBody,
    rotation_contraint: LockedAxes,
    velocity: Velocity,
}

impl PhysicsBundle {
    pub fn moving_entity(size: Vec3) -> Self {
        Self {
            flags: ActiveEvents::COLLISION_EVENTS,
            active_collition_types: ActiveCollisionTypes::default()
                | ActiveCollisionTypes::KINEMATIC_KINEMATIC,
            collider: Collider::cuboid(size.x / 2., size.y / 2., size.z / 2.),
            colliding_entities: CollidingEntities::default(),
            rigid_body: RigidBody::KinematicPositionBased,
            rotation_contraint: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::zero(),
        }
    }
}

fn bullet_collision_detection(
    mut commands: Commands,
    bullets: Query<Entity, With<Bullet>>,
    mut targets: Query<(&mut Health, &CollidingEntities), With<Target>>,
) {
    for (mut health, colliding_entities) in &mut targets {
        for bullet_entity in &bullets {
            if colliding_entities.contains(bullet_entity) {
                commands.entity(bullet_entity).despawn_recursive();
                health.value -= 1.0;
            }
        }
    }
}
