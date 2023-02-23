use pulau_rs::{ByRank, QuickUnion, UnionFind};

fn main() {
    let (mut buf, mut heuristic) = (vec![0; 10], vec![0; 10]);

    let mut uf =
        UnionFind::<QuickUnion<ByRank<true>, true>, u32, 10>::new(&mut buf, &mut heuristic);

    uf.union_sets(1, 2);
    uf.union_sets(2, 3);

    println!("1 and 3 is: {}", uf.connected(1, 3));
}
