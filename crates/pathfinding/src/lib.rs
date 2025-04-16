use bevy::prelude::*;
use map_generation::map_generation::WorldMap;
use path::Path;
use pathfinder::{Pathfinder, PathfindingErrors, PathfindingState};

pub mod path;
pub mod pathfinder;

pub fn plugin(app: &mut App) {
    app.register_type::<Pathfinder>()
        .register_type::<Path>()
        .add_systems(Update, calculate_path.run_if(resource_exists::<WorldMap>))
        .add_systems(Update, (path::tick_path, path::follow_path).chain());
}

fn calculate_path(
    world_map: Res<WorldMap>,
    mut query: Query<(Entity, &mut Pathfinder)>,
    mut commands: Commands,
) {
    for (entity, mut path) in &mut query {
        match path.calculate_step(&world_map) {
            PathfindingState::Calculating => (),
            PathfindingState::Failed(err) => {
                info!("pathfinding failed");
                match err {
                    PathfindingErrors::NotEnoughChunks => {}
                    PathfindingErrors::Unreachable => {
                        commands.entity(entity).remove::<Pathfinder>();
                    }
                }
            }
            PathfindingState::Complete(path) => {
                info!("pathfinding done");
                commands.entity(entity).remove::<Pathfinder>().insert(path);
            }
        }
    }
}
