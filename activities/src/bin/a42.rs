use std::borrow::BorrowMut;

// Topic: Implementing Iterator
//
// Summary:
// A game uses a scoring system that includes a score multiplier.
// The multiplier starts at 1 and increases by 1 each iteration.
// The amount the multiplier increases each iteration can be
// adjusted through in-game powerups.
//
// Example multiplier progression:
// 1, 2, 3, (+1 powerup obtained), 5, 7, 9, ...
//
// Requirements:
// * Write a program that uses an iterator to generate a score multiplier
// * The iterator must start at 1 and increase by 1 each iteration
//   * It must be possible to increase the per-iteration amount through powerups
//
// Notes:
// * Use the .next() method to advance the iterator to confirm it works correctly
// * Only the Iterator trait needs to be implemented for this activity
struct Score {
    multiplier: usize,
    per_iteration: usize,
}
impl Score {
    fn new() -> Self {
        Self {
            multiplier: 1,
            per_iteration: 1,
        }
    }
    fn powerup(&mut self) {
        self.per_iteration += 1;
    }
}

impl Iterator for Score {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.multiplier += self.per_iteration;
        Some(self.multiplier)
    }
}
fn main() {
    let mut score = Score::new();
    score.borrow_mut().take(10).map(|x| println!("{}", x)).for_each(drop);
    println!("final score: {}", score.multiplier);
}
