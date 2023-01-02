use crate::{GameAssets, GameState, PhysicsBundle};
use bevy::{math::Vec3Swizzles, prelude::*};

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
    pub path_index: usize,
}

#[derive(Resource)]
pub struct TargetPath {
    pub waypoints: Vec<Vec2>,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: f32,
}

pub struct TargetPlugin;

pub struct TargetDeathEvent;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_targets))
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(move_targets)
                    .with_system(target_death),
            )
            .insert_resource(TargetPath {
                waypoints: vec![
                    Vec2::new(6.0, 2.0),
                    Vec2::new(30.0, 10.0),
                    Vec2::new(50.0, 1.0),
                ],
            })
            .add_event::<TargetDeathEvent>();
    }
}

fn spawn_targets(mut commands: Commands, game_assets: Res<GameAssets>) {
    for i in 0..20 {
        let x: f32 = 2.0 * i as f32;
        commands
            .spawn(SceneBundle {
                scene: game_assets.target_scene.clone(),
                transform: Transform::from_xyz(-x, 0.0, 3.0),
                ..default()
            })
            .insert(PhysicsBundle::moving_entity(Vec3::new(0.4, 0.4, 0.4)))
            .insert(Target {
                speed: 0.3,
                ..default()
            })
            .insert(Health { value: 3.0 })
            .insert(Name::new("Target"));
    }
}

fn target_death(
    mut commands: Commands,
    targets: Query<(Entity, &Health)>,
    mut death_event_writer: EventWriter<TargetDeathEvent>,
) {
    for (entity, target_health) in &targets {
        if target_health.value <= 0.0 {
            death_event_writer.send(TargetDeathEvent);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_targets(
    mut targets: Query<(&mut Target, &mut Transform)>,
    path: Res<TargetPath>,
    time: Res<Time>,
) {
    for (mut target, mut transform) in &mut targets {
        let delta = target.speed * time.delta_seconds();
        let delta_target = path.waypoints[target.path_index] - transform.translation.xz();

        if delta_target.length().round() > delta {
            let movement = delta_target.normalize() * delta;
            transform.translation += movement.extend(0.0).xzy();
            let y = transform.translation.y;
            transform.look_at(path.waypoints[target.path_index].extend(y).xzy(), Vec3::Y);
        } else {
            //At current step
            target.path_index += 1;
        }
        transform.translation.x += target.speed * time.delta_seconds();
    }
}
