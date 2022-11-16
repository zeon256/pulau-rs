#![no_std]
use core::marker::PhantomData;

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

impl<A, T, R, const N: usize> UnionFind<A, T, N, R>
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
        self.algorithm.find()
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
    fn find(&self) -> T {}
}

pub trait Connected
where
    Self: WithContainer,
{
    fn connected(&self) -> bool;
}

pub struct QuickFind;

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

#[cfg(test)]
mod tests {
    use crate::{UnionFind, QuickFind, Find, Union, Connected};

    fn test_qf() {
        let uf_qf = UnionFind::<QuickFind, u32, 32>::new();
    }
}
