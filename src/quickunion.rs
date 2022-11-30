use crate::{Connected, Find, Union, UnionFind, WithContainer, IndexType};

#[derive(Debug, Default)]
pub struct QuickUnion<const WEIGHTED: bool = true, const COMPRESS_PATH: bool = true>;

impl WithContainer for QuickUnion {
    type RankContainer<T: Copy, const N: usize> = [T; N];
    type RepresentativeContainer<R: Copy, const N: usize> = [R; N];
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
                    rank: [1; N],
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
    fn connected(&self, representative: &Self::RepresentativeContainer<T, N>, a: T, b: T) -> bool {
        self.find(representative, a) == self.find(representative, b)
    }
}

impl<T, const N: usize, const M: usize> Union<T, N, M> for QuickUnion
where
    T: IndexType,
{
    fn union(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        rank: &mut Self::RankContainer<T, M>,
        a: T,
        b: T,
    ) {
    }
}

impl<T, const N: usize> Find<T, N> for QuickUnion
where
    T: IndexType,
{
    fn find(&self, representative: &Self::RepresentativeContainer<T, N>, a: T) -> T {
        unimplemented!()
    }
}

generate_default_ctor_quickunion!(u8, u16, u32, u64, usize);
