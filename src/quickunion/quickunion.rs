
//! Quick Union implementation (split into modules)

pub mod heuristics {
    pub mod unweighted;
    pub mod by_rank;
    pub mod by_size;
    pub mod by_random;
}
pub mod algorithm_container;
pub mod constructors;
pub mod borrowed_impls;
pub mod traits_impl;
