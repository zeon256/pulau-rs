//! Trait implementations for QuickUnion: Connected, Union, Find

use crate::{Connected, Find, Union, VertexType, quickunion::Heuristic};
use super::QuickUnion;

impl<H, T, const PATH_COMPRESS: bool> Connected<T> for QuickUnion<H, PATH_COMPRESS>
where
    T: VertexType,
    Self: Find<T>,
{
    fn connected(representative: &mut [T], a: T::IdentifierType, b: T::IdentifierType) -> bool {
        Self::find(representative, a) == Self::find(representative, b)
    }
}

impl<H, T, const COMPRESS_PATH: bool> Union<T, H> for QuickUnion<H, COMPRESS_PATH>
where
    T: VertexType,
    H: Heuristic,
    Self: Find<T>,
{
    fn union_sets<'a>(
        representative: &mut [T],
        heuristic: &mut [usize],
        mut a: T::IdentifierType,
        mut b: T::IdentifierType,
        r: &mut H::RngProvider,
    ) {
        a = Self::find(representative, a).id();
        b = Self::find(representative, b).id();
        H::handle_decision(a, b, heuristic, representative, r)
    }
}

impl<H, V: VertexType, const COMPRESS_PATH: bool> Find<V> for QuickUnion<H, COMPRESS_PATH> {
    fn find(representative: &mut [V], mut a: V::IdentifierType) -> V {
        while a != representative[V::usize(a)].id() {
            // path compression
            if COMPRESS_PATH {
                representative[V::usize(a)] =
                    representative[V::usize(representative[V::usize(a)].id()).id()];
            }
            a = representative[V::usize(a)].id()
        }
        representative[V::usize(a)]
    }
}
