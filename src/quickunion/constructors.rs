//! Constructors and macros for QuickUnion

use crate::{rng::PhantomRng, UnionFind, QuickUnion, BySize, Unweighted};

macro_rules! generate_representative {
    ($n:expr, $num_type:ident) => {{
        let mut representative = [0; $n];
        for i in 0..($n as $num_type) {
            representative[i as usize] = i;
        }
        representative
    }};
}

/// Macro to generate default constructor for weighted quickunion (by rank) with path compression
macro_rules! generate_default_ctor {
    ($($num_type:ident), *) => {
        $(
        impl<const N: usize> Default for UnionFind<'_, QuickUnion, $num_type, N>
        {
            fn default() -> Self {
                Self {
                    representative: generate_representative!(N, $num_type),
                    heuristic: [0; N],
                    algorithm: Default::default(),
                    rng: PhantomRng
                }
            }
        }

        impl<const N: usize> Default for UnionFind<'_, QuickUnion<BySize>, $num_type, N>
        {
            fn default() -> Self {
                Self {
                    representative: generate_representative!(N, $num_type),
                    heuristic: [1; N],
                    algorithm: Default::default(),
                    rng: PhantomRng
                }
            }
        }

        impl<const N: usize, const PATH_COMPRESS: bool> Default for UnionFind<'_, QuickUnion<Unweighted, PATH_COMPRESS>, $num_type, N>
        {
            fn default() -> Self {
                Self {
                    representative: generate_representative!(N, $num_type),
                    heuristic: [0; 0],
                    algorithm: Default::default(),
                    rng: PhantomRng
                }
            }
        }
        )*
    };
}

generate_default_ctor!(u8, u16, u32, u64, usize);
