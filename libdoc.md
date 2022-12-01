# pulau-rs
Allocation-free UnionFind library for bare metal environments

The library provides the following algorithms that is used with [UnionFind].
- QuickFind
- QuickUnion
- Weighted QuickUnion
- Weighted QuickUnion With Path Compression (Default)

## Asymptotic Complexity
| Algorithm                                        |           Struct            |  Init  |               Union |                Find |           Connected |
| :----------------------------------------------- | :-------------------------: | :----: | ------------------: | ------------------: | ------------------: |
| QuickFind                                        |         [QuickFind]         | `O(N)` |              `O(N)` |              `O(1)` |              `O(1)` |
| QuickUnion                                       | [QuickUnion<false, false>]  | `O(N)` |              `O(N)` |              `O(N)` |              `O(N)` |
| Weighted QuickUnion                              | [QuickUnion<ByRank, false>] | `O(N)` |           `O(lg N)` |           `O(lg N)` |           `O(lg N)` |
| Weighted (Rank) QuickUnion With Path Compression | [QuickUnion<ByRank, true>]  | `O(N)` | `Amortized O(α(N))` | `Amortized O(α(N))` | `Amortized O(α(N))` |
| Weighted (Size) QuickUnion With Path Compression | [QuickUnion<BySize, true>]  | `O(N)` | `Amortized O(α(N))` | `Amortized O(α(N))` | `Amortized O(α(N))` |

*Where `α` is the inverse [Ackermann function](https://en.wikipedia.org/wiki/Ackermann_function)

## Applications of UnionFind
- Checking for connected components in a graph
- Checking for cycles in a graph
- Searching for connected components in an image
- Finding minimum spanning tree using Kruskal

