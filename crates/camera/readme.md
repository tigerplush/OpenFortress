# camera

The camera crate is responsible for the actual in-game camera. It should handle all camera related functions, such es input handling, zooming, panning etc. If the app leaves the game state, this crate should clean up after itself. It should also handle different Substate for the main game state.