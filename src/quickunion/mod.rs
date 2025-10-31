use crate::{Owned, VertexType};
use core::marker::PhantomData;
use rand_core::RngCore;

pub mod heuristics;
pub mod algorithm_container;
pub mod constructors;
pub mod borrowed_impls;
pub mod traits_impl;

pub use heuristics::{ByRank, BySize, Unweighted};

/// Heuristic for quick union algorithm
pub trait Heuristic {
    type RngProvider: RngCore + AsMut<Self::RngProvider>;

    fn handle_decision<T>(
        a: T::IdentifierType,
        b: T::IdentifierType,
        heuristic: &mut [usize],
        representative: &mut [T],
        r: &mut Self::RngProvider,
    ) where
        T: VertexType;
}

/// [`QuickUnion`] algorithm
///
/// This algorithm is parameterized by the following
/// - `H` - Heuristic Type. Available types: [`ByRank`], [`BySize`], [`Unweighted`]
/// - `COMPRESS_PATH` - boolean value, enables path compression during find operation
///
/// By default, [`ByRank`] heuristic is used and path compression is enabled
#[derive(Debug, Default)]
pub struct QuickUnion<H = ByRank<Owned>, const COMPRESS_PATH: bool = true> {
    heuristic: PhantomData<H>,
}

#[cfg(test)]
mod tests {
    use super::{BySize, Heuristic, Unweighted};
    use crate::{
        AlgorithmContainer, Borrowed, ByRank, QuickUnion, Thin, UnionFind, VertexType, rng::PhantomRng, tests::CityVertex
    };
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
    fn test_qu_mem_owned_u32() {
        assert_eq!(
            mem::size_of::<[u32; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<Unweighted, false>, u32, 10>>()
        );
    }

    #[test]
    fn test_qu_mem_owned_city_vertex() {
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<Unweighted, false>, CityVertex<'_>, 10>>()
        );
    }

    #[test]
    fn test_qu_mem_borrowed_u32() {
        assert_eq!(
            mem::size_of::<&[u32]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<Unweighted<Borrowed>, false>, u32, 10>>()
        );
    }

