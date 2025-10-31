//! ByRandom heuristic for QuickUnion

use rand_core::RngCore;
use crate::{Owned, VertexType, quickunion::Heuristic};

pub struct ByRandom<R, K = Owned>(core::marker::PhantomData<(R, K)>);

impl<R: RngCore + AsMut<R>, K> Heuristic for ByRandom<R, K> {
    type RngProvider = R;

    #[inline]
    fn handle_decision<T>(
        a: T::IdentifierType,
        b: T::IdentifierType,
        _heuristic: &mut [usize],
        representative: &mut [T],
        rng: &mut Self::RngProvider,
    ) where
        T: VertexType,
    {
        if a == b {
            return;
        }
        let root_a = T::usize(a);
        let root_b = T::usize(b);
        if rng.next_u32() % 2 == 0 {
            representative[root_a] = representative[root_b];
        } else {
            representative[root_b] = representative[root_a];
        }
    }
}
