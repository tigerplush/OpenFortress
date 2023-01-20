

use bevy::prelude::*;

#[cfg(feature = "debug")]
use bevy::log::LogPlugin;


#[cfg(feature = "inspector")]
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

#[cfg(feature = "fps")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod path;
use path::*;

mod position;
use position::*;

mod map;
use map::*;

fn main() {
    let mut app = App::new();

    #[cfg(not(feature = "debug"))]
    app.add_plugins(DefaultPlugins);
    #[cfg(feature = "debug")]
    app.add_plugins(DefaultPlugins.set(LogPlugin {
        level: bevy::log::Level::DEBUG,
        ..default()
    }));
    #[cfg(feature = "inspector")]
    app.add_plugin(WorldInspectorPlugin::new())
        .register_inspectable::<Path>();
    #[cfg(feature = "fps")]
    app.add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(FrameTimeDiagnosticsPlugin::default());
    app.insert_resource(Map::generate(10, 10, 10));
    app.add_startup_system(setup);
    app.add_startup_system(spawn_dwarf);
    app.add_startup_system(spawn_food);
    app.add_system(calculate_path);
    app.add_system(animate_sprite);
    app.add_system(follow_path);
    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Dwarf;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

fn spawn_dwarf(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("Dwarf Sprite Sheet 1.3v.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 32.0), 8, 8, None, None);
    let texture_atlas_handle = texture_atlasses.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle.clone(),
            transform: Transform::from_scale(Vec3::splat(1.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
        .insert(Dwarf)
        .insert(Path::new(Position::ZERO, Position::new(5, 5, 0)))
        .insert(Name::from("Dwarf"));

    commands
    .spawn(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_xyz(-32.0, -32.0, 0.0),
        ..default()
    })
    .insert(AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
    .insert(Dwarf)
    .insert(Path::new(Position::from_world(Vec3::new(-32.0, -32.0, 0.0)), Position::new(-6, -6, 5)))
    .insert(Name::from("Dwarf 2"));
}

fn animate_sprite(
    time: Res<Time>,
    texture_atlasses: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let _texture_atlas = texture_atlasses.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % 5;
        }
    }
}

#[derive(Component)]
struct Food;

fn spawn_food(mut commands: Commands) {
    commands
        .spawn_empty()
        .insert(Food)
        .insert(Position::new(5, 5, 0))
        .insert(Name::from("Food"));
}
