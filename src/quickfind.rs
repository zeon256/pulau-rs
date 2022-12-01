use crate::{Connected, Find, IndexType, Union, UnionFind, WithContainer};

/// [`QuickFind`] algorithm
#[derive(Debug, Default)]
pub struct QuickFind;

impl WithContainer for QuickFind {
    type HeuristicContainer<T: IndexType, const N: usize> = [T; 0];
    type RepresentativeContainer<R: IndexType, const N: usize> = [R; N];
}

macro_rules! generate_default_ctor_quickfind {
    ($($num_type:ident), *) => {
        $(
        impl<const N: usize> Default for UnionFind<QuickFind, $num_type, N, 0>
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

impl<T, const N: usize> Connected<T, N> for QuickFind
where
    T: IndexType,
{
    fn connected(&mut self, representative: &mut Self::RepresentativeContainer<T, N>, a: T, b: T) -> bool {
        self.find(representative, a) == self.find(representative, b)
    }
}

impl<T, const N: usize> Union<T, N, 0> for QuickFind
where
    T: IndexType,
{
    fn union_sets(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        _heuristic: &mut Self::HeuristicContainer<T, 0>,
        a: T,
        b: T,
    ) {
        let root_a = self.find(representative, a);
        let root_b = self.find(representative, b);
        for item in representative {
            if *item == root_a {
                *item = root_b;
            }
        }
    }
}

impl<T, const N: usize> Find<T, N> for QuickFind
where
    T: IndexType,
{
    fn find(&mut self, representative: &mut Self::RepresentativeContainer<T, N>, a: T) -> T {
        assert!(a.usize() < N);
        representative[a.usize()]
    }
}

generate_default_ctor_quickfind!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use crate::{QuickFind, UnionFind};

    #[test]
    fn test_qf() {
        let mut uf = UnionFind::<QuickFind, u32, 10>::new();
        uf.union_sets(4, 3);
        uf.union_sets(3, 8);
        uf.union_sets(6, 5);
        uf.union_sets(9, 4);
        assert_eq!(uf.connected(3, 9), true);
    }
}
