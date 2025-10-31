//! AlgorithmContainer trait implementations for QuickUnion

use rand_core::RngCore;

use crate::{AlgorithmContainer, Borrowed, ByRank, BySize, QuickUnion, Thin, Unweighted, VertexType, quickunion::heuristics::ByRandom, rng::PhantomRng};

impl AlgorithmContainer for QuickUnion<ByRank> {
    type HeuristicKind<'a> = ByRank;
    type HeuristicContainer<'a, const N: usize> = [usize; N];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = [V; N];
    type RngKind<'a> = PhantomRng;
}

impl AlgorithmContainer for QuickUnion<BySize> {
    type HeuristicKind<'a> = BySize;
    type HeuristicContainer<'a, const N: usize> = [usize; N];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = [V; N];
    type RngKind<'a> = PhantomRng;
}

impl<const P: bool> AlgorithmContainer for QuickUnion<Unweighted, P> {
    type HeuristicKind<'a> = Unweighted;
    type HeuristicContainer<'a, const N: usize> = [usize; 0];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = [V; N];
    type RngKind<'a> = PhantomRng;
}

impl<const P: bool> AlgorithmContainer for QuickUnion<Unweighted<Borrowed>, P> {
    type HeuristicKind<'a> = Unweighted<Borrowed>;
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

impl<const P: bool> AlgorithmContainer for QuickUnion<BySize<Borrowed>, P> {
    type HeuristicKind<'a> = BySize<Borrowed>;
    type HeuristicContainer<'a, const N: usize> = &'a mut [usize];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V];
    type RngKind<'a> = PhantomRng;
}

impl<const P: bool> AlgorithmContainer for QuickUnion<BySize<Borrowed<Thin>>, P> {
    type HeuristicKind<'a> = BySize<Borrowed<Thin>>;
    type HeuristicContainer<'a, const N: usize> = &'a mut [usize; N];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V; N];
    type RngKind<'a> = PhantomRng;
}

impl<const P: bool> AlgorithmContainer for QuickUnion<ByRank<Borrowed>, P> {
    type HeuristicKind<'a> = ByRank<Borrowed>;
    type HeuristicContainer<'a, const N: usize> = &'a mut [usize];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V];
    type RngKind<'a> = PhantomRng;
}

impl<const P: bool> AlgorithmContainer for QuickUnion<ByRank<Borrowed<Thin>>, P> {
    type HeuristicKind<'a> = ByRank<Borrowed<Thin>>;
    type HeuristicContainer<'a, const N: usize> = &'a mut [usize; N];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V; N];
    type RngKind<'a> = PhantomRng;
}

impl<R, const P: bool> AlgorithmContainer for QuickUnion<ByRandom<R>, P>
where
    R: RngCore + AsMut<R>,
{
    type HeuristicKind<'a> = ByRandom<R>;
    type HeuristicContainer<'a, const N: usize> = [usize; 0];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = [V; N];
    type RngKind<'a> = R;
}

impl<R, const P: bool> AlgorithmContainer for QuickUnion<ByRandom<R, Borrowed>, P>
where
    R: RngCore + AsMut<R>,
{
    type HeuristicKind<'a> = ByRandom<R, Borrowed>;
    type HeuristicContainer<'a, const N: usize> = &'a mut [usize];
    type RepresentativeContainer<'a, V: VertexType + 'a, const N: usize> = &'a mut [V];
    type RngKind<'a> = R;
}