    #[test]
    fn test_qu_mem_borrowed_city_vertex() {
        assert_eq!(
            mem::size_of::<&[CityVertex<'_>]>(),
            mem::size_of::<
                UnionFind::<'_, QuickUnion<Unweighted<Borrowed>, false>, CityVertex<'_>, 10>,
            >()
        );
    }

    #[test]
    fn test_qu_mem_borrowed_thin_u32() {
        // HeuristicContainer is [usize; 0] (zero-sized), so only the representative pointer counts
        assert_eq!(
            mem::size_of::<&[u32; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<Unweighted<Borrowed<Thin>>, false>, u32, 10>>(
            )
        );
    }

    #[test]
    fn test_qu_mem_borrowed_thin_city_vertex() {
        // HeuristicContainer is [usize; 0] (zero-sized), so only the representative pointer counts
        assert_eq!(
            mem::size_of::<&[CityVertex<'_>; 10]>(),
            mem::size_of::<
                UnionFind::<'_, QuickUnion<Unweighted<Borrowed<Thin>>, false>, CityVertex<'_>, 10>,
            >()
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
    fn test_qupc_mem_owned_u32() {
        assert_eq!(
            mem::size_of::<[u32; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<Unweighted, true>, u32, 10>>()
        );
    }

    #[test]
    fn test_qupc_mem_owned_city_vertex() {
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
    fn test_wqupc_sz_mem_owned_u32() {
        assert_eq!(
            mem::size_of::<[u32; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<BySize>, u32, 10>>()
        );
    }

    #[test]
    fn test_wqupc_sz_mem_borrowed_u32() {
        assert_eq!(
            mem::size_of::<&[u32]>() + mem::size_of::<&[usize]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<BySize<Borrowed>>, u32, 10>>()
        );
    }

    #[test]
    fn test_wqupc_sz_mem_owned_city_vertex() {
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<BySize, true>, CityVertex<'_>, 10>>()
        );
    }

    #[test]
    fn test_wqupc_sz_mem_borrowed_city_vertex() {
        assert_eq!(
            mem::size_of::<&'_ [CityVertex<'_>]>() + mem::size_of::<&'_ [usize]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<BySize<Borrowed>, true>, CityVertex<'_>, 10>>(
            )
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
    fn test_wqupc_rank_mem_owned_u32() {
        assert_eq!(
            mem::size_of::<[u32; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion, u32, 10>>()
        );
    }

    #[test]
    fn test_wqupc_rank_mem_borrowed_u32() {
        assert_eq!(
            mem::size_of::<&[u32]>() + mem::size_of::<&[usize]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<ByRank<Borrowed>>, u32, 10>>()
        );
    }

    #[test]
    fn test_wqupc_rank_mem_owned_city_vertex() {
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>() + mem::size_of::<[usize; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickUnion, CityVertex<'_>, 10>>()
        );
    }

    struct ByRankHeaplessVec;

    impl AlgorithmContainer for QuickUnion<ByRankHeaplessVec> {
        type HeuristicKind<'a> = ByRankHeaplessVec;
        type HeuristicContainer<'a, const N: usize> = heapless::Vec<usize, N>;
        type RepresentativeContainer<'a, R: VertexType + 'a, const N: usize> = heapless::Vec<R, N>;
        type RngKind<'a> = PhantomRng;
    }

    impl<const N: usize> UnionFind<'_, QuickUnion<ByRankHeaplessVec>, u8, N> {
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
                rng: PhantomRng,
            }
        }
    }

    impl Heuristic for ByRankHeaplessVec {
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

    #[test]
    fn test_wqupc_rank_mem_heapless_vec_u8() {
        assert_eq!(
            mem::size_of::<heapless::Vec<u8, 12>>() + mem::size_of::<heapless::Vec<usize, 12>>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<ByRankHeaplessVec>, u8, 12>>()
        );
    }

    #[test]
    fn test_wqupc_rank_mem_heapless_vec_city_vertex() {
        assert_eq!(
            mem::size_of::<heapless::Vec<CityVertex<'_>, 10>>()
                + mem::size_of::<heapless::Vec<usize, 10>>(),
            mem::size_of::<UnionFind::<'_, QuickUnion<ByRankHeaplessVec>, CityVertex<'_>, 10>>()
        );
    }

    #[test]
    fn test_vec_heapless() {
        let mut uf = UnionFind::<QuickUnion<ByRankHeaplessVec>, u8, 12>::new();

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
    fn test_vec_heapless_borrowed() {
        const N: usize = 12;
        let mut representative = heapless::Vec::<_, N>::new();
        let _ = representative.resize(N, 0);

        for i in 0..(N as u8) {
            representative[i as usize] = i;
        }

        let mut heuristic = heapless::Vec::<usize, N>::from_slice(&[0; N]).unwrap();

        let mut uf = UnionFind::<QuickUnion<ByRank<Borrowed>>, u8, 12>::new(
            &mut representative,
            &mut heuristic,
        );

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
    fn test_slice() {
        let mut representative = (0..12).collect::<heapless::Vec<u8, 12>>();
        let mut heuristic = heapless::Vec::<usize, 12>::from_slice(&[0; 12]).unwrap();

        let mut uf = UnionFind::<QuickUnion<ByRank<Borrowed>>, u8, 12>::new(
            &mut representative,
            &mut heuristic,
        );

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
    fn test_slice_by_size() {
        let mut representative = (0..10).collect::<heapless::Vec<_, 10>>();
        let mut heuristic = heapless::Vec::<usize, 10>::from_slice(&[1; 10]).unwrap();

        let mut uf = UnionFind::<QuickUnion<BySize<Borrowed>>, u8, 10>::new(
            &mut representative,
            &mut heuristic,
        );

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

    // Tests for ByRandom heuristic
    // #[cfg(test)]
    // mod by_random_tests {
    //     use super::*;
    //     use crate::quickunion::ByRandom;
    //     use rand::{RngCore, SeedableRng};
    //     use rand_hc::Hc128Rng;

    //     impl

    //     #[test]
    //     fn test_by_random_basic() {
    //         // Use a seeded RNG for deterministic tests
    //         let rng = Hc128Rng::seed_from_u64(42);
    //         let mut uf = UnionFind::<QuickUnion<ByRandom<Hc128Rng>>, u8, 10>::new(rng);

    //         uf.union_sets(1, 2);
    //         uf.union_sets(3, 4);
    //         uf.union_sets(5, 6);

    //         // These should be connected
    //         assert!(uf.connected(1, 2));
    //         assert!(uf.connected(3, 4));
    //         assert!(uf.connected(5, 6));

    //         // These should not be connected
    //         assert!(!uf.connected(1, 3));
    //         assert!(!uf.connected(2, 5));
    //     }

    // #[test]
    // fn test_by_random_deterministic() {
    //     // With the same seed, we should get the same results
    //     let rng1 = Hc128Rng::seed_from_u64(12345);
    //     let mut uf1 = UnionFind::<QuickUnion<ByRandom<Hc128Rng>>, u8, 10>::new(rng1);

    //     let rng2 = Hc128Rng::seed_from_u64(12345);
    //     let mut uf2 = UnionFind::<QuickUnion<ByRandom<Hc128Rng>>, u8, 10>::new(rng2);

    //     // Perform same operations
    //     for i in 0..4 {
    //         uf1.union_sets(i, i + 1);
    //         uf2.union_sets(i, i + 1);
    //     }

    //     // Both should have the same representative array
    //     assert_eq!(uf1.representative, uf2.representative);
    // }

    // #[test]
    // fn test_by_random_large_tree() {
    //     let rng = Hc128Rng::seed_from_u64(999);
    //     let mut uf = UnionFind::<QuickUnion<ByRandom<Hc128Rng>>, u8, 20>::new(rng);

    //     // Build a large connected component
    //     for i in 0..19 {
    //         uf.union_sets(i, i + 1);
    //     }

    //     // All elements should be in the same component
    //     for i in 0..19 {
    //         assert!(uf.connected(0, i));
    //     }
    // }

    // #[test]
    // fn test_by_random_multiple_components() {
    //     let rng = Hc128Rng::seed_from_u64(777);
    //     let mut uf = UnionFind::<QuickUnion<ByRandom<Hc128Rng>>, u8, 12>::new(rng);

    //     // Create component 1: {0, 1, 2, 3}
    //     uf.union_sets(0, 1);
    //     uf.union_sets(1, 2);
    //     uf.union_sets(2, 3);

    //     // Create component 2: {4, 5, 6}
    //     uf.union_sets(4, 5);
    //     uf.union_sets(5, 6);

    //     // Create component 3: {7, 8, 9}
    //     uf.union_sets(7, 8);
    //     uf.union_sets(8, 9);

    //     // Verify components
    //     assert!(uf.connected(0, 3));
    //     assert!(uf.connected(4, 6));
    //     assert!(uf.connected(7, 9));

    //     // Verify separation
    //     assert!(!uf.connected(0, 4));
    //     assert!(!uf.connected(4, 7));
    //     assert!(!uf.connected(0, 7));
    // }

    // #[test]
    // fn test_by_random_union_same_element() {
    //     let rng = Hc128Rng::seed_from_u64(555);
    //     let mut uf = UnionFind::<QuickUnion<ByRandom<Hc128Rng>>, u8, 10>::new(rng);

    //     // Union an element with itself should be a no-op
    //     uf.union_sets(5, 5);
    //     assert_eq!(5, uf.find_set(5));
    // }

    // #[test]
    // fn test_by_random_borrowed() {
    //     let rng = Hc128Rng::seed_from_u64(42);
    //     let mut representative = (0..10).collect::<heapless::Vec<u8, 10>>();
    //     let mut heuristic = heapless::Vec::<usize, 10>::from_slice(&[0; 10]).unwrap();

    //     let mut uf = UnionFind::<QuickUnion<ByRandom<Hc128Rng, Borrowed>>, u8, 10>::new_with_rng(
    //         &mut representative,
    //         &mut heuristic,
    //         rng,
    //     );

    //     uf.union_sets(1, 2);
    //     uf.union_sets(3, 4);
    //     uf.union_sets(5, 6);

    //     assert!(uf.connected(1, 2));
    //     assert!(uf.connected(3, 4));
    //     assert!(uf.connected(5, 6));
    //     assert!(!uf.connected(1, 3));
    // }

    // #[test]
    // fn test_by_random_different_seeds() {
    //     // Different seeds should potentially produce different tree structures
    //     let rng1 = Hc128Rng::seed_from_u64(111);
    //     let mut uf1 = UnionFind::<QuickUnion<ByRandom<Hc128Rng>>, u8, 10>::new(rng1);

    //     let rng2 = Hc128Rng::seed_from_u64(222);
    //     let mut uf2 = UnionFind::<QuickUnion<ByRandom<Hc128Rng>>, u8, 10>::new(rng2);

    //     // Perform same operations
    //     for i in 0..5 {
    //         uf1.union_sets(i, i + 1);
    //         uf2.union_sets(i, i + 1);
    //     }

    //     // Both should have same connectivity
    //     for i in 0..5 {
    //         assert!(uf1.connected(0, i));
    //         assert!(uf2.connected(0, i));
    //     }

    //     // But potentially different tree structures (representative arrays may differ)
    //     // This is expected behavior with random linking
    // }
    // }
}
