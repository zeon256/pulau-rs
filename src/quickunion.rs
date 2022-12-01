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
/// - `T` - Heuristic Type. Available types: [`ByRank`], [`BySize`], [`Unweighted`]
/// - `COMPRESS_PATH` - boolean value, enables path compression during find operation
/// By default, both are true
#[derive(Debug, Default)]
pub struct QuickUnion<T = ByRank, const COMPRESS_PATH: bool = true> {
    heuristic: PhantomData<T>,
}

impl WithContainer for QuickUnion {
    type HeuristicContainer<T: IndexType, const N: usize> = [T; N];
    type RepresentativeContainer<R: IndexType, const N: usize> = [R; N];
}

impl WithContainer for QuickUnion<BySize, true> {
    type HeuristicContainer<T: IndexType, const N: usize> = [T; N];
    type RepresentativeContainer<R: IndexType, const N: usize> = [R; N];
}

impl WithContainer for QuickUnion<Unweighted, false> {
    type HeuristicContainer<T: IndexType, const N: usize> = [T; 0];
    type RepresentativeContainer<R: IndexType, const N: usize> = [R; N];
}

macro_rules! generate_default_ctor_quickunion {
    ($($num_type:ident), *) => {
        $(
        impl<const N: usize> Default for UnionFind<QuickUnion, $num_type, N, N>
        {
            fn default() -> Self {
                let mut representative = [0; N];

                for i in 0..(N as $num_type) {
                    representative[i as usize] = i;
                }

                Self {
                    representative,
                    heuristic: [0; N],
                    algorithm: Default::default(),
                }
            }
        }
        )*
    };
}

macro_rules! generate_default_ctor_quickunion_by_size {
    ($($num_type:ident), *) => {
        $(
        impl<const N: usize> Default for UnionFind<QuickUnion<BySize>, $num_type, N, N>
        {
            fn default() -> Self {
                let mut representative = [0; N];

                for i in 0..(N as $num_type) {
                    representative[i as usize] = i;
                }

                Self {
                    representative,
                    heuristic: [1; N],
                    algorithm: Default::default(),
                }
            }
        }
        )*
    };
}

macro_rules! generate_default_ctor_quickunion_unweighted {
    ($($num_type:ident), *) => {
        $(
        impl<const N: usize> Default for UnionFind<QuickUnion<Unweighted, false>, $num_type, N, N>
        {
            fn default() -> Self {
                let mut representative = [0; N];

                for i in 0..(N as $num_type) {
                    representative[i as usize] = i;
                }

                Self {
                    representative,
                    heuristic: [0; 0],
                    algorithm: Default::default(),
                }
            }
        }
        )*
    };
}

impl<T, const N: usize> Connected<T, N> for QuickUnion
where
    T: IndexType,
{
    fn connected(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        a: T,
        b: T,
    ) -> bool {
        self.find(representative, a) == self.find(representative, b)
    }
}

impl<T, const N: usize, const M: usize> Union<T, N, M> for QuickUnion
where
    T: IndexType,
{
    fn union_sets(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        rank: &mut Self::HeuristicContainer<T, M>,
        mut a: T,
        mut b: T,
    ) {
        a = self.find(representative, a);
        b = self.find(representative, b);

        if a != b {
            if rank[a.usize()] < rank[b.usize()] {
                core::mem::swap(&mut a, &mut b);
            }
            representative[b.usize()] = a;
            if rank[a.usize()] == rank[b.usize()] {
                rank[a.usize()] += T::one();
            }
        }
    }
}

impl<T, const N: usize> Find<T, N> for QuickUnion
where
    T: IndexType,
{
    fn find(&mut self, representative: &mut Self::RepresentativeContainer<T, N>, mut a: T) -> T {
        while a != representative[a.usize()] {
            representative[a.usize()] = representative[representative[a.usize()].usize()];
            a = representative[a.usize()]
        }
        a
    }
}

impl<T, const N: usize> Connected<T, N> for QuickUnion<BySize>
where
    T: IndexType,
{
    fn connected(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        a: T,
        b: T,
    ) -> bool {
        self.find(representative, a) == self.find(representative, b)
    }
}

impl<T, const N: usize, const M: usize> Union<T, N, M> for QuickUnion<BySize>
where
    T: IndexType,
{
    fn union_sets(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        size: &mut Self::HeuristicContainer<T, M>,
        mut a: T,
        mut b: T,
    ) {
        a = self.find(representative, a);
        b = self.find(representative, b);

        if a != b {
            if size[a.usize()] < size[b.usize()] {
                core::mem::swap(&mut a, &mut b);
            }
            representative[b.usize()] = a;
            size[a.usize()] += size[b.usize()];
        }
    }
}

impl<T, const N: usize> Find<T, N> for QuickUnion<BySize>
where
    T: IndexType,
{
    fn find(&mut self, representative: &mut Self::RepresentativeContainer<T, N>, mut a: T) -> T {
        while a != representative[a.usize()] {
            representative[a.usize()] = representative[representative[a.usize()].usize()];
            a = representative[a.usize()]
        }
        a
    }
}

impl<T, const N: usize> Connected<T, N> for QuickUnion<Unweighted, false>
where
    T: IndexType,
{
    fn connected(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        a: T,
        b: T,
    ) -> bool {
        self.find(representative, a) == self.find(representative, b)
    }
}

impl<T, const N: usize, const M: usize> Union<T, N, M> for QuickUnion<Unweighted, false>
where
    T: IndexType,
{
    fn union_sets(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        _heuristic: &mut Self::HeuristicContainer<T, M>,
        mut a: T,
        mut b: T,
    ) {
        a = self.find(representative, a);
        b = self.find(representative, b);

        if a == b {
            return;
        }

        representative[a.usize()] = b;
    }
}

impl<T, const N: usize> Find<T, N> for QuickUnion<Unweighted, false>
where
    T: IndexType,
{
    fn find(&mut self, representative: &mut Self::RepresentativeContainer<T, N>, mut a: T) -> T {
        while a != representative[a.usize()] {
            a = representative[a.usize()]
        }
        a
    }
}

generate_default_ctor_quickunion!(u8, u16, u32, u64, usize);
generate_default_ctor_quickunion_by_size!(u8, u16, u32, u64, usize);
generate_default_ctor_quickunion_unweighted!(u8, u16, u32, u64, usize);
