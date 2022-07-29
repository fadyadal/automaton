use itertools::Itertools;

use crate::{
    cell::BasicCell,
    common::{Dimensions, DoubleVec, Index},
};

/// A given [BasicWorld] knows how to go from one state of [BasicCell] to the next on each
/// tick by provding a [BasicWorld::changes] method
pub trait BasicWorld<C>
where
    C: BasicCell,
    Self: Sized,
{
    /// Gives a blank [BasicWorld] with given [Dimensions], using the [BasicCell]'s [Default] value
    fn blank(dimensions: Dimensions) -> Self {
        let default = C::default();
        let cells = vec![vec![(); dimensions.0]; dimensions.1]
            .into_iter()
            .map(|row| row.into_iter().map(|_| default).collect())
            .collect();
        Self::new(cells, dimensions)
    }

    /// Gives a random [BasicWorld] of given [Dimensions]
    fn random<R: rand::Rng + ?Sized>(rng: &mut R, dimensions: Dimensions) -> Self {
        let cells = (0..dimensions.0 * dimensions.1)
            .chunks(dimensions.0)
            .into_iter()
            .map(|chunk| chunk.map(|_| C::random(rng)).collect())
            .collect();
        Self::new(cells, dimensions)
    }

    /// Constaruct a new [BasicWorld]
    fn new(cells: DoubleVec<C>, dimensions: Dimensions) -> Self;

    /// Gets a shared referene to the grid of [BasicCell]s
    fn cells(&self) -> &DoubleVec<C>;
    /// Gets a mutable referene to the grid of [BasicCell]s
    fn cells_mut(&mut self) -> &mut DoubleVec<C>;

    /// Given a [BasicWorld], return the changes to [BasicCell]s that
    /// would happned the *upcoming* tick and the [Index]s where they happened
    fn changes(&self) -> Vec<(Index, C)>;

    /// Returns the dela that happened the previous [BasicWorld::tick]
    fn delta(&self) -> &Vec<(Index, C)>;
    /// Returns a mutable reference to that value
    fn delta_mut(&mut self) -> &mut Vec<(Index, C)>;

    /// Get the dimensions of the world
    fn dimensions(&self) -> Dimensions;

    /// Commit the [BasicWorld::changes] to memory
    fn tick(&mut self) {
        let changes = self.changes();
        for ((x, y), cell) in changes.iter() {
            self.cells_mut()[*y][*x] = *cell;
        }
        *self.delta_mut() = changes;
    }

    /// Returns the Moore Neihgbors for a given [BasicCell] for a given [Index] (x, y)
    fn moore_neighbors(&self, (x, y): Index) -> Vec<&C> {
        let (x, y) = (x as isize, y as isize);
        (x.max(1) - 1..=(x + 1).min(self.dimensions().0 as isize - 1))
            .cartesian_product(y.max(1) - 1..=(y + 1).min(self.dimensions().1 as isize - 1))
            .filter(move |&item| item != (x, y))
            .map(|(x, y)| &self.cells()[y as usize][x as usize])
            .collect()
    }
}
