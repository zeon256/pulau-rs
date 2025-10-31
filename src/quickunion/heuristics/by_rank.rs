//! ByRank heuristic for QuickUnion

use crate::{Owned, VertexType, quickunion::Heuristic, rng::PhantomRng};

pub struct ByRank<K = Owned>(core::marker::PhantomData<K>);

impl<K> Heuristic for ByRank<K> {
    type RngProvider = PhantomRng;

    #[inline(always)]
    fn handle_decision<T>(
        mut a: T::IdentifierType,
        mut b: T::IdentifierType,
        rank: &mut [usize],
        representative: &mut [T],
        _r: &mut Self::RngProvider,
    ) where
        T: VertexType,
    {
        if a != b {
            if rank[T::usize(a)] < rank[T::usize(b)] {
                core::mem::swap(&mut a, &mut b);
            }
            representative[T::usize(b)] = representative[T::usize(a)];
            if rank[T::usize(a)] == rank[T::usize(b)] {
                rank[T::usize(a)] += 1;
            }
        }
    }
}
