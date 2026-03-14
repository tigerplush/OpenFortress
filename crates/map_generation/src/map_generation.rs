use bevy::prelude::*;
use bevy_ecs_tilemap::TilemapPlugin;
use common::{constants::TILE_SIZE, states::AppState};

#[derive(Default, Reflect, Resource)]
#[reflect(Resource)]
pub struct WorldGenerationSettings {
    pub seed: u32,
}

use crate::{
    chunk_visualisation,
    messages::{BlockUpdate, UpdateMap},
    world_map::WorldMap,
};

pub fn plugin(app: &mut App) {
    app.register_type::<WorldMap>()
        .register_type::<WorldGenerationSettings>()
        .add_message::<UpdateMap>()
        .add_message::<BlockUpdate>()
        .insert_resource(ClearColor(Color::srgb_u8(50, 45, 52)))
        .add_plugins(TilemapPlugin)
        .add_plugins(chunk_visualisation::plugin)
        .add_systems(OnEnter(AppState::MainGame), spawn_world)
        .add_systems(
            Update,
            (handle_messages,).run_if(in_state(AppState::MainGame)),
        );
}

fn spawn_world(world_generation_settings: Res<WorldGenerationSettings>, mut commands: Commands) {
    let entity = commands
        .spawn((
            Name::new("World Map"),
            // Transform::default(),
            // due to an issue with bevy_ecs_tilemap, we have to move the whole world by half a tile
            Transform::from_translation((-TILE_SIZE / 2.0).extend(0.0)),
            Visibility::Inherited,
        ))
        .id();
    commands.insert_resource(WorldMap::new(entity, world_generation_settings.seed));
}

fn handle_messages(
    mut world_map: ResMut<WorldMap>,
    mut message_reader: MessageReader<UpdateMap>,
    mut message_writer: MessageWriter<BlockUpdate>,
) {
    for update_message in message_reader.read() {
        match *update_message {
            UpdateMap::DamageBlock(world_coordinates, damage) => {
                if world_map.damage_block(world_coordinates, damage) {
                    debug!("block {:?} was destroyed", world_coordinates);
                    message_writer.write(BlockUpdate::Removed(world_coordinates));
                }
            }
            UpdateMap::ScheduleForRemoval(world_coordinates) => {
                if world_map
                    .get_block(world_coordinates)
                    .is_some_and(|block| block.is_solid())
                {
                    message_writer.write(BlockUpdate::ScheduleForRemoval(world_coordinates));
                }
            }
        }
    }
}
