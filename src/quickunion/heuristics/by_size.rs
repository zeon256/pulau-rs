//! BySize heuristic for QuickUnion

use core::marker::PhantomData;

use crate::{Owned, VertexType, quickunion::Heuristic, rng::PhantomRng};

pub struct BySize<K = Owned>(PhantomData<K>);

impl<K> Heuristic for BySize<K> {
    type RngProvider = PhantomRng;

    #[inline(always)]
    fn handle_decision<T>(
        mut a: T::IdentifierType,
        mut b: T::IdentifierType,
        size: &mut [usize],
        representative: &mut [T],
        _r: &mut Self::RngProvider,
    ) where
        T: VertexType,
    {
        if a != b {
            if size[T::usize(a)] < size[T::usize(b)] {
                core::mem::swap(&mut a, &mut b);
            }
            representative[T::usize(b)] = representative[T::usize(a)];
            size[T::usize(a)] += size[T::usize(b)];
        }
    }
}
