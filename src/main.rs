mod bullet;
mod main_menu;
mod physics;
mod player;
mod target;
mod tower;

pub use bullet::*;
pub use main_menu::*;
pub use physics::*;
pub use player::*;
use std::fmt::Debug;
use std::hash::Hash;
pub use target::*;
pub use tower::*;

use bevy::prelude::*;
use bevy_inspector_egui::WorldInspectorPlugin;
use bevy_mod_picking::*;
use bevy_rapier3d::prelude::{NoUserData, RapierPhysicsPlugin};

pub const HEIGHT: f32 = 720.0;
pub const WIDTH: f32 = 1000.0;

#[derive(Resource)]
pub struct GameAssets {
    tower_base_scene: Handle<Scene>,
    tower_a_scene: Handle<Scene>,
    tower_b_scene: Handle<Scene>,
    tower_c_scene: Handle<Scene>,
    tower_a_icon: Handle<Image>,
    tower_b_icon: Handle<Image>,
    tower_c_icon: Handle<Image>,
    tower_a_bullet_scene: Handle<Scene>,
    tower_b_bullet_scene: Handle<Scene>,
    tower_c_bullet_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Gameplay,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: WIDTH,
                height: HEIGHT,
                title: "Bevy Tower Defense".to_string(),
                resizable: true,
                monitor: MonitorSelection::Primary,
                ..Default::default()
            },
            ..default()
        }))
        // Inspector
        .add_plugin(WorldInspectorPlugin::new())
        // Physics
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin {
        //     always_on_top: true,
        //     ..default()
        // })
        // Picking
        .add_plugins(DefaultPickingPlugins)
        // .add_system(debug_what_is_selected)
        // Our State
        .add_state(GameState::MainMenu)
        // Our systems
        .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_basic_scene))
        .add_startup_system(spawn_camera)
        .add_startup_system_to_stage(StartupStage::PreStartup, asset_loading)
        .add_system(camera_controls)
        .add_plugin(PlayerPlugin)
        .add_plugin(BulletPlugin)
        .add_plugin(TargetPlugin)
        .add_plugin(TowerPlugin)
        .add_plugin(PhysicsPlugin)
        .add_plugin(MainMenuPlugin)
        .run()
}

// fn debug_what_is_selected(selection: Query<(&Name, &Selection)>) {
//     for (name, selection) in &selection {
//         if selection.selected() {
//             info!("{}", name);
//         }
//     }
// }

fn camera_controls(
    keyboard: Res<Input<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();
    let mut forward = camera.forward();
    let left = camera.left();
    forward.y = 0.0;
    forward = forward.normalize();

    let speed = 3.0;
    let rotate_speed = 1.0;
    if keyboard.pressed(KeyCode::W) {
        camera.translation += forward * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::S) {
        camera.translation -= forward * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::A) {
        camera.translation += left * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::D) {
        camera.translation -= left * speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Q) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::E) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        tower_base_scene: assets
            .load("tower-defense-kit-1/Models/GLTFformat/towerSquare_bottomA.glb#Scene0"),
        tower_a_scene: assets
            .load("tower-defense-kit-1/Models/GLTFformat/towerSquare_sampleA.glb#Scene0"),
        tower_b_scene: assets
            .load("tower-defense-kit-1/Models/GLTFformat/towerSquare_sampleB.glb#Scene0"),
        tower_c_scene: assets
            .load("tower-defense-kit-1/Models/GLTFformat/towerSquare_sampleC.glb#Scene0"),
        tower_a_icon: assets.load("tower-defense-kit-1/Side/towerSquare_sampleA.png"),
        tower_b_icon: assets.load("tower-defense-kit-1/Side/towerSquare_sampleB.png"),
        tower_c_icon: assets.load("tower-defense-kit-1/Side/towerSquare_sampleC.png"),
        tower_a_bullet_scene: assets
            .load("kay-kit-dungeon-pack1.0/Models/gltf/arrow-flippedx.gltf.glb#Scene0"),
        tower_b_bullet_scene: assets
            .load("kay-kit-dungeon-pack1.0/Models/gltf/arrow-flippedx.gltf.glb#Scene0"),
        tower_c_bullet_scene: assets
            .load("kay-kit-dungeon-pack1.0/Models/gltf/arrow-flippedx.gltf.glb#Scene0"),
        target_scene: assets
            .load("kay-kit-dungeon-pack1.0/Models/Characters/gltf/barbarian.glb#Scene0"),
    });
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(PickingCameraBundle::default());
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 50.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..default()
        })
        .insert(Name::new("Ground"));
    commands
        .spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(4.0, 8.0, 4.0),
            ..default()
        })
        .insert(Name::new("Light"));
}
