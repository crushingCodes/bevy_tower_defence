use crate::{Bullet, GameAssets, GameState, Lifetime, PhysicsBundle, Player, Target};
use bevy::ecs::query::QuerySingleError;
use bevy::prelude::shape::Capsule;
use bevy::prelude::*;
use bevy::utils::FloatOrd;
use bevy_inspector_egui::Inspectable;
use bevy_mod_picking::*;

#[derive(Component)]
pub struct TowerUIRoot;

pub struct TowerPlugin;
impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Tower>()
            .add_system_set(SystemSet::on_enter(GameState::Gameplay).with_system(spawn_tower_bases))
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(create_ui_on_selection)
                    .with_system(grey_tower_buttons.after(create_ui_on_selection))
                    .with_system(tower_button_clicked)
                    .with_system(tower_shooting),
            );
    }
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Tower {
    pub shooting_timer: Timer,
    pub bullet_offset: Vec3,
    pub range: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TowerButtonAttributes {
    pub cost: u32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct TowerButtonState {
    cost: u32,
    affordable: bool,
}

#[derive(Inspectable, Component, Clone, Copy, Debug)]
pub enum TowerType {
    TowerA,
    TowerB,
    TowerC,
}

impl TowerType {
    fn get_tower(&self, assets: &GameAssets) -> (Handle<Scene>, Tower) {
        match self {
            TowerType::TowerA => (
                assets.tower_a_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                    range: 4.5,
                },
            ),
            TowerType::TowerB => (
                assets.tower_b_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                    range: 4.5,
                },
            ),
            TowerType::TowerC => (
                assets.tower_c_scene.clone(),
                Tower {
                    shooting_timer: Timer::from_seconds(0.5, TimerMode::Repeating),
                    bullet_offset: Vec3::new(0.0, 0.6, 0.0),
                    range: 4.5,
                },
            ),
        }
    }

    fn get_tower_button(&self, assets: &GameAssets) -> (Handle<Image>, TowerButtonAttributes) {
        match self {
            TowerType::TowerA => (
                assets.tower_a_icon.clone(),
                TowerButtonAttributes { cost: 50 },
            ),
            TowerType::TowerB => (
                assets.tower_b_icon.clone(),
                TowerButtonAttributes { cost: 80 },
            ),
            TowerType::TowerC => (
                assets.tower_c_icon.clone(),
                TowerButtonAttributes { cost: 110 },
            ),
        }
    }

    fn get_bullet(&self, direction: Vec3, assets: &GameAssets) -> (Handle<Scene>, Bullet) {
        match self {
            TowerType::TowerA => (
                assets.tower_a_bullet_scene.clone(),
                Bullet {
                    direction,
                    speed: 3.5,
                },
            ),
            TowerType::TowerB => (
                assets.tower_b_bullet_scene.clone(),
                Bullet {
                    direction,
                    speed: 3.5,
                },
            ),
            TowerType::TowerC => (
                assets.tower_c_bullet_scene.clone(),
                Bullet {
                    direction,
                    speed: 3.5,
                },
            ),
        }
    }
}

fn spawn_tower_bases(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    for i in 0..20 {
        let x: f32 = 6.0 * i as f32;
        let z = match i % 2 == 0 {
            true => 6.0,
            false => 0.0,
        };
        let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3).into());
        let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9).into());
        commands
            .spawn(SpatialBundle::from_transform(Transform::from_xyz(
                x, 0.0, z,
            )))
            .insert(Name::new("Tower base"))
            .insert(meshes.add(Capsule::default().into()))
            .insert(default_collider_color.clone())
            .insert(Highlighting {
                initial: default_collider_color,
                hovered: Option::from(selected_collider_color.clone()),
                pressed: Option::from(selected_collider_color.clone()),
                selected: Option::from(selected_collider_color),
            })
            .insert(PickableBundle::default())
            .with_children(|commands| {
                commands.spawn(SceneBundle {
                    scene: game_assets.tower_base_scene.clone(),
                    transform: Transform::from_xyz(0.0, -0.5, 0.0),
                    ..default()
                });
            });
    }
}

