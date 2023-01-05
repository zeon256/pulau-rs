use core::marker::PhantomData;

use crate::{Connected, Find, IndexType, Union, UnionFind, WithContainer};

/// Link by rank of tree
#[derive(Default, Debug)]
pub struct ByRank;

/// Link by size of tree
#[derive(Default, Debug)]
pub struct BySize;

/// No heuristic linking
#[derive(Default, Debug)]
pub struct Unweighted;

/// [`QuickUnion`] algorithm
///
/// This algorithm is parameterized by the following
/// - `H` - Heuristic Type. Available types: [`ByRank`], [`BySize`], [`Unweighted`]
/// - `COMPRESS_PATH` - boolean value, enables path compression during find operation
///
/// By default, [`ByRank`] heuristic is used and path compression is enabled
#[derive(Debug, Default)]
pub struct QuickUnion<H = ByRank, const COMPRESS_PATH: bool = true> {
    heuristic: PhantomData<H>,
}

impl WithContainer for QuickUnion<ByRank> {
    type HeuristicContainer<const N: usize> = [usize; N];
    type RepresentativeContainer<R: IndexType, const N: usize> = [R; N];
}

impl WithContainer for QuickUnion<BySize> {
    type HeuristicContainer<const N: usize> = [usize; N];
    type RepresentativeContainer<R: IndexType, const N: usize> = [R; N];
}

impl<const PATH_COMPRESS: bool> WithContainer for QuickUnion<Unweighted, PATH_COMPRESS> {
    type HeuristicContainer<const N: usize> = [usize; 0];
    type RepresentativeContainer<R: IndexType, const N: usize> = [R; N];
}

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
        impl<const N: usize> Default for UnionFind<QuickUnion, $num_type, N>
        {
            fn default() -> Self {
                Self {
                    representative: generate_representative!(N, $num_type),
                    heuristic: [0; N],
                    algorithm: Default::default(),
                }
            }
        }

        impl<const N: usize> Default for UnionFind<QuickUnion<BySize>, $num_type, N>
        {
            fn default() -> Self {
                Self {
                    representative: generate_representative!(N, $num_type),
                    heuristic: [1; N],
                    algorithm: Default::default(),
                }
            }
        }

        impl<const N: usize> Default for UnionFind<QuickUnion<Unweighted, false>, $num_type, N>
        {
            fn default() -> Self {
                Self {
                    representative: generate_representative!(N, $num_type),
                    heuristic: [0; 0],
                    algorithm: Default::default(),
                }
            }
        }

        impl<const N: usize> Default for UnionFind<QuickUnion<Unweighted, true>, $num_type, N>
        {
            fn default() -> Self {
                Self {
                    representative: generate_representative!(N, $num_type),
                    heuristic: [0; 0],
                    algorithm: Default::default(),
                }
            }
        }
        )*
    };
}

impl<H, T, const N: usize, const PATH_COMPRESS: bool> Connected<T, N>
    for QuickUnion<H, PATH_COMPRESS>
where
    T: IndexType,
    Self: Find<T, N>,
{
    fn connected(
        representative: &mut Self::RepresentativeContainer<T, N>,
        a: T::IdentifierType,
        b: T::IdentifierType,
    ) -> bool {
        Self::find(representative, a) == Self::find(representative, b)
    }
}

