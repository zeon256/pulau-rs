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

use core::ops::{AddAssign, Index, IndexMut};

pub use crate::quickfind::QuickFind;
pub use crate::quickunion::QuickUnion;
pub use crate::quickunion::{ByRank, BySize, Unweighted};

pub trait IndexType: Copy + Eq + PartialOrd + AddAssign<Self> {
    fn usize(self) -> usize;
    fn one() -> Self;
}

macro_rules! generate_index_type_impl{
    ($($num_type:ident), *) => {
        $(
            impl IndexType for $num_type {

                #[inline(always)]
                fn usize(self) -> usize {
                    self as usize
                }

                #[inline(always)]
                fn one() -> Self {
                    1
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
/// - `T` - Any unsigned integral types, i.e., [`u8`], [`u16`], [`u32`], [`u64`], [`usize`]
/// - `N` - Size of internal representative buffer
/// - `M` - Size of internal heuristic buffer, this defaults to sz `N`, but it must be 0 if you are using [`QuickFind`]
///
/// # Example
/// ```rust
/// use pulau_rs::{UnionFind, QuickFind, QuickUnion};
/// fn make_uf_quickfind() {
///     // construct with quickfind algorithm with fixed size 10
///     let mut uf = UnionFind::<QuickFind, u32, 10, 0>::new();
/// }
///
/// fn make_uf_quickunion() {
///     // construct with weighted quickunion with path compression algorithm with fixed size 10
///     let mut uf = UnionFind::<QuickUnion, u32, 10>::new();
/// }
/// ```
///
/// # Size Guarantees
/// Size of [`UnionFind`] depends on whether the algorithm you have chosen is weighted
///
/// If it's weighted then, size of [`UnionFind`] is `2 * T * N`
///
/// Else it will be `T * N`
pub struct UnionFind<A, T, const N: usize, const M: usize = N>
where
    T: IndexType,
    A: Union<T, N, M> + Find<T, N> + Connected<T, N>,
{
    representative: A::RepresentativeContainer<T, N>,
    heuristic: A::HeuristicContainer<T, M>,
    algorithm: A,
}

impl<A, T, const N: usize, const M: usize> UnionFind<A, T, N, M>
where
    T: IndexType,
    A: Union<T, N, M> + Find<T, N> + Connected<T, N> + Default,
    Self: Default,
{
    /// Construct a new [`UnionFind`] structure based on the type parameters provided
    pub fn new() -> Self {
        Self::default()
    }

    /// Checks whether 2 nodes are connected to each other
    pub fn connected(&mut self, a: T, b: T) -> bool {
        self.algorithm.connected(&mut self.representative, a, b)
    }

    /// Finds a node
    pub fn find(&mut self, a: T) -> T {
        self.algorithm.find(&mut self.representative, a)
    }

    /// Unions 2 node. If those 2 nodes are already part of the same component
    /// then this does nothing
    pub fn union_sets(&mut self, a: T, b: T) {
        self.algorithm
            .union_sets(&mut self.representative, &mut self.heuristic, a, b)
    }

    /// Gets the representative slice
    pub fn representative(&self) -> &A::RepresentativeContainer<T, N> {
        &self.representative
    }

    /// Gets the heuristic slice
    pub fn heuristic(&self) -> &A::HeuristicContainer<T, M> {
        &self.heuristic
    }
}

pub trait WithContainer {
    type HeuristicContainer<T: IndexType, const N: usize>: AsRef<[T]>
        + AsMut<[T]>
        + Index<usize, Output = T>
        + IndexMut<usize, Output = T>;

    type RepresentativeContainer<R: IndexType, const N: usize>: AsRef<[R]>
        + AsMut<[R]>
        + Index<usize, Output = R>
        + IndexMut<usize, Output = R>;
}

pub trait Union<T, const N: usize, const M: usize>
where
    Self: WithContainer,
    T: IndexType,
{
    fn union_sets(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        heuristic: &mut Self::HeuristicContainer<T, M>,
        a: T,
        b: T,
    );
}

pub trait Find<T, const N: usize>
where
    Self: WithContainer,
    T: IndexType,
{
    fn find(&mut self, representative: &mut Self::RepresentativeContainer<T, N>, a: T) -> T;
}

pub trait Connected<T, const N: usize>
where
    Self: WithContainer,
    T: IndexType,
{
    fn connected(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        a: T,
        b: T,
    ) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::{QuickFind, QuickUnion, UnionFind};
    use core::mem::size_of;

    #[test]
    fn test_qf_sz() {
        assert_eq!(
            size_of::<UnionFind::<QuickFind, u32, 32, 0>>(),
            size_of::<[u32; 32]>()
        );
        assert_eq!(
            size_of::<UnionFind::<QuickFind, u8, 32, 0>>(),
            size_of::<[u8; 32]>()
        );
        assert_eq!(
            size_of::<UnionFind::<QuickFind, usize, 32, 0>>(),
            size_of::<[usize; 32]>()
        );
    }

    #[test]
    fn test_wqupc_sz() {
        assert_eq!(
            size_of::<UnionFind::<QuickUnion, usize, 32>>(),
            size_of::<[usize; 32]>() * 2
        );

        assert_eq!(
            size_of::<UnionFind::<QuickUnion, usize, 32>>(),
            size_of::<[usize; 32]>() + size_of::<[usize; 32]>()
        );
    }
}
