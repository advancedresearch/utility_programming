/*

utility_programming: number example
==============================================
A simple example demonstrating how to use API.

Utilty Programming is about finding the balance between various features.

You are not programming in the conventional sense, instead the problem
is to figure out what happens when you optimize in a certain way.

For example, if you give a reward for finding a prime number,
then you are more likely to get a prime number,
but it depends on whether there are other conflicting features.

*/

extern crate utility_programming as up;
extern crate rand;

use up::{Generator, Modifier, ModifyOptimizer, Utility};

/// Computes utility of a number.
pub enum NumberUtility {
    /// Targets a specific number value.
    ///
    /// `penalty` means that the utility usually is negative.
    Target {value: u8, penalty: f64},
    /// A reward is given if the number is a prime.
    ///
    /// `reward` means that the utility usually is positive.
    Prime {reward: f64},
}

impl Utility<u8> for NumberUtility {
    fn utility(&self, obj: &u8) -> f64 {
        match *self {
            NumberUtility::Target {value, penalty} => {
                // Assign absolute difference a penalty.
                (*obj as f64 - value as f64).abs() * penalty
            }
            NumberUtility::Prime {reward} => {
                if *obj < 2 {return 0.0};
                for i in 2..*obj {
                    if (*obj % i) == 0 {return 0.0};
                }
                reward
            }
        }
    }
}

/// Generates a number.
pub enum NumberGenerator {
    /// Generate a random number.
    Random,
    /// Start at a fixed value.
    Fixed(u8),
}

impl Generator for NumberGenerator {
    type Output = u8;
    fn generate(&mut self) -> Self::Output {
        match *self {
            NumberGenerator::Random => rand::random::<u8>(),
            NumberGenerator::Fixed(val) => val,
        }
    }
}

/// Modifies a number.
pub enum NumberModifier {
    /// Increments the number.
    Inc,
    /// Decrements the number.
    Dec,
}

/// Stores a number change.
///
/// This is used to `undo` and `redo` modifications
/// when looking for a better match.
#[derive(Copy, Clone)]
pub struct NumberChange {
    old: u8,
    new: u8,
}

impl Modifier<u8> for NumberModifier {
    type Change = NumberChange;
    fn modify(&mut self, obj: &mut u8) -> Self::Change {
        let old = *obj;
        let new = match *self {
            NumberModifier::Inc => if *obj < 255 {*obj + 1} else {*obj},
            NumberModifier::Dec => if *obj > 0 {*obj - 1} else {*obj},
        };
        *obj = new;
        NumberChange {old, new}
    }
    fn undo(&mut self, change: &Self::Change, obj: &mut u8) {
        *obj = change.old;
    }
    fn redo(&mut self, change: &Self::Change, obj: &mut u8) {
        *obj = change.new;
    }
}

fn main() {
    // Generate a number.
    // A random generator is picked when using a list of generators.
    let mut num = vec![
        NumberGenerator::Random,
        NumberGenerator::Fixed(100),
        NumberGenerator::Fixed(0),
    ].generate();

    let target = 42;

    println!("Starting at: {}", num);
    let mut optimizer = ModifyOptimizer {
        modifier: vec![
            NumberModifier::Inc,
            NumberModifier::Dec
        ],
        utility: vec![
            NumberUtility::Target {value: target, penalty: -1.0},
            NumberUtility::Prime {reward: 5.0},
        ],
        // Make sure that the optimizer is likely to make progress when possible.
        depth: 20,
        tries: 1000,
    };
    loop {
        println!("{}, utility {}", num, optimizer.utility.utility(&num));
        let old = num;
        optimizer.modify(&mut num);
        if num == old {break}
    }
}
