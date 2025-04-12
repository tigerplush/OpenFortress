# pathfinding

This crate is responsible for pathfinding. Right now it is an A* alghorithm using almost no optimizations. Pathfinders run once per frame and check all neighbors of a tile.

## Requirements

* Pathfinding should be sensible. It doesn't need to find the most optimal route if it is a good route.
* Pathfinding should not escalate resources. When a path isn't found for several tries, instead of requesting more of the world it should just fail.
* If pathfinding fails it should emit an event with the failure reason where different systems can observe that failure and decide what to do with it.
* Pathfinding must be able to find the nearest point for some of the work orders (e.g. mining: a mining work order should be fulfilled from a tile neighboring the actual target)