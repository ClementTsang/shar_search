# shar_search

Implements Shar's algorithm in Rust for a (theoretically) branchless binary search. Inspired by [Beautiful Branchless Binary Search](https://probablydance.com/2023/04/27/beautiful-branchless-binary-search/) and [Beautiful Binary Search in D](https://muscar.eu/shar-binary-search-meta.html).

Currently WIP - unfortunately, Rust/clang don't seem to make it easy to make the loop unroll properly in a branchless manner.

