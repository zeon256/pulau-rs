<p align="center">
  <img src="./logo.png">
</p>
<p align="center">
  <a href="https://github.com/zeon256/pulau-rs/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/lta-rs/lta-models?style=flat-square"/>
  </a>
  <a href="https://docs.rs/lta">
    <img src="https://img.shields.io/badge/docs-docs.rs-blue?style=flat-square&logo=Docs.rs"/>
  </a>
  <a href="https://zeon256.github.io/pulau-rs/pulau_rs/">
    <img src="https://img.shields.io/badge/docs-main--branch-red?style=flat-square&logo=Docs.rs"/>
  </a>
  <a href="https://github.com/zeon256/pulau-rs/actions">
    <img src="https://img.shields.io/github/workflow/status/zeon256/pulau-rs/rust.yml?style=flat-square"/>
  </a>
</p>

# pulau-rs
Allocation-free UnionFind library for bare metal environments

The library provides the following algorithms that is used with `UnionFind`.
- QuickFind
- QuickUnion
- Weighted QuickUnion
- Weighted QuickUnion With Path Compression (Default)

## Asymptotic Complexity
| Algorithm                                        |             Struct              |  Init  |               Union |                Find |           Connected |
| :----------------------------------------------- | :-----------------------------: | :----: | ------------------: | ------------------: | ------------------: |
| QuickFind                                        |           `QuickFind`           | `O(N)` |              `O(N)` |              `O(1)` |              `O(1)` |
| QuickUnion                                       | `QuickUnion<UnWeighted, false>` | `O(N)` |              `O(N)` |              `O(N)` |              `O(N)` |
| Weighted (Rank) QuickUnion With Path Compression |   `QuickUnion<ByRank, true>`    | `O(N)` | `Amortized O(α(N))` | `Amortized O(α(N))` | `Amortized O(α(N))` |
| Weighted (Size) QuickUnion With Path Compression |   `QuickUnion<BySize, true>`    | `O(N)` | `Amortized O(α(N))` | `Amortized O(α(N))` | `Amortized O(α(N))` |

*Where `α` is the inverse [Ackermann function](https://en.wikipedia.org/wiki/Ackermann_function)

## Applications of UnionFind
- Checking for connected components in a graph
- Checking for cycles in a graph
- Searching for connected components in an image
- Finding minimum spanning tree using Kruskal