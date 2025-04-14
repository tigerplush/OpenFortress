use bevy::prelude::*;

pub trait Neighbors<T> {
    /// Returns all neighbors on the same layer with their squared cost.
    ///
    /// The order is:
    /// ```
    /// NW, N, NE,
    ///  W,     E,
    /// SW, S, SE
    /// ```
    fn same_layer_neighbors(&self) -> Vec<(T, u32)>;

    /// Returns all neighbors with their squared cost.
    /// Order is like in [`Self::same_layer_neighbors`] but first on the layer above, then the same layer, then the layer below.
    fn all_neighbors(&self) -> Vec<(T, u32)>;
}

#[rustfmt::skip]
impl Neighbors<IVec3> for IVec3 {
    fn same_layer_neighbors(&self) -> Vec<(IVec3, u32)> {
        vec![
            (self + IVec3::new(-1,  1,  0), 2), (self + IVec3::new( 0,  1,  0), 1), (self + IVec3::new( 1,  1,  0), 2),
            (self + IVec3::new(-1,  0,  0), 1),                                     (self + IVec3::new( 1,  0,  0), 1),
            (self + IVec3::new(-1, -1,  0), 2), (self + IVec3::new( 0, -1,  0), 1), (self + IVec3::new( 1, -1,  0), 2),
        ]
    }

    fn all_neighbors(&self) -> Vec<(IVec3, u32)> {
        vec![
            // layer above
            (self + IVec3::new(-1,  1,  1), 3), (self + IVec3::new( 0,  1,  1), 2), (self + IVec3::new( 1,  1,  1), 3),
            (self + IVec3::new(-1,  0,  1), 2), (self + IVec3::new( 0,  0,  1), 1), (self + IVec3::new( 1,  0,  1), 2),
            (self + IVec3::new(-1, -1,  1), 3), (self + IVec3::new( 0, -1,  1), 2), (self + IVec3::new( 1, -1,  1), 3),
            // same layer
            (self + IVec3::new(-1,  1,  0), 2), (self + IVec3::new( 0,  1,  0), 1), (self + IVec3::new( 1,  1,  0), 2),
            (self + IVec3::new(-1,  0,  0), 1),                                     (self + IVec3::new( 1,  0,  0), 1),
            (self + IVec3::new(-1, -1,  0), 2), (self + IVec3::new( 0, -1,  0), 1), (self + IVec3::new( 1, -1,  0), 2),
            // layer below
            (self + IVec3::new(-1,  1, -1), 3), (self + IVec3::new( 0,  1, -1), 2), (self + IVec3::new( 1,  1, -1), 3),
            (self + IVec3::new(-1,  0, -1), 2), (self + IVec3::new( 0,  0, -1), 1), (self + IVec3::new( 1,  0, -1), 2),
            (self + IVec3::new(-1, -1, -1), 3), (self + IVec3::new( 0, -1, -1), 2), (self + IVec3::new( 1, -1, -1), 3),
        ]
    }
}
