# Diet Coke Sort

[![Assurance](https://github.com/austinjsisinni/diet-coke-sort/actions/workflows/assurance.yml/badge.svg)](https://github.com/austinjsisinni/diet-coke-sort/actions/workflows/assurance.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**FizzSort** is a crisp, stable, adaptive sorting algorithm for Rust. Its shape
is inspired by the sensory arc of a cold can of Diet Coke: find the smooth
streams already present, sharpen the short ones, let ordered bubbles rise and
merge, then finish cleanly with minimal movement of the actual values.

```rust
let mut values = [5, 1, 4, 1, 3];
let report = diet_coke_sort::sort(&mut values);

assert_eq!(values, [1, 1, 3, 4, 5]);
assert_eq!(report.len(), 5);
```

## Why another sorting algorithm?

`O(n log n)` is optimal in the general comparison model, not a compromise.
FizzSort combines familiar, well-understood techniques in a deliberately
assurable package:

- **Stable:** equal elements keep their original order.
- **Adaptive:** already ordered data needs exactly `n - 1` comparisons.
- **Low movement:** the final permutation takes at most `n - 1` swaps, useful
  when values are large or costly to move.
- **Comparator-panic isolation:** comparisons happen against indices before the
  input is mutated. A panicking comparator leaves the input unchanged, aside
  from any interior mutation the comparator itself performs.
- **Auditable:** safe Rust only, no dependencies, deterministic measurements,
  requirements-to-test traceability, and a multi-platform verification gate.

This is not a claim of a new complexity result. Natural merge sorting, binary
insertion, and permutation application are established techniques. FizzSort's
value is their particular combination and its explicit assurance story.

## How it tastes

1. **Open the can — discover streams.** Scan the input for ascending or
   strictly descending natural runs. Strictness protects stability when a
   descending run is reversed.
2. **Crisp the sip — extend short runs.** Grow runs to 32 elements with stable
   binary insertion. Because 32 is fixed, this work stays linear overall.
3. **Release the fizz — merge indices.** Stably merge adjacent runs in balanced
   passes. Only an index permutation moves during comparisons.
4. **Clean finish — apply once.** Convert the sorted source order to cycles and
   rearrange the values with at most `n - 1` swaps.

| Property | FizzSort |
|---|---|
| Best case | `O(n)` comparisons |
| Worst case | `O(n log n)` comparisons |
| Auxiliary space | `O(n)` indices |
| Stable | Yes |
| Requires `Clone` | No |
| Unsafe code | Forbidden |
| Comparator panic before mutation | Input remains unchanged |

The tradeoff is honest: indirect comparisons and `O(n)` index storage can make
FizzSort slower than Rust's highly tuned standard-library sorts for small,
cheap values. Prefer FizzSort when stability, auditability, panic isolation, or
low movement of large values is more important than universal throughput.

## API

```rust
use diet_coke_sort::{sort, sort_by};

let mut ascending = [3, 1, 2];
let report = sort(&mut ascending);
assert_eq!(ascending, [1, 2, 3]);
assert!(report.comparisons() > 0);

let mut descending = [3, 1, 2];
sort_by(&mut descending, |left, right| right.cmp(left));
assert_eq!(descending, [3, 2, 1]);
```

Run the sensory example with `cargo run --example flavor`.

## Assurance approach

The project applies lightweight practices inspired by NASA's Software
Engineering Requirements and Handbook, including a selected coding standard,
static analysis, repeatable automated testing, and bidirectional traceability.
It is **not** NASA software, NASA-certified, or affiliated with NASA.

- [Requirements and traceability](docs/requirements.md)
- [Design description](docs/design.md)
- [Coding standard](docs/coding-standard.md)
- [Software test plan](docs/test-plan.md)
- [Assurance and release plan](docs/assurance-plan.md)

NASA references:

- [SWE-061 — Coding Standards](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695445/SWE-061+-+Coding+Standards)
- [SWE-062 — Unit Test](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695446/SWE-062+-+Unit+Test)
- [SWE-066 — Perform Testing](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695449/SWE-066+-+Perform+Testing)
- [SWE-052 — Bidirectional Traceability](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695427/SWE-052+-+Bidirectional+Traceability)
- [SWE-135 — Static Analysis](https://swehb.nasa.gov/spaces/SWEHBVD/pages/102695494/SWE-135+-+Static+Analysis)

## Development

The acceptance gate is:

```text
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
```

See [CONTRIBUTING.md](CONTRIBUTING.md) before proposing a change.

## License and trademarks

Licensed under the [MIT License](LICENSE).

Diet Coke is a trademark of The Coca-Cola Company. This independent open-source
project is not endorsed by or affiliated with The Coca-Cola Company. No brand
artwork or trade dress is used.
