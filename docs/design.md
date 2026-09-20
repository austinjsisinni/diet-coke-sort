# Software Design Description

## Purpose

FizzSort provides a stable, adaptive comparison sort while isolating fallible
comparison work from mutation of caller-owned values. The implementation uses
only safe Rust and has no third-party dependencies.

## Architecture

The library exposes two operations:

- `sort`: ascending order for `T: Ord`.
- `sort_by`: caller-supplied total-order comparator.

Both return `SortReport`, a saturating set of deterministic execution counts.

The internal pipeline has four phases.

### 1. Natural-run discovery

An index vector initially contains `0..n`. Adjacent referenced values are
scanned to identify nondecreasing or strictly decreasing runs. A decreasing run
is reversed. Requiring strict decrease is the stability invariant: equal source
elements are never reversed.

### 2. Crisp-run extension

Runs shorter than `CRISP_RUN` (32) are extended with binary insertion. The
insertion point is the upper bound, so a new index follows all equal indices
already in the run. At most 31 elements per run are inserted; therefore this
phase performs `O(n)` work for a fixed run bound.

### 3. Stable balanced merge

Adjacent index runs are merged in passes. On equality, the index from the left
run wins. Each pass is linear and the number of passes is at most
`ceil(log2(r))`, where `r <= ceil(n / 32)` is the number of initial runs. Thus
the worst-case comparison count is `O(n log n)`.

### 4. Permutation application

The sorted vector maps each destination to its original source. The scratch
buffer is reused to build its inverse: each original source maps to its final
destination. Swapping values and their destination entries decomposes this
permutation into cycles. A cycle of length `k` needs `k - 1` swaps, so all
cycles together need at most `n - 1` swaps.

## Key invariants

1. `order` is always a permutation of `0..n`.
2. Every recorded run is sorted and stable under the comparator.
3. Recorded runs are contiguous, non-overlapping, and cover `0..n`.
4. Merge selects the left item on equality.
5. No element of the input slice moves until the last comparison has returned.
6. During cycle application, `targets[position]` travels with the element at
   `position` and denotes that element's required destination.

## Failure behavior

- A comparator panic can occur only in phases 1–3. Those phases mutate index
  storage, not caller values, so stack unwinding leaves the input unchanged.
- Phase 4 contains no comparator or user callback and uses bounds-checked slice
  operations. It cannot observe an invalid permutation produced by normal
  execution because invariants 1–4 preserve permutation membership.
- Counters use saturating addition so instrumentation cannot overflow.

## Complexity

| Resource | Bound |
|---|---|
| Already-sorted comparisons | `n - 1` |
| Worst-case comparisons | `O(n log n)` |
| Index initialization and scans | `O(n)` |
| Value swaps | at most `n - 1` |
| Auxiliary storage | `O(n)` machine words |

Indirect comparison can be less cache-friendly than moving small values during
the sort. The design intentionally prioritizes predictable assurance properties
and low movement of arbitrary `T` over being the fastest choice for every type.

