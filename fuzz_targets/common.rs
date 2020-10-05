//! Adopted from https://github.com/libra/libra/tree/master/testsuite/libra-fuzzer
//!

use proptest::{
    strategy::{Strategy, ValueTree},
    test_runner::TestRunner,
};
// credit libra-fuzzer

#[derive(Default)]
pub struct SeedGen {
    runner: TestRunner,
}

impl SeedGen {
    /// Creates a new value generator with the default RNG.
    pub fn new() -> Self {
        Default::default()
    }

    /// Creates a new value generator with a deterministic RNG.
    pub fn deterministic() -> Self {
        Self {
            runner: TestRunner::deterministic(),
        }
    }

    pub fn generate<S: Strategy>(&mut self, strategy: S) -> S::Value {
        strategy
            .new_tree(&mut self.runner)
            .expect("Some went wrong with generating seeds!")
            .current()
    }
}
