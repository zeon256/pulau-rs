//! Constructors and macros for QuickUnion

use crate::{
    quickunion::heuristics::ByRandom, rng::PhantomRng, ByRank, BySize, Owned, QuickUnion,
    UnionFind, Unweighted,
};
use rand_core::RngCore;

/// Helper macro to generate the representative array
macro_rules! generate_representative {
    ($n:expr, $num_type:ident) => {{
        let mut representative = [0; $n];
        for i in 0..($n as $num_type) {
            representative[i as usize] = i;
        }
        representative
    }};
}

/// Macro to generate Default impls for Owned heuristics (ByRank, BySize)
macro_rules! generate_default_owned {
    ($($num_type:ident),* $(,)?) => {
        $(
            impl<const N: usize> Default for UnionFind<'_, QuickUnion<ByRank>, $num_type, N> {
                fn default() -> Self {
                    Self {
                        representative: generate_representative!(N, $num_type),
                        heuristic: [0; N],
                        algorithm: Default::default(),
                        rng: PhantomRng,
                    }
                }
            }

            impl<const N: usize> Default for UnionFind<'_, QuickUnion<BySize>, $num_type, N> {
                fn default() -> Self {
                    Self {
                        representative: generate_representative!(N, $num_type),
                        heuristic: [1; N],
                        algorithm: Default::default(),
                        rng: PhantomRng,
                    }
                }
            }
        )*
    };
}

/// Macro to generate Default impls for Unweighted heuristics
macro_rules! generate_default_unweighted {
    ($($num_type:ident),* $(,)?) => {
        $(
            impl<const N: usize, const P: bool> Default for UnionFind<'_, QuickUnion<Unweighted, P>, $num_type, N> {
                fn default() -> Self {
                    Self {
                        representative: generate_representative!(N, $num_type),
                        heuristic: [0; 0],
                        algorithm: Default::default(),
                        rng: PhantomRng,
                    }
                }
            }
        )*
    };
}

/// Macro to generate Default impls for ByRandom<R> heuristics
macro_rules! generate_default_random {
    ($($num_type:ident),* $(,)?) => {
        $(
            impl<R, const N: usize, const P: bool> UnionFind<'_, QuickUnion<ByRandom<R, Owned>, P>, $num_type, N>
            where
                R: RngCore,
            {
                pub fn new(rng: R) -> Self {
                    Self {
                        representative: generate_representative!(N, $num_type),
                        heuristic: [0; 0],
                        algorithm: Default::default(),
                        rng
                    }
                }
            }
        )*
    };
}

// ✅ Generate all Default impls for primitive integer types
generate_default_owned!(u8, u16, u32, u64, usize);
generate_default_unweighted!(u8, u16, u32, u64, usize);
generate_default_random!(u8, u16, u32, u64, usize);
