#![no_std]
#![cfg_attr(not(debug_assertions), deny(warnings))]
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

pub mod quickfind;
pub mod quickunion;

pub use crate::quickfind::QuickFind;
pub use crate::quickunion::QuickUnion;

pub trait IndexType: Copy + Eq {
    fn usize(self) -> usize;
}

macro_rules! generate_index_type_impl{
    ($($num_type:ident), *) => {
        $(
            impl IndexType for $num_type {
                fn usize(self) -> usize {
                    self as usize
                }
            }
        )*
    };
}

generate_index_type_impl!(u8, u16, u32, u64, usize);

pub type UnionFindQuickUnion<T, const N: usize> = UnionFind<QuickUnion, T, N, N>;

pub struct UnionFind<A, T, const N: usize, const M: usize = 0>
where
    T: IndexType,
    A: Union<T, N, M> + Find<T, N> + Connected<T, N>,
{
    pub representative: A::RepresentativeContainer<T, N>,
    pub rank: A::RankContainer<T, M>,
    algorithm: A,
}

impl<A, T, const N: usize, const M: usize> UnionFind<A, T, N, M>
where
    T: IndexType,
    A: Union<T, N, M> + Find<T, N> + Connected<T, N> + Default,
    Self: Default,
{
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connected(&self, a: T, b: T) -> bool {
        self.algorithm.connected(&self.representative, a, b)
    }

    pub fn find(&self, a: T) -> T {
        self.algorithm.find(&self.representative, a)
    }

    pub fn union(&mut self, a: T, b: T) {
        self.algorithm
            .union(&mut self.representative, &mut self.rank, a, b)
    }
}

pub trait WithContainer {
    type RankContainer<T: IndexType, const N: usize>: AsRef<[T]> + AsMut<[T]>;
    type RepresentativeContainer<R: IndexType, const N: usize>: AsRef<[R]> + AsMut<[R]>;
}

pub trait Union<T, const N: usize, const M: usize>
where
    Self: WithContainer,
    T: IndexType,
{
    fn union(
        &mut self,
        representative: &mut Self::RepresentativeContainer<T, N>,
        rank: &mut Self::RankContainer<T, M>,
        a: T,
        b: T,
    );
}

pub trait Find<T, const N: usize>
where
    Self: WithContainer,
    T: IndexType,
{
    fn find(&self, representative: &Self::RepresentativeContainer<T, N>, a: T) -> T;
}

pub trait Connected<T, const N: usize>
where
    Self: WithContainer,
    T: IndexType,
{
    fn connected(&self, representative: &Self::RepresentativeContainer<T, N>, a: T, b: T) -> bool;
}

#[cfg(test)]
mod tests {
    use crate::{QuickFind, QuickUnion, UnionFind};
    use core::mem::size_of;

    #[test]
    fn test_qf_sz() {
        assert_eq!(
            size_of::<UnionFind::<QuickFind, u32, 32>>(),
            size_of::<[u32; 32]>()
        );
        assert_eq!(
            size_of::<UnionFind::<QuickFind, u8, 32>>(),
            size_of::<[u8; 32]>()
        );
        assert_eq!(
            size_of::<UnionFind::<QuickFind, usize, 32>>(),
            size_of::<[usize; 32]>()
        );
    }

    #[test]
    fn test_wqupc_sz() {
        assert_eq!(
            size_of::<UnionFind::<QuickUnion, usize, 32, 32>>(),
            size_of::<[usize; 32]>() * 2
        );

        assert_eq!(
            size_of::<UnionFind::<QuickUnion, usize, 32, 32>>(),
            size_of::<[usize; 32]>() + size_of::<[usize; 32]>()
        );
    }
}
