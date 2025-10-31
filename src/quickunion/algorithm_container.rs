//! AlgorithmContainer trait implementations for QuickUnion

use rand_core::RngCore;

use crate::{
    quickunion::heuristics::ByRandom, rng::PhantomRng, AlgorithmContainer, Borrowed, ByRank,
    BySize, Fat, Owned, QuickUnion, Thin, Unweighted, VertexType,
};

/// Implements AlgorithmContainer for QuickUnion with `Owned` heuristics.
macro_rules! impl_owned {
    ($($heur:ident),* $(,)?) => {
        $(
            impl AlgorithmContainer for QuickUnion<$heur<Owned>> {
                type HeuristicKind<'a> = $heur;
                type HeuristicContainer<'a, const N: usize> = [usize; N];
                type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = [V; N];
                type RngKind<'a> = PhantomRng;
            }
        )*
    };
}

/// Implements AlgorithmContainer for QuickUnion with Borrowed<Fat|Thin> heuristics.
macro_rules! impl_borrowed {
    ($($heur:ident),* $(,)?) => {
        $(
            impl<const P: bool> AlgorithmContainer for QuickUnion<$heur<Borrowed<Fat>>, P> {
                type HeuristicKind<'a> = $heur<Borrowed<Fat>>;
                type HeuristicContainer<'a, const N: usize> = &'a mut [usize];
                type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V];
                type RngKind<'a> = PhantomRng;
            }

            impl<const P: bool> AlgorithmContainer for QuickUnion<$heur<Borrowed<Thin>>, P> {
                type HeuristicKind<'a> = $heur<Borrowed<Thin>>;
                type HeuristicContainer<'a, const N: usize> = &'a mut [usize; N];
                type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V; N];
                type RngKind<'a> = PhantomRng;
            }
        )*
    };
}

/// Implements AlgorithmContainer for QuickUnion with ByRandom<R> and borrowed variants.
macro_rules! impl_random {
    () => {
        impl<R, const P: bool> AlgorithmContainer for QuickUnion<ByRandom<R>, P>
        where
            R: RngCore,
        {
            type HeuristicKind<'a> = ByRandom<R>;
            type HeuristicContainer<'a, const N: usize> = [usize; 0];
            type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = [V; N];
            type RngKind<'a> = R;
        }

        impl<R, const P: bool> AlgorithmContainer for QuickUnion<ByRandom<R, Borrowed<Fat>>, P>
        where
            R: RngCore,
        {
            type HeuristicKind<'a> = ByRandom<R, Borrowed<Fat>>;
            type HeuristicContainer<'a, const N: usize> = &'a mut [usize];
            type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V];
            type RngKind<'a> = R;
        }

        impl<R, const P: bool> AlgorithmContainer for QuickUnion<ByRandom<R, Borrowed<Thin>>, P>
        where
            R: RngCore,
        {
            type HeuristicKind<'a> = ByRandom<R, Borrowed<Thin>>;
            type HeuristicContainer<'a, const N: usize> = &'a mut [usize];
            type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V];
            type RngKind<'a> = R;
        }
    };
}

impl<const P: bool> AlgorithmContainer for QuickUnion<Unweighted<Owned>, P> {
    type HeuristicKind<'a> = Unweighted;
    type HeuristicContainer<'a, const N: usize> = [usize; 0];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = [V; N];
    type RngKind<'a> = PhantomRng;
}

impl<const P: bool> AlgorithmContainer for QuickUnion<Unweighted<Borrowed<Fat>>, P> {
    type HeuristicKind<'a> = Unweighted<Borrowed<Fat>>;
    type HeuristicContainer<'a, const N: usize> = [usize; 0];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V];
    type RngKind<'a> = PhantomRng;
}

impl<const P: bool> AlgorithmContainer for QuickUnion<Unweighted<Borrowed<Thin>>, P> {
    type HeuristicKind<'a> = Unweighted<Borrowed<Thin>>;
    type HeuristicContainer<'a, const N: usize> = [usize; 0];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V; N];
    type RngKind<'a> = PhantomRng;
}

impl_owned!(ByRank, BySize);
impl_borrowed!(ByRank, BySize);
impl_random!();
