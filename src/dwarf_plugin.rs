use bevy::prelude::*;

use crate::{path::*, position::Position, ClaimedBy, Food};

pub struct DwarfPlugin;

impl Plugin for DwarfPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dwarf)
            .add_systems(Update, (animate_sprite, consume_food, find_food));
    }
}

#[derive(Component)]
struct Dwarf;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

fn spawn_dwarf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle = asset_server.load("Dwarf Sprite Sheet 1.3v.png");
    let texture_atlas = TextureAtlasLayout::from_grid(UVec2::new(64, 32), 8, 8, None, None);
    let texture_atlas_handle = texture_atlasses.add(texture_atlas);

    commands.spawn((
        SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_handle.clone(),
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Dwarf,
        Name::from("Dwarf"),
    ));

    commands.spawn((
        SpriteBundle {
            texture: texture_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0))
                .with_translation(Vec3::new(-32.0, -32.0, 0.0)),
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_handle.clone(),
            index: 0,
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        Dwarf,
        Name::from("Dwarf 2"),
    ));
}

fn animate_sprite(time: Res<Time>, mut query: Query<(&mut AnimationTimer, &mut TextureAtlas)>) {
    for (mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = (sprite.index + 1) % 5;
        }
    }
}

fn consume_food(
    query: Query<(Entity, &Path, &Transform)>,
    foods: Query<(Entity, &Transform), With<Food>>,
    mut commands: Commands,
) {
    for (entity, path, current_position) in &query {
        if path.state != PathState::Done {
            return;
        }

        for (food_entity, &position) in &foods {
            if Position::from_world(position.translation)
                == Position::from_world(current_position.translation)
            {
                info!("Food is on the same tile, consuming...");
                commands.entity(food_entity).despawn();
                commands.entity(entity).remove::<Path>();
            }
        }
    }
}

fn find_food(
    query: Query<(Entity, &Transform), (With<Dwarf>, Without<Path>)>,
    available_foods: Query<(Entity, &Transform), (With<Food>, Without<ClaimedBy>)>,
    mut commands: Commands,
) {
    // we only assign one dwarf to one food per frame so two dwarves don't claim the same food
    if let Some((entity, transform)) = query.iter().next() {
        let mut distance: f32 = f32::INFINITY;
        let mut target: Option<Entity> = None;
        let mut target_pos: Option<Vec3> = None;
        for (food_entity, food_transform) in &available_foods {
            let new_distance = transform.translation.distance(food_transform.translation);
            if new_distance < distance {
                distance = new_distance;
                target = Some(food_entity);
                target_pos = Some(food_transform.translation);
            }
        }
        if let Some(target_entity) = target {
            commands.entity(target_entity).insert(ClaimedBy(entity));
            commands.entity(entity).insert(Path::new(
                Position::from_world(transform.translation),
                Position::from_world(target_pos.unwrap()),
            ));
        }
    }
}
