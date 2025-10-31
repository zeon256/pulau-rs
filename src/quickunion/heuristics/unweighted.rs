//! Unweighted heuristic for QuickUnion

use crate::{Owned, VertexType, quickunion::Heuristic, rng::PhantomRng};

pub struct Unweighted<K = Owned>(core::marker::PhantomData<K>);

impl<K> Heuristic for Unweighted<K> {
    type RngProvider = PhantomRng;

    #[inline(always)]
    fn handle_decision<T>(
        a: T::IdentifierType,
        b: T::IdentifierType,
        _heuristic: &mut [usize],
        representative: &mut [T],
        _r: &mut Self::RngProvider,
    ) where
        T: VertexType,
    {
        if a == b {
            return;
        }
        representative[T::usize(a)] = representative[T::usize(b)];
    }
}