impl<T, const N: usize> Union<T, N> for QuickUnion
where
    T: IndexType,
{
    fn union_sets(
        representative: &mut Self::RepresentativeContainer<T, N>,
        rank: &mut Self::HeuristicContainer<N>,
        mut a: T::IdentifierType,
        mut b: T::IdentifierType,
    ) {
        a = Self::find(representative, a).id();
        b = Self::find(representative, b).id();

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

impl<T, const N: usize> Union<T, N> for QuickUnion<BySize>
where
    T: IndexType,
{
    fn union_sets(
        representative: &mut Self::RepresentativeContainer<T, N>,
        size: &mut Self::HeuristicContainer<N>,
        mut a: T::IdentifierType,
        mut b: T::IdentifierType,
    ) {
        a = Self::find(representative, a).id();
        b = Self::find(representative, b).id();

        if a != b {
            if size[T::usize(a)] < size[T::usize(b)] {
                core::mem::swap(&mut a, &mut b);
            }
            representative[T::usize(b)] = representative[T::usize(a)];
            size[T::usize(a)] += size[T::usize(b)];
        }
    }
}

impl<T, const N: usize, const PATH_COMPRESS: bool> Union<T, N>
    for QuickUnion<Unweighted, PATH_COMPRESS>
where
    T: IndexType,
    Self: Find<T, N>,
{
    fn union_sets(
        representative: &mut Self::RepresentativeContainer<T, N>,
        _heuristic: &mut Self::HeuristicContainer<N>,
        mut a: T::IdentifierType,
        mut b: T::IdentifierType,
    ) {
        a = Self::find(representative, a).id();
        b = Self::find(representative, b).id();

        if a == b {
            return;
        }

        representative[T::usize(a)] = representative[T::usize(b)];
    }
}

impl<A, T, const N: usize> Find<T, N> for QuickUnion<A, false>
where
    T: IndexType,
    Self: WithContainer,
{
    fn find(
        representative: &mut Self::RepresentativeContainer<T, N>,
        mut a: T::IdentifierType,
    ) -> T {
        while a != representative[T::usize(a)].id() {
            a = representative[T::usize(a)].id()
        }
        representative[T::usize(a)]
    }
}

impl<A, T, const N: usize> Find<T, N> for QuickUnion<A, true>
where
    T: IndexType,
    Self: WithContainer,
{
    fn find(
        representative: &mut Self::RepresentativeContainer<T, N>,
        mut a: T::IdentifierType,
    ) -> T {
        while a != representative[T::usize(a)].id() {
            // path compression
            representative[T::usize(a)] = representative[T::usize(representative[T::usize(a)].id()).id()];
            a = representative[T::usize(a)].id()
        }
        representative[T::usize(a)]
    }
}

generate_default_ctor!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use crate::{QuickUnion, UnionFind, tests::CityVertex};
    use core::mem;
    use super::{BySize, Unweighted};

    #[test]
    fn test_qu() {
        let mut uf = UnionFind::<QuickUnion<Unweighted, false>, u8, 10>::default();
        uf.union_sets(4, 3);
        uf.union_sets(3, 8);
        uf.union_sets(6, 5);
        uf.union_sets(9, 4);
        assert!(uf.connected(3, 9));
    }

    #[test]
    fn test_getter_qu() {        
        let mut uf = UnionFind::<QuickUnion<Unweighted, false>, u8, 10>::default();
        uf.union_sets(4, 3);
        uf.union_sets(3, 8);
        uf.union_sets(6, 5);
        uf.union_sets(9, 4);
        for _ in uf.heuristic() {
            panic!("Should not even loop!");
        }
    }

    #[test]
    fn test_qu_mem() {
        assert_eq!(
            mem::size_of::<[u32; 10]>(),
            mem::size_of::<UnionFind::<QuickUnion<Unweighted, false>, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>(),
            mem::size_of::<UnionFind::<QuickUnion<Unweighted, false>, CityVertex<'_>, 10>>()
        );
    }

    #[test]
    fn test_qupc() {
        let mut uf = UnionFind::<QuickUnion<Unweighted, true>, u8, 10>::default();
        uf.union_sets(4, 3);
        uf.union_sets(3, 8);
        uf.union_sets(6, 5);
        uf.union_sets(9, 4);
        assert!(uf.connected(3, 9));
    }

    #[test]
    fn test_getter_qupc() {        
        let mut uf = UnionFind::<QuickUnion<Unweighted, true>, u8, 10>::default();
        uf.union_sets(4, 3);
        uf.union_sets(3, 8);
        uf.union_sets(6, 5);
        uf.union_sets(9, 4);
        for _ in uf.heuristic() {
            panic!("Should not even loop!");
        }
    }

    #[test]
    fn test_qupc_mem() {
        assert_eq!(
            mem::size_of::<[u32; 10]>(),
            mem::size_of::<UnionFind::<QuickUnion<Unweighted, true>, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>(),
            mem::size_of::<UnionFind::<QuickUnion<Unweighted, true>, CityVertex<'_>, 10>>()
        );
    }

    #[test]
    fn test_wqupc_sz() {
        let mut uf = UnionFind::<QuickUnion<BySize>, u8, 10>::default();
        uf.union_sets(1, 2);
        uf.union_sets(2, 3);
        uf.union_sets(3, 4);
        assert_eq!([1, 4, 1, 1, 1, 1, 1, 1, 1, 1], uf.heuristic);
        uf.union_sets(5, 6);
        uf.union_sets(6, 7);
        uf.union_sets(7, 8);
        uf.union_sets(8, 9);
        assert_eq!([1, 4, 1, 1, 1, 5, 1, 1, 1, 1], uf.heuristic);
        assert_eq!([0, 1, 1, 1, 1, 5, 5, 5, 5, 5], uf.representative);
        uf.union_sets(4, 5);
        assert_eq!([0, 5, 1, 1, 1, 5, 5, 5, 5, 5], uf.representative);
    }

    #[test]
    fn test_wqupc_mem() {
        assert_eq!(
            mem::size_of::<[u32; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<QuickUnion<BySize>, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<QuickUnion<BySize, true>, CityVertex<'_>, 10>>()
        );
    }

    #[test]
    fn test_wqupc_rank() {
        let mut uf = UnionFind::<QuickUnion, u8, 12>::default();
        uf.union_sets(1, 2);
        uf.union_sets(2, 3);
        uf.union_sets(3, 4);
        assert_eq!([0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], uf.heuristic);
        uf.union_sets(5, 6);
        uf.union_sets(6, 7);
        uf.union_sets(7, 8);
        uf.union_sets(8, 9);
        assert_eq!([0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], uf.heuristic);
        assert_eq!([0, 1, 1, 1, 1, 5, 5, 5, 5, 5, 10, 11], uf.representative);
        uf.union_sets(4, 5);
        assert_eq!([0, 1, 1, 1, 1, 1, 5, 5, 5, 5, 10, 11], uf.representative);
        assert_eq!([0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], uf.heuristic);
        uf.union_sets(4, 11);
        assert_eq!([0, 1, 1, 1, 1, 1, 5, 5, 5, 5, 10, 1], uf.representative);
        assert_eq!([0, 2, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0], uf.heuristic);
    }

    #[test]
    fn test_wqupc_rank_mem() {
        assert_eq!(
            mem::size_of::<[u32; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<QuickUnion, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<QuickUnion, CityVertex<'_>, 10>>()
        );
    }
}