fn create_ui_on_selection(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    selections: Query<&Selection>,
    root: Query<Entity, With<TowerUIRoot>>,
) {
    let at_least_one_selected = selections.iter().any(|selection| selection.selected());
    match root.get_single() {
        Ok(root) => {
            if !at_least_one_selected {
                commands.entity(root).despawn_recursive();
            }
        }
        // No root exists
        Err(QuerySingleError::NoEntities(..)) => {
            if at_least_one_selected {
                create_ui(commands, &game_assets);
            }
        }
        _ => unreachable!("Too manage ui tower roots!"),
    }
}

fn create_ui(mut commands: Commands, game_assets: &GameAssets) {
    let tower_types = [TowerType::TowerA, TowerType::TowerB, TowerType::TowerC];
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: Color::NONE.into(),
            ..default()
        })
        .insert(TowerUIRoot)
        .with_children(|commands| {
            for tower_type in tower_types {
                let (tower_icon, tower_attributes) = tower_type.get_tower_button(game_assets);
                commands
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(15.0 * 9.0 / 16.0), Val::Percent(15.0)),
                            align_self: AlignSelf::FlexEnd,
                            margin: UiRect::all(Val::Percent(2.0)),
                            ..default()
                        },
                        image: tower_icon.into(),
                        ..default()
                    })
                    .insert(TowerButtonState {
                        cost: tower_attributes.cost,
                        // Maintained in a different system after this system
                        affordable: false,
                    })
                    .insert(tower_type);
            }
        });
}

fn grey_tower_buttons(
    mut buttons: Query<(&mut BackgroundColor, &mut TowerButtonState)>,
    player: Query<&Player>,
) {
    let player = player.single();
    for (mut tint, mut state) in &mut buttons {
        if player.money > state.cost {
            state.affordable = true;
            *tint = Color::WHITE.into();
        } else {
            state.affordable = false;
            *tint = Color::GRAY.into();
        }
    }
}

fn tower_button_clicked(
    mut commands: Commands,
    interactions: Query<(&Interaction, &TowerType, &TowerButtonState), Changed<Interaction>>,
    selection: Query<(Entity, &Selection, &Transform)>,
    assets: Res<GameAssets>,
    mut player: Query<&mut Player>,
) {
    let mut player = player.single_mut();
    for (interaction, tower_type, button_state) in &interactions {
        if matches!(interaction, Interaction::Clicked) {
            for (entity, selection, transform) in &selection {
                if selection.selected() {
                    if player.money >= button_state.cost {
                        player.money -= button_state.cost;
                        commands.entity(entity).despawn_recursive();
                        spawn_tower(&mut commands, &assets, transform.translation, tower_type);
                    }
                }
            }
        }
    }
}

fn spawn_tower(
    commands: &mut Commands,
    assets: &GameAssets,
    position: Vec3,
    tower_type: &TowerType,
) -> Entity {
    let (model, tower) = tower_type.get_tower(assets);
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_translation(
            position,
        )))
        .insert(Name::new("Tower_1"))
        .insert(*tower_type)
        .insert(tower)
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: model,
                transform: Transform::from_xyz(0.0, -0.5, 0.0),
                ..default()
            });
        })
        .id()
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<(Entity, &mut Tower, &TowerType, &GlobalTransform)>,
    targets: Query<&GlobalTransform, With<Target>>,
    assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for (tower_ent, mut tower, tower_type, transform) in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let bullet_spawn = transform.translation() + tower.bullet_offset;

            let direction = targets
                .iter()
                .filter(|target_transform| {
                    Vec3::distance(target_transform.translation(), bullet_spawn) < tower.range
                })
                .min_by_key(|target_transform| {
                    FloatOrd(Vec3::distance(target_transform.translation(), bullet_spawn))
                })
                .map(|closest_target| closest_target.translation() - bullet_spawn);

            if let Some(direction) = direction {
                let (model, bullet) = tower_type.get_bullet(direction, &assets);
                commands.entity(tower_ent).with_children(|commands| {
                    commands
                        .spawn(SceneBundle {
                            scene: model,
                            transform: Transform::from_translation(tower.bullet_offset)
                                .looking_at(bullet.direction, Vec3::Y),
                            ..Default::default()
                        })
                        .insert(Lifetime {
                            timer: Timer::from_seconds(1000.5, TimerMode::Once),
                        })
                        .insert(bullet)
                        .insert(PhysicsBundle::moving_entity(Vec3::new(0.2, 0.2, 0.2)))
                        .insert(Name::new("Bullet"));
                });
            }
        }
    }
}
