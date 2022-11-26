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

pub struct UnionFind<A, T, const N: usize, R = usize, const M: usize = N>
where
    T: Copy,
    R: Copy,
    A: Union + Find + Connected,
{
    pub representative: A::RepresentativeContainer<T, N>,
    pub rank: A::RankContainer<R, M>,
    algorithm: A,
}

impl<A, T, R, const N: usize, const M: usize> UnionFind<A, T, N, R, M>
where
    T: Copy,
    R: Copy,
    A: Union + Find + Connected,
{
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn connected(&self) -> bool {
        self.algorithm.connected()
    }

    pub fn find(&self) -> T {
        // self.algorithm.find()
        unimplemented!()
    }
}

pub trait WithContainer {
    type RankContainer<T: Copy, const N: usize>: AsRef<[T]> + AsMut<[T]>;
    type RepresentativeContainer<R: Copy, const N: usize>: AsRef<[R]> + AsMut<[R]>;
}

pub trait Union
where
    Self: WithContainer,
{
    fn union(&self) {}
}

pub trait Find
where
    Self: WithContainer,
{
    fn find(&self) {}
}

pub trait Connected
where
    Self: WithContainer,
{
    fn connected(&self) -> bool;
}

pub struct QuickFind;
pub struct QuickUnion<const WEIGHTED: bool = true, const COMPRESS_PATH: bool = true>;

impl WithContainer for QuickFind {
    type RankContainer<T: Copy, const N: usize> = [T; 0];
    type RepresentativeContainer<R: Copy, const N: usize> = [R; N];
}

impl Connected for QuickFind {
    fn connected(&self) -> bool {
        todo!()
    }
}

impl Union for QuickFind {
    fn union(&self) {}
}

impl Find for QuickFind {
    fn find(&self) {}
}

impl WithContainer for QuickUnion {
    type RankContainer<T: Copy, const N: usize> = [T; N];
    type RepresentativeContainer<R: Copy, const N: usize> = [R; N];
}

impl Connected for QuickUnion {
    fn connected(&self) -> bool {
        todo!()
    }
}

impl Union for QuickUnion {
    fn union(&self) {}
}

impl Find for QuickUnion {
    fn find(&self) {}
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
            size_of::<UnionFind::<QuickUnion, usize, 32>>(),
            size_of::<[usize; 32]>() * 2
        );

        assert_eq!(
            size_of::<UnionFind::<QuickUnion, usize, 32, u32>>(),
            size_of::<[usize; 32]>() + size_of::<[u32; 32]>()
        );
    }

    #[test]
    fn test_qf() {
        // let uf_qf = UnionFind::<QuickFind, u32, 32>::new();
    }
}
