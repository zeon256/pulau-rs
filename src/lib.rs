#![no_std]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_flatten,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![doc = include_str!("../libdoc.md")]

pub mod quickfind;
pub mod quickunion;

use core::marker::PhantomData;
use core::ops::AddAssign;

pub use crate::quickfind::QuickFind;
pub use crate::quickunion::QuickUnion;
pub use crate::quickunion::{ByRank, BySize, Unweighted};

pub trait VertexType: Eq + Copy {
    type IdentifierType: Copy + Eq + PartialOrd + AddAssign<Self::IdentifierType>;

    fn id(&self) -> Self::IdentifierType;
    fn usize(a: Self::IdentifierType) -> usize;
}

macro_rules! generate_index_type_impl{
    ($($num_type:ident), *) => {
        $(
            impl VertexType for $num_type {
                type IdentifierType = Self;

                #[inline(always)]
                fn id(&self) -> Self {
                    *self
                }

                #[inline(always)]
                fn usize(a: Self) -> usize {
                    a as usize
                }
            }
        )*
    };
}

generate_index_type_impl!(u8, u16, u32, u64, usize);

/// [`UnionFind`] data structure
///
/// This data structure stores a collection of disjoint (non-overlapping) sets.
///
/// [`UnionFind`] is parameterized by the following
/// - `A` - Algorithm, i.e., [`QuickFind`], [`QuickUnion`]
/// - `T` - Any unsigned integral types, i.e., [`u8`], [`u16`], [`u32`], [`u64`], [`usize`] or any type that implements `VertexType`
/// - `N` - Size of internal representative buffer
///
/// # Example
/// ```rust
/// use pulau_rs::{UnionFind, QuickFind, QuickUnion};
/// fn make_uf_quickfind() {
///     // construct with quickfind algorithm with fixed size 10
///     let mut uf = UnionFind::<QuickFind, u32, 10>::default();
/// }
///
/// fn make_uf_quickunion() {
///     // construct with weighted quickunion with path compression algorithm with fixed size 10
///     let mut uf = UnionFind::<QuickUnion, u32, 10>::default();
/// }
/// ```
///
/// # Size Guarantees
/// Size of [`UnionFind`] depends on whether the algorithm you have chosen is weighted
///
/// Assuming no padding,
/// If it's weighted then, size of [`UnionFind`] is `T * N + size_of(usize) * N`
///
/// Else it will be `T * N`
/// 
/// If you are using borrowed buffers, then the size will be the `core::mem::size_of::<usize>() * 2`
/// if it weighted, else it will just be `core::mem::size_of::<usize>()`
pub struct UnionFind<'a, A, T, const N: usize>
where
    T: VertexType + 'a,
    A: AlgorithmContainer,
{
    representative: A::RepresentativeContainer<'a, T, N>,
    heuristic: A::HeuristicContainer<'a, N>,
    algorithm: PhantomData<A>,
}

impl<'a, A, T, const N: usize> UnionFind<'a, A, T, N>
where
    T: VertexType,
    A: AlgorithmContainer + Union<T> + Find<T> + Connected<T>,
{
    /// Checks whether 2 nodes are connected to each other
    pub fn connected(&mut self, a: T::IdentifierType, b: T::IdentifierType) -> bool {
        A::connected(self.representative.as_mut(), a, b)
    }

    /// Finds a node
    pub fn find(&mut self, a: T::IdentifierType) -> T {
        A::find(self.representative.as_mut(), a)
    }

    /// Unions 2 node. If those 2 nodes are already part of the same component
    /// then this does nothing
    pub fn union_sets(&mut self, a: T::IdentifierType, b: T::IdentifierType) {
        A::union_sets(self.representative.as_mut(), self.heuristic.as_mut(), a, b)
    }

    /// Gets the representative slice
    pub fn representative(&self) -> &A::RepresentativeContainer<'a, T, N> {
        &self.representative
    }

    /// Gets the heuristic slice
    pub fn heuristic(&self) -> &A::HeuristicContainer<'a, N> {
        &self.heuristic
    }
}

pub trait AlgorithmContainer {
    type HeuristicContainer<'a, const N: usize>: AsRef<[usize]> + AsMut<[usize]>;

    type RepresentativeContainer<'a, R: VertexType + 'a, const N: usize>: AsRef<[R]> + AsMut<[R]>;
}

pub trait Union<T>
where
    T: VertexType,
{
    fn union_sets(
        representative: &mut [T],
        heuristic: &mut [usize],
        a: T::IdentifierType,
        b: T::IdentifierType,
    );
}

pub trait Find<T>
where
    T: VertexType,
{
    fn find(representative: &mut [T], a: T::IdentifierType) -> T;
}

pub trait Connected<T>
where
    T: VertexType,
{
    fn connected(representative: &mut [T], a: T::IdentifierType, b: T::IdentifierType) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::{QuickFind, QuickUnion, UnionFind, VertexType};
    use core::mem::size_of;

    #[test]
    fn test_qf_sz() {
        assert_eq!(
            size_of::<UnionFind::<'_, QuickFind, u32, 32>>(),
            size_of::<[u32; 32]>()
        );
        assert_eq!(
            size_of::<UnionFind::<'_, QuickFind, u8, 32>>(),
            size_of::<[u8; 32]>()
        );
        assert_eq!(
            size_of::<UnionFind::<'_, QuickFind, usize, 32>>(),
            size_of::<[usize; 32]>()
        );
    }

    #[test]
    fn test_wqupc_sz() {
        assert_eq!(
            size_of::<UnionFind::<'_, QuickUnion, usize, 32>>(),
            size_of::<[usize; 32]>() * 2
        );

        assert_eq!(
            size_of::<UnionFind::<'_, QuickUnion, usize, 32>>(),
            size_of::<[usize; 32]>() + size_of::<[usize; 32]>()
        );
    }

    #[derive(Clone, Copy)]
    pub struct CityVertex<'a> {
        pub id: u8,
        pub name: &'a str,
        pub road_cost: u32,
    }

    impl<'a> CityVertex<'a> {
        pub fn new(id: u8, name: &'a str, road_cost: u32) -> Self {
            Self {
                id,
                name,
                road_cost,
            }
        }
    }

    impl PartialEq for CityVertex<'_> {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl PartialOrd for CityVertex<'_> {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            Some(self.id.cmp(&other.id))
        }
    }

    impl Eq for CityVertex<'_> {}

    impl VertexType for CityVertex<'_> {
        type IdentifierType = u8;

        fn id(&self) -> u8 {
            self.id
        }

        fn usize(a: u8) -> usize {
            a as usize
        }
    }
}
