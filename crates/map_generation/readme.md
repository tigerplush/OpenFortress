# map_generation

This crate is responsible for generating and maintaining the maps.

## Visualisation and rendering
* Every chunk should be rendered as a single Sprite/Mesh
* Starting at the current layer every tile is checked downards (negative z direction). If a solid tile is hit, that tile will be rendered
* On every tile there is a fog, starting at 0% opacity. With each layer downwards, the opacity rises by 10%, so that no more than 11 z layers have to be checked
* When current block is solid and the block above it is empty, it should be shown as a floor tile
* When the current block is solid and the block above it is solid, it should be rendered as per the 8bit bitmask rules

## 8 bit bitmask tiling
