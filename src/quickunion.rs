use core::marker::PhantomData;

use crate::{AlgorithmContainer, Connected, Find, Union, UnionFind, VertexType};

/// Link by rank of tree
#[derive(Default, Debug)]
pub struct ByRank;

/// Link by size of tree
#[derive(Default, Debug)]
pub struct BySize;

/// No heuristic linking
#[derive(Default, Debug)]
pub struct Unweighted;

pub trait Heuristic {
    fn handle_decision<T>(
        a: T::IdentifierType,
        b: T::IdentifierType,
        heuristic: &mut [usize],
        representative: &mut [T],
    ) where
        T: VertexType;
}

impl Heuristic for Unweighted {
    fn handle_decision<T>(
        a: T::IdentifierType,
        b: T::IdentifierType,
        _heuristic: &mut [usize],
        representative: &mut [T],
    ) where
        T: VertexType,
    {
        if a == b {
            return;
        }

        representative[T::usize(a)] = representative[T::usize(b)];
    }
}

impl Heuristic for ByRank {
    fn handle_decision<T>(
        mut a: T::IdentifierType,
        mut b: T::IdentifierType,
        rank: &mut [usize],
        representative: &mut [T],
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

impl Heuristic for BySize {
    fn handle_decision<T>(
        mut a: T::IdentifierType,
        mut b: T::IdentifierType,
        size: &mut [usize],
        representative: &mut [T],
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

impl AlgorithmContainer for QuickUnion<ByRank> {
    type HeuristicContainer<'a, const N: usize> = [usize; N];
    type RepresentativeContainer<'a, R: VertexType + 'a, const N: usize> = [R; N];
}

impl AlgorithmContainer for QuickUnion<BySize> {
    type HeuristicContainer<'a, const N: usize> = [usize; N];
    type RepresentativeContainer<'a, R: VertexType + 'a, const N: usize> = [R; N];
}

impl<const PATH_COMPRESS: bool> AlgorithmContainer for QuickUnion<Unweighted, PATH_COMPRESS> {
    type HeuristicContainer<'a, const N: usize> = [usize; 0];
    type RepresentativeContainer<'a, R: VertexType + 'a, const N: usize> = [R; N];
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
        impl<const N: usize> Default for UnionFind<'_, QuickUnion, $num_type, N>
        {
            fn default() -> Self {
                Self {
                    representative: generate_representative!(N, $num_type),
                    heuristic: [0; N],
                    algorithm: Default::default(),
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
                }
            }
        }
        )*
    };
}

impl<H, T, const PATH_COMPRESS: bool> Connected<T> for QuickUnion<H, PATH_COMPRESS>
where
    T: VertexType,
    Self: Find<T>,
{
    fn connected(representative: &mut [T], a: T::IdentifierType, b: T::IdentifierType) -> bool {
        Self::find(representative, a) == Self::find(representative, b)
    }
}

impl<H, T, const COMPRESS_PATH: bool> Union<T> for QuickUnion<H, COMPRESS_PATH>
where
    T: VertexType,
    H: Heuristic,
    Self: Find<T>,
{
    fn union_sets(
        representative: &mut [T],
        heuristic: &mut [usize],
        mut a: T::IdentifierType,
        mut b: T::IdentifierType,
    ) {
        a = Self::find(representative, a).id();
        b = Self::find(representative, b).id();
        H::handle_decision(a, b, heuristic, representative)
    }
}

impl<H, T, const COMPRESS_PATH: bool> Find<T> for QuickUnion<H, COMPRESS_PATH>
where
    T: VertexType,
{
    fn find(representative: &mut [T], mut a: T::IdentifierType) -> T {
        while a != representative[T::usize(a)].id() {
            // path compression
            if COMPRESS_PATH {
                representative[T::usize(a)] =
                    representative[T::usize(representative[T::usize(a)].id()).id()];
            }
            a = representative[T::usize(a)].id()
        }
        representative[T::usize(a)]
    }
}

generate_default_ctor!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use super::{BySize, Heuristic, Unweighted};
    use crate::{tests::CityVertex, AlgorithmContainer, QuickUnion, UnionFind, VertexType};
    use core::mem;

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
            mem::size_of::<UnionFind::<'_, QuickUnion<Unweighted, false>, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<Unweighted, false>, CityVertex<'_>, 10>>()
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
            mem::size_of::<UnionFind::<'_, QuickUnion<Unweighted, true>, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<Unweighted, true>, CityVertex<'_>, 10>>()
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
            mem::size_of::<UnionFind::<'_, QuickUnion<BySize>, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<BySize, true>, CityVertex<'_>, 10>>()
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
            mem::size_of::<UnionFind::<'_, QuickUnion, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion, CityVertex<'_>, 10>>()
        );
    }

    struct ByRankVec;

    impl AlgorithmContainer for QuickUnion<ByRankVec> {
        type HeuristicContainer<'a, const N: usize> = heapless::Vec<usize, N>;
        type RepresentativeContainer<'a, R: VertexType + 'a, const N: usize> = heapless::Vec<R, N>;
    }

    impl<const N: usize> UnionFind<'_, QuickUnion<ByRankVec>, u8, N> {
        pub fn new() -> Self {
            let mut representative = heapless::Vec::<_, N>::new();
            let _ = representative.resize(N, 0);

            for i in 0..(N as u8) {
                representative[i as usize] = i;
            }

            let heuristic = heapless::Vec::<usize, N>::from_slice(&[0; N]).unwrap();

            Self {
                representative,
                heuristic,
                algorithm: Default::default(),
            }
        }
    }

    impl Heuristic for ByRankVec {
        fn handle_decision<T>(
            mut a: T::IdentifierType,
            mut b: T::IdentifierType,
            rank: &mut [usize],
            representative: &mut [T],
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

    #[test]
    fn test_vec_heapless() {
        let mut uf = UnionFind::<QuickUnion<ByRankVec>, u8, 12>::new();

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
    struct ByRankSlice;

    impl AlgorithmContainer for QuickUnion<ByRankSlice> {
        type HeuristicContainer<'a, const N: usize> = &'a mut [usize];
        type RepresentativeContainer<'a, R: VertexType + 'a, const N: usize> = &'a mut [R];
    }

    impl<'a, const N: usize> UnionFind<'a, QuickUnion<ByRankSlice>, u8, N> {
        pub fn new(representative: &'a mut [u8], heuristic: &'a mut [usize]) -> Self {
            Self {
                representative,
                heuristic,
                algorithm: Default::default(),
            }
        }
    }

    impl Heuristic for ByRankSlice {
        fn handle_decision<T>(
            mut a: T::IdentifierType,
            mut b: T::IdentifierType,
            rank: &mut [usize],
            representative: &mut [T],
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

    #[test]
    fn test_slice() {
        let mut representative = heapless::Vec::<_, 12>::new();
        let _ = representative.resize(12, 0);

        for i in 0..(12 as u8) {
            representative[i as usize] = i;
        }

        let mut heuristic = heapless::Vec::<usize, 12>::from_slice(&[0; 12]).unwrap();

        let mut uf =
            UnionFind::<QuickUnion<ByRankSlice>, u8, 12>::new(&mut representative, &mut heuristic);

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
}
