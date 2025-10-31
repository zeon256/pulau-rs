//! Borrowed and Thin borrowed implementations for QuickUnion

use crate::{
    quickunion::heuristics::ByRandom, rng::PhantomRng, Borrowed, ByRank, BySize, QuickUnion, Thin,
    UnionFind, Unweighted, VertexType,
};
use rand_core::RngCore;

impl<'a, T, const N: usize, const PATH_COMPRESS: bool>
    UnionFind<'a, QuickUnion<BySize<Borrowed>, PATH_COMPRESS>, T, N>
where
    T: VertexType,
{
    pub fn new(representative: &'a mut [T], heuristic: &'a mut [usize]) -> Self {
        Self {
            representative,
            heuristic,
            algorithm: Default::default(),
            rng: PhantomRng,
        }
    }
}

impl<'a, T, const N: usize, const P: bool> UnionFind<'a, QuickUnion<ByRank<Borrowed>, P>, T, N>
where
    T: VertexType,
{
    pub fn new(representative: &'a mut [T], heuristic: &'a mut [usize]) -> Self {
        debug_assert!(
            representative.len() >= N,
            "Representative slice must have at least len >= N!"
        );
        debug_assert!(
            heuristic.len() >= N,
            "Heuristic slice must have at least len >= N!"
        );

        Self {
            representative,
            heuristic,
            algorithm: Default::default(),
            rng: PhantomRng,
        }
    }
}

impl<'a, T, const N: usize, const P: bool> UnionFind<'a, QuickUnion<Unweighted<Borrowed>, P>, T, N>
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

impl<'a, R, T, const N: usize, const P: bool>
    UnionFind<'a, QuickUnion<ByRandom<R, Borrowed>, P>, T, N>
where
    T: VertexType,
    R: RngCore + AsMut<R>,
{
    pub fn new(representative: &'a mut [T], heuristic: &'a mut [usize], rng: R) -> Self {
        debug_assert!(
            representative.len() >= N,
            "Representative slice must have at least len >= N!"
        );
        debug_assert!(
            heuristic.len() >= N,
            "Heuristic slice must have at least len >= N!"
        );

        Self {
            representative,
            heuristic,
            algorithm: Default::default(),
            rng,
        }
    }
}

impl<'a, T, const N: usize, const PATH_COMPRESS: bool>
    UnionFind<'a, QuickUnion<Unweighted<Borrowed<Thin>>, PATH_COMPRESS>, T, N>
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

impl<'a, T, const N: usize, const PATH_COMPRESS: bool>
    UnionFind<'a, QuickUnion<BySize<Borrowed<Thin>>, PATH_COMPRESS>, T, N>
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

impl<'a, T, const N: usize, const PATH_COMPRESS: bool>
    UnionFind<'a, QuickUnion<ByRank<Borrowed<Thin>>, PATH_COMPRESS>, T, N>
where
    T: VertexType,
{
    pub fn new(representative: &'a mut [T; N], heuristic: &'a mut [usize; N]) -> Self {
        debug_assert!(
            representative.len() >= N,
            "Representative slice must have at least len >= N!"
        );
        debug_assert!(
            heuristic.len() >= N,
            "Heuristic slice must have at least len >= N!"
        );

        Self {
            representative,
            heuristic,
            algorithm: Default::default(),
            rng: PhantomRng,
        }
    }
}
