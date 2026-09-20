# DietCokeSort

[![Assurance](https://github.com/austinjsisinni/diet-coke-sort/actions/workflows/assurance.yml/badge.svg)](https://github.com/austinjsisinni/diet-coke-sort/actions/workflows/assurance.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**DietCokeSort** is a crisp, stable, adaptive sorting algorithm for Rust. It has
zero sugar, zero dependencies, and zero tolerance for disorder. One calorie;
`O(n log n)` worst-case comparisons. The math is refreshing.

<p align="center">
  <img src="assets/dietcoke-sort-nutrition-facts.png" width="640" alt="DietCokeSort nutrition facts: already-sorted comparisons n - 1; worst-case comparisons O(n log n); auxiliary storage O(n) indices; value swaps at most n - 1; stable; no Clone required; unsafe code forbidden">
</p>

```rust
let mut cans = [5, 1, 4, 1, 3];
let report = diet_coke_sort::sort(&mut cans);

assert_eq!(cans, [1, 1, 3, 4, 5]);
assert_eq!(report.len(), 5);
```

## Why did the sort cross the soda aisle?

To get to the *ordered* side. We'll be here all week.

- **Stable:** equal values keep their order. Nobody cuts the fountain line.
- **Adaptive:** sorted input needs exactly `n - 1` comparisons—Caffeine Free
  Anxiety.
- **Low movement:** at most `n - 1` value swaps. Large values can chill in the
  fridge while their indices do the cardio.
- **Panic isolation:** comparisons finish before values move. A panicking
  comparator leaves the input unchanged unless it performs interior mutation.
- **Auditable:** safe Rust, deterministic metrics, traced tests, and no runtime
  dependencies. The ingredient list is shorter than a mini can.

This is a useful blend, not a claim that we found a secret complexity formula.
Natural runs, binary insertion, stable merging, and permutation cycles are
established techniques. Diet Coke Lime brings the lime; DietCokeSort brings the
particular combination and assurance story.

## How the can opens

1. **Classic—discover runs.** Find nondecreasing or strictly decreasing streams.
   Strict decrease lets reversal preserve stable ties.
2. **Lime—crisp short runs.** Stable binary insertion extends runs to 32
   elements. That's elements, not fluid ounces.
3. **Feisty Cherry—merge.** Balanced passes merge indices, choosing the left
   index on equality. The values remain calmer than an unopened can.
4. **Caffeine Free—clean finish.** Apply the completed permutation in cycles
   with at most `n - 1` swaps. No jitters, no sticky aftertaste.

The fine print: `O(n)` index storage and indirect comparisons can lose to
Rust's highly tuned standard sorts for small, cheap values. Choose DietCokeSort
when stability, auditability, comparator-panic isolation, or moving large values
sparingly matters more than winning every microbenchmark. Sometimes the
original formula is the right order; sometimes you want Black Cherry Vanilla.

## Flavor interface: pick your pour

The flavors are zero-sized Rust types implementing `SortFlavor<T>`. Dispatch is
static, so the interface adds no runtime tag or mystery syrup.

| Flavor | Best served with | Engine | Report |
|---|---|---|---|
| `DietCoke` | `i8` through `i128`, `u8` through `u128`, strings, big integers, exact decimals, any `Ord` type | DietCokeSort | `SortReport` |
| `DietCokeWithLime` | `f64` | DietCokeSort + `f64::total_cmp` | `SortReport` |
| `DietCokeWithLimeCaffeineFree` | `f32` | DietCokeSort + `f32::total_cmp` | `SortReport` |
| `CocaColaSort` | any `Ord` type | Rust's `slice::sort` | `()`—no tasting notes; how lame |

```rust
use diet_coke_sort::{
    CocaColaSort, DietCoke, DietCokeWithLime,
    DietCokeWithLimeCaffeineFree, SortFlavor,
};

let mut integers = [i128::MAX, 0, i128::MIN];
let report = DietCoke.sort(&mut integers);
assert_eq!(integers, [i128::MIN, 0, i128::MAX]);
assert_eq!(report.len(), 3);

let mut precise = [f64::NAN, f64::from_bits(1), -0.0, 0.0, f64::MAX];
DietCokeWithLime.sort(&mut precise);
assert!(precise.windows(2).all(|pair| pair[0].total_cmp(&pair[1]).is_le()));

let mut compact = [f32::INFINITY, f32::MIN_POSITIVE, f32::NEG_INFINITY];
DietCokeWithLimeCaffeineFree.sort(&mut compact);
assert_eq!(compact, [f32::NEG_INFINITY, f32::MIN_POSITIVE, f32::INFINITY]);

let mut full_sugar = [3, 1, 2];
CocaColaSort.sort(&mut full_sugar);
assert_eq!(full_sugar, [1, 2, 3]);
```

### Precision without the sugar coating

The lime flavors use Rust's `total_cmp`, which follows IEEE 754 `totalOrder`:
negative and positive zero are distinct, subnormals keep their exact bits, and
NaNs receive a deterministic place instead of making comparison shrug. Sorting
does no arithmetic, normalization, parsing, or conversion, so it cannot round
the values. The active [IEEE 754-2019 standard](https://standards.ieee.org/ieee/754/6210/)
covers binary and decimal floating-point arithmetic; Rust documents the exact
[`f64::total_cmp` sequence](https://doc.rust-lang.org/stable/core/primitive.f64.html#method.total_cmp).

For precision beyond built-in floats, use `DietCoke` with an arbitrary-precision
type implementing `Ord`, or use `sort_by` with that type's exact comparator.
The algorithm only asks, “which comes first?” It never takes a sip of your
digits.

Run `cargo run --example flavor` for a tasting flight. Discontinued Diet Coke
variants are welcome; programs are famously bad at checking store inventory.

## NASA-inspired assurance, condensed into one can

This project borrows lightweight practices from NASA's Software Engineering
Handbook: defined coding rules, static analysis, repeatable tests, and
requirements-to-test traceability. It is **not** NASA software, certified by
NASA, or affiliated with NASA. We aim for the stars, but the can stays on the
desk.

| Requirement | Verification |
|---|---|
| Correct total ordering and no lost values | nominal and deterministic stress tests against Rust's sort |
| Stable ties and custom comparators | duplicate-heavy, descending-run, and custom-order tests |
| Empty and singleton inputs | boundary tests |
| Comparator panic leaves input unchanged | injected-panic test |
| Exact extreme, subnormal, adjacent-float, signed-zero, infinity, and NaN handling | bit-for-bit `f32` and `f64` reference-order tests |
| Arbitrary-width precision | 512-bit ordered-value test with no conversion or arithmetic |
| Determinism and complexity invariants | report, sorted-stream, and swap-bound tests |
| Safe, documented, portable Rust | forbidden `unsafe`, Clippy, rustdoc, doctests, three-OS CI, Rust 1.74 MSRV |

The source uses `#![forbid(unsafe_code)]`; compiler warnings, Clippy `all` and
`pedantic`, formatting drift, rustdoc warnings, and test failures block the
build. Tests are deterministic and run on every push and pull request. That's
quality fizz-urance without a 40-page beverage menu.

NASA references: [coding standards](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695445/SWE-061+-+Coding+Standards),
[unit testing](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695446/SWE-062+-+Unit+Test),
[perform testing](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695449/SWE-066+-+Perform+Testing),
[traceability](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695427/SWE-052+-+Bidirectional+Traceability),
and [static analysis](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695494/SWE-135+-+Static+Analysis).

## Refill responsibly

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

Changes should stay focused, update tests with behavior, and keep the guarantees
above. Open a pull request—or a pour request, if your children have already
left the room.

## License and legal aftertaste

Licensed under the [MIT License](LICENSE): permissive, refreshing, and free of
high-fructose license syrup.

Diet Coke and its variant names are trademarks of The Coca-Cola Company. This
independent open-source project is not endorsed by or affiliated with The
Coca-Cola Company. No brand artwork or trade dress is used.
