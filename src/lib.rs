//! Calculates fast for
//! * all R(1, _)
//! * no larger than R(2, 6)
//! * no larger R(3, 3)

mod graph;
mod ramsey;
mod and_eq;

// use rayon::prelude::*;

pub use graph::{add, next, Graph, MAX_GRAPH_VERTICES};
pub use ramsey::ramsey;
pub use and_eq::{print_enabled, simd_and_eq, simd_and_eq_zero, i256};
