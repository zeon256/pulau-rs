use crate::{AlgorithmContainer, Connected, Find, Union, UnionFind, VertexType};

/// [`QuickFind`] algorithm
#[derive(Debug, Default)]
pub struct QuickFind<const IS_SLICE: bool = false>;

impl AlgorithmContainer for QuickFind<false> {
    type HeuristicContainer<'a, const N: usize> = [usize; 0];
    type RepresentativeContainer<'a, R: VertexType + 'a, const N: usize> = [R; N];
}

impl AlgorithmContainer for QuickFind<true> {
    type HeuristicContainer<'a, const N: usize> = [usize; 0];
    type RepresentativeContainer<'a, R: VertexType + 'a, const N: usize> = &'a mut [R];
}

macro_rules! generate_default_ctor_quickfind {
    ($($num_type:ident), *) => {
        $(
        impl<const N: usize> Default for UnionFind<'_, QuickFind, $num_type, N>
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

impl<'a, T, const N: usize> UnionFind<'a, QuickFind<true>, T, N>
where
    T: VertexType,
{
    pub fn new(representative: &'a mut [T]) -> Self {
        Self {
            representative,
            heuristic: [0; 0],
            algorithm: Default::default(),
        }
    }
}

impl<T, const IS_SLICE: bool> Connected<T> for QuickFind<IS_SLICE>
where
    T: VertexType,
    Self: Find<T>,
{
    fn connected(representative: &mut [T], a: T::IdentifierType, b: T::IdentifierType) -> bool {
        Self::find(representative, a) == Self::find(representative, b)
    }
}

impl<T, const IS_SLICE: bool> Union<T> for QuickFind<IS_SLICE>
where
    T: VertexType,
    Self: Find<T>,
{
    fn union_sets(
        representative: &mut [T],
        _heuristic: &mut [usize],
        a: T::IdentifierType,
        b: T::IdentifierType,
    ) {
        let root_a = Self::find(representative, a);
        let root_b = Self::find(representative, b);
        for item in representative {
            if *item == root_a {
                *item = root_b;
            }
        }
    }
}

impl<T, const IS_SLICE: bool> Find<T> for QuickFind<IS_SLICE>
where
    T: VertexType,
{
    fn find(representative: &mut [T], a: T::IdentifierType) -> T {
        assert!(T::usize(a) < representative.len());
        representative[T::usize(a)]
    }
}

generate_default_ctor_quickfind!(u8, u16, u32, u64, usize);

#[cfg(test)]
mod tests {
    use crate::{tests::CityVertex, QuickFind, UnionFind};
    use core::{mem, panic};

    #[test]
    fn test_qf() {
        let mut uf = UnionFind::<QuickFind, u32, 10>::default();
        uf.union_sets(4, 3);
        uf.union_sets(3, 8);
        uf.union_sets(6, 5);
        uf.union_sets(9, 4);
        assert!(uf.connected(3, 9));
    }

    #[test]
    fn test_qf_slice() {
        let mut representative = (0..10).collect::<heapless::Vec<_, 10>>();
        let mut uf = UnionFind::<QuickFind<true>, u32, 10>::new(representative.as_mut());
        uf.union_sets(4, 3);
        uf.union_sets(3, 8);
        uf.union_sets(6, 5);
        uf.union_sets(9, 4);
        assert!(uf.connected(3, 9));
    }

    impl<'a, const N: usize> TryFrom<[CityVertex<'a>; N]>
        for UnionFind<'_, QuickFind, CityVertex<'a>, N>
    {
        type Error = &'static str;

        fn try_from(cities: [CityVertex<'a>; N]) -> Result<Self, Self::Error> {
            for id in 0..N {
                if cities[id].id as usize != id {
                    return Err("Invalid cities id!");
                }
            }

            Ok(Self {
                representative: cities,
                heuristic: [0; 0],
                algorithm: Default::default(),
            })
        }
    }

    #[test]
    fn test_custom_type() {
        let cities = [
            CityVertex::new(0, "Zurich", 320),
            CityVertex::new(1, "Munich", 210),
            CityVertex::new(2, "Paris", 180),
            CityVertex::new(3, "London", 190),
            CityVertex::new(4, "Oslo", 250),
            CityVertex::new(5, "Stockholm", 280),
            CityVertex::new(6, "Helsinki", 280),
        ];

        let mut uf = UnionFind::<QuickFind, CityVertex<'static>, 7>::try_from(cities).unwrap();
        uf.union_sets(4, 3);
        uf.union_sets(3, 2);
        uf.union_sets(6, 5);
        assert!(uf.connected(4, 2));
        assert!(uf.connected(6, 5));
    }

    #[test]
    fn test_sz() {
        assert_eq!(
            mem::size_of::<[u32; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickFind, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<&'_ [u32]>(),
            mem::size_of::<UnionFind::<'_, QuickFind<true>, u32, 10>>()
        );
        assert_eq!(
            mem::size_of::<[CityVertex<'_>; 10]>(),
            mem::size_of::<UnionFind::<'_, QuickFind, CityVertex<'_>, 10>>()
        );
    }

    #[test]
    fn test_getter() {
        let mut uf = UnionFind::<QuickFind, u32, 10>::default();
        uf.union_sets(4, 3);
        uf.union_sets(3, 8);
        uf.union_sets(6, 5);
        uf.union_sets(9, 4);
        for _ in uf.heuristic() {
            panic!("Should not even loop!");
        }
    }
}
