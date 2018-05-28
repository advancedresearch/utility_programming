# Utility-Programming: A library for composable utility programming.

This library is brought to you by
the [AdvancedResearch](https://github.com/advancedresearch/advancedresearch.github.io) community.

### Introduction

Utility programming can be thought of as a form of programming with soft constraints.
Instead of programming rules explicitly, various aspects of the solution is assigned a utility.
An optimization algorithm then generates and modifies objects to maximize the utility.

The advantage of utility programming is that one feature can be traded with another.
By adjusting the weights of utility of various features, the optimized object can
be modified to fit new criteria without rewriting the algorithm.

Utility programming differs from other kinds of machine learning in the following ways:

- The goal is to develop reusable composable components based on a common set of abstractions
- The utility of sub-components are often used in the strategy of higher level components
- Utilities are not treated as final goals, but as a technique for optimization and learning
- The objects to be optimized are virtual
- The optimization process is assumed to a have few side effects

These properties results in higher focus on the programming aspects by adjusting utilities.

From a library perspective, finding utility definitions that interact nicely with
each other is the challenge of API designers.

### Design

The abstractions for utility programming consists of 3 parts:

- `Utility` trait (implemented by objects that measure some utility)
- `Generator` trait (implemented by objects that generate some kind of object)
- `Modifier` trait (implemented by objects that modify other objects)

All traits are implemented for `Vec<T>` where `T` implements the trait:

- `Vec<T: Utility>` sums the utility of each sub-utility
- `Vec<T: Generator>` picks a random generator to generate the object
- `Vec<T: Modifier>` picks a random modifier to modify the object

It is common to use `enum` instead of `struct` to combine variants with `Vec<T>`.

Modification requires `undo` and `redo` for backtracking and replication.

### Utility Epistomology

Epistomology is philosophy of knowledge.
Utility Epistomology means philosophy of utility knowledge.

Utility optimization consists of 3 major efficiency levels:

1. Object generation (blind utility)
2. Modification (target utility)
3. Trade-off prediction (heuristics utility)

The optimized solution is a result of the 3 levels above.
All the information that is used to evaluate the utility
is stored in the object of interest.
This means one can predict certain aspects of the optimal solution
from the utility parameters, generators and modifiers.

An optimal agent operating at the 1st level is required behavior to be optimally creative.
It is able to think of anything that might be possible for a given problem.

An optimal agent operating at the 2nd level is required behavior to be instrumentally rational.
It is able to maximize its goal in the most efficient way.

An optimal agent operating at the 3rd level is required behavior to be zen rational.
It is able to predict what the optimized solution might look like if the utility was different.

One motivation for developing tools for utility programming is to study
[Zen Rationality](https://github.com/advancedresearch/path_semantics/blob/master/papers-wip/zen-rationality.pdf).
