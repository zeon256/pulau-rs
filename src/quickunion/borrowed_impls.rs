//! Borrowed and Thin borrowed implementations for QuickUnion

use crate::{
    quickunion::heuristics::ByRandom, rng::PhantomRng, Borrowed, ByRank, BySize, Fat, QuickUnion,
    Thin, UnionFind, Unweighted, VertexType,
};
use rand_core::RngCore;

/// Implements `UnionFind::new` for Borrowed<Fat> and Borrowed<Thin> variants.
macro_rules! impl_unionfind_borrowed {
    ($($heur:ident),* $(,)?) => {
        $(
            impl<'a, T, const N: usize, const P: bool>
                UnionFind<'a, QuickUnion<$heur<Borrowed<Fat>>, P>, T, N>
            where
                T: VertexType,
            {
                pub fn new(representative: &'a mut [T], heuristic: &'a mut [usize]) -> Self {
                    debug_assert!(representative.len() >= N);
                    debug_assert!(heuristic.len() >= N);
                    Self {
                        representative,
                        heuristic,
                        algorithm: Default::default(),
                        rng: PhantomRng,
                    }
                }
            }

            impl<'a, T, const N: usize, const P: bool>
                UnionFind<'a, QuickUnion<$heur<Borrowed<Thin>>, P>, T, N>
            where
                T: VertexType,
            {
                pub fn new(representative: &'a mut [T; N], heuristic: &'a mut [usize; N]) -> Self {
                    Self {
                        representative,
                        heuristic,
                        algorithm: Default::default(),
                        rng: PhantomRng,
                    }
                }
            }
        )*
    };
}

/// Implements `UnionFind::new` for Unweighted borrowed variants.
macro_rules! impl_unionfind_unweighted_borrowed {
    () => {
        impl<'a, T, const N: usize, const P: bool>
            UnionFind<'a, QuickUnion<Unweighted<Borrowed<Fat>>, P>, T, N>
        where
            T: VertexType,
        {
            pub fn new(representative: &'a mut [T]) -> Self {
                Self {
                    representative,
                    heuristic: [0; 0],
                    algorithm: Default::default(),
                    rng: PhantomRng,
                }
            }
        }

        impl<'a, T, const N: usize, const P: bool>
            UnionFind<'a, QuickUnion<Unweighted<Borrowed<Thin>>, P>, T, N>
        where
            T: VertexType,
        {
            pub fn new(representative: &'a mut [T; N]) -> Self {
                Self {
                    representative,
                    heuristic: [0; 0],
                    algorithm: Default::default(),
                    rng: PhantomRng,
                }
            }
        }
    };
}

/// Implements `UnionFind::new` for ByRandom<R, Borrowed<Fat>>.
macro_rules! impl_unionfind_random_borrowed {
    () => {
        impl<'a, R, T, const N: usize, const P: bool>
            UnionFind<'a, QuickUnion<ByRandom<R, Borrowed<Fat>>, P>, T, N>
        where
            T: VertexType,
            R: RngCore + AsMut<R> + AsRef<R>,
        {
            pub fn new(representative: &'a mut [T], heuristic: &'a mut [usize], rng: R) -> Self {
                debug_assert!(representative.len() >= N);
                debug_assert!(heuristic.len() >= N);
                Self {
                    representative,
                    heuristic,
                    algorithm: Default::default(),
                    rng,
                }
            }
        }

        impl<'a, R, T, const N: usize, const P: bool>
            UnionFind<'a, QuickUnion<ByRandom<R, Borrowed<Thin>>, P>, T, N>
        where
            T: VertexType,
            R: RngCore + AsMut<R> + AsRef<R>,
        {
            pub fn new(
                representative: &'a mut [T; N],
                heuristic: &'a mut [usize; N],
                rng: R,
            ) -> Self {
                Self {
                    representative,
                    heuristic,
                    algorithm: Default::default(),
                    rng,
                }
            }
        }
    };
}

impl_unionfind_borrowed!(ByRank, BySize);
impl_unionfind_unweighted_borrowed!();
impl_unionfind_random_borrowed!();
