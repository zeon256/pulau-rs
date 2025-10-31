//! ByRandom heuristic for QuickUnion

use core::marker::PhantomData;

use crate::{quickunion::Heuristic, Owned, VertexType};
use rand_core::RngCore;

pub struct ByRandom<R, K = Owned>(PhantomData<(R, K)>);

impl<R: RngCore, K> Heuristic for ByRandom<R, K> {
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

        // coin flip to decide which root becomes the parent
        if rng.next_u32() % 2 == 0 {
            representative[root_a] = representative[root_b];
        } else {
            representative[root_b] = representative[root_a];
        }
    }
}
