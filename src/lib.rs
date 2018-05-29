//! # Utility-Programming: A library for composable utility programming.
//!
//! This library is brought to you by
//! the [AdvancedResearch](https://github.com/advancedresearch/advancedresearch.github.io) community.
//!
//! ### Introduction
//!
//! Utility programming can be thought of as a form of programming with soft constraints.
//! Instead of programming rules explicitly, various aspects of the solution is assigned a utility.
//! An optimization algorithm then generates and modifies objects to maximize the utility.
//!
//! The advantage of utility programming is that one feature can be traded with another.
//! By adjusting the weights of utility of various features, the optimized object can
//! be modified to fit new criteria without rewriting the algorithm.
//!
//! Utility programming differs from other kinds of machine learning in the following ways:
//!
//! - The goal is to develop reusable composable components based on a common set of abstractions
//! - The utility of sub-components are often used in the strategy of higher level components
//! - Utilities are not treated as final goals, but as a technique for optimization and learning
//! - The objects to be optimized are virtual
//! - The optimization process is assumed to a have few side effects
//!
//! These properties results in higher focus on the programming aspects by adjusting utilities.
//!
//! From a library perspective, finding utility definitions that interact nicely with
//! each other is the challenge of API designers.
//!
//! ### Design
//!
//! The abstractions for utility programming consists of 3 parts:
//!
//! - `Utility` trait (implemented by objects that measure some utility)
//! - `Generator` trait (implemented by objects that generate some kind of object)
//! - `Modifier` trait (implemented by objects that modify other objects)
//!
//! All traits are implemented for `Vec<T>` where `T` implements the trait:
//!
//! - `Vec<T: Utility>` sums the utility of each sub-utility
//! - `Vec<T: Generator>` picks a random generator to generate the object
//! - `Vec<T: Modifier>` picks a random modifier to modify the object
//!
//! It is common to use `enum` instead of `struct` to combine variants with `Vec<T>`.
//!
//! Modification requires `undo` and `redo` for backtracking and replication.
//!
//! ### Utility Epistomology
//!
//! Epistomology is philosophy of knowledge.
//! Utility Epistomology means philosophy of utility knowledge.
//!
//! Utility optimization consists of 3 major efficiency levels:
//!
//! 1. Object generation (blind utility)
//! 2. Modification (target utility)
//! 3. Trade-off prediction (heuristics utility)
//!
//! The optimized solution is a result of the 3 levels above.
//! All the information that is used to evaluate the utility
//! is stored in the object of interest.
//! This means one can predict certain aspects of the optimal solution
//! from the utility parameters, generators and modifiers.
//!
//! An optimal agent operating at the 1st level is required behavior to be optimally creative.
//! It is able to think of anything that might be possible for a given problem.
//!
//! An optimal agent operating at the 2nd level is required behavior to be instrumentally rational.
//! It is able to maximize its goal in the most efficient way.
//!
//! An optimal agent operating at the 3rd level is required behavior to be zen rational.
//! It is able to predict what the optimized solution might look like if the utility was different.
//!
//! One motivation for developing tools for utility programming is to study
//! [Zen Rationality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/zen-rationality.pdf).

extern crate rand;

/// Implemented by objects that measure utility of an object.
pub trait Utility<T> {
    /// Computes the utility of an object.
    fn utility(&self, obj: &T) -> f64;
}

/// Sums up utility from multiple sub-terms.
impl<T, U: Utility<T>> Utility<T> for Vec<U> {
    fn utility(&self, obj: &T) -> f64 {
        self.iter().map(|it| it.utility(obj)).sum()
    }
}

/// Implemented by objects that generates other objects.
pub trait Generator {
    /// The type of the object generated.
    type Output;
    /// Generate a new object.
    ///
    /// This might be indeterministic.
    fn generate(&mut self) -> Self::Output;
}

impl<T: Generator> Generator for Vec<T> {
    type Output = T::Output;
    fn generate(&mut self) -> Self::Output {
        let index = rand::random::<usize>() % self.len();
        self[index].generate()
    }
}

/// Modifies objects in a way that can be reversed.
pub trait Modifier<T> {
    /// The change applied to an object.
    type Change;
    /// Modify an object and return the change.
    ///
    /// This might be indeterministic.
    fn modify(&mut self, obj: &mut T) -> Self::Change;
    /// Undo change made to an object.
    ///
    /// Required to be deterministic.
    fn undo(&mut self, change: &Self::Change, obj: &mut T);
    /// Redo change made to an object.
    ///
    /// Required to be deterministic.
    fn redo(&mut self, change: &Self::Change, obj: &mut T);
}

impl<T, U: Modifier<T>> Modifier<T> for Vec<U> {
    type Change = (usize, U::Change);
    fn modify(&mut self, obj: &mut T) -> Self::Change {
        let index = rand::random::<usize>() % self.len();
        (index, self[index].modify(obj))
    }
    fn undo(&mut self, change: &Self::Change, obj: &mut T) {
        self[change.0].undo(&change.1, obj)
    }
    fn redo(&mut self, change: &Self::Change, obj: &mut T) {
        self[change.0].redo(&change.1, obj)
    }
}

/// Modifies an object using a modifier by maximizing utility.
pub struct ModifyOptimizer<M, U> {
    /// The modifier to modify the object.
    pub modifier: M,
    /// The measured utility.
    pub utility: U,
    /// The number of tries before giving up.
    pub tries: usize,
    /// The number of repeated modifications before backtracking.
    pub depth: usize,
}

impl<T, M, U> Modifier<T> for ModifyOptimizer<M, U>
    where M: Modifier<T>, U: Utility<T>, M::Change: Clone
{
    type Change = Vec<M::Change>;
    fn modify(&mut self, obj: &mut T) -> Self::Change {
        let mut best = vec![];
        let mut best_utility: f64 = self.utility.utility(obj);
        let mut stack = vec![];
        for _ in 0..self.tries {
            for _ in 0..self.depth {
                stack.push(self.modifier.modify(obj));
                let utility = self.utility.utility(obj);
                if best_utility < utility {
                    best = stack.clone();
                    best_utility = utility;
                }
            }
            while let Some(ref action) = stack.pop() {
                self.modifier.undo(action, obj);
            }
        }
        for i in 0..best.len() {
            self.modifier.redo(&best[i], obj);
        }
        best
    }
    fn undo(&mut self, change: &Self::Change, obj: &mut T) {
        for i in (0..change.len()).rev() {
            self.modifier.undo(&change[i], obj);
        }
    }
    fn redo(&mut self, change: &Self::Change, obj: &mut T) {
        for i in 0..change.len() {
            self.modifier.redo(&change[i], obj);
        }
    }
}
