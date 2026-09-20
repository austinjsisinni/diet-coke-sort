# Software Requirements and Traceability

Status: baseline for release 0.1.0  
Scope: the `diet-coke-sort` Rust library

The word **shall** marks a verifiable requirement. The table is bidirectional:
each requirement names its verification procedure, and every normative test
procedure below names the requirement it verifies.

| ID | Requirement | Verification |
|---|---|---|
| REQ-F-001 | The library shall order every input according to the supplied total-order comparator. | `req_f_001_sorts_nominal_input`; `deterministic_stress_matches_standard_stable_sort`; CI-TEST |
| REQ-F-002 | The library shall retain source order among elements that compare equal. | `req_f_002_preserves_equal_element_order`; `strict_descending_run_does_not_reverse_equal_values`; `comparison_equality_is_stable_across_many_runs` |
| REQ-F-003 | The output shall contain exactly the input elements with no duplication or loss. | `deterministic_stress_matches_standard_stable_sort`; source review of cycle application |
| REQ-F-004 | Empty and singleton slices shall be accepted without comparison or mutation. | `req_f_004_handles_empty_and_singleton_inputs` |
| REQ-F-005 | The library shall support caller-defined total orders. | `req_f_005_supports_custom_ordering`; doctest for `sort_by` |
| REQ-S-001 | The implementation shall contain no `unsafe` Rust. | `#![forbid(unsafe_code)]`; CI-CLIPPY |
| REQ-S-002 | If the comparator panics, the library shall not have mutated the input slice. | `req_s_002_comparator_panic_leaves_input_unchanged` |
| REQ-Q-001 | Given the same input and comparator, the result and report shall be deterministic. | `req_q_001_report_is_deterministic` |
| REQ-Q-002 | All public items shall have API documentation, and examples shall compile. | CI-DOC; CI-DOCTEST |
| REQ-P-001 | For a valid total order, worst-case comparator calls shall be `O(n log n)` and an already sorted input shall take `n - 1` calls. | Design analysis; `req_p_001_recognizes_an_already_sorted_stream` |
| REQ-P-002 | Auxiliary storage shall be `O(n)`. | Design analysis and source review |
| REQ-P-003 | Applying the completed permutation shall require at most `n - 1` swaps. | Cycle-decomposition design analysis; stress tests observe the report invariant |

## Verification procedure index

| Procedure | Command or method | Requirements covered |
|---|---|---|
| CI-FMT | `cargo fmt --all -- --check` | Coding-standard conformance |
| CI-CLIPPY | `cargo clippy --all-targets --all-features -- -D warnings` | REQ-S-001 and coding-standard conformance |
| CI-TEST | `cargo test --all-targets` on Linux, Windows, macOS, and MSRV | REQ-F-001 through REQ-F-005, REQ-S-002, REQ-Q-001, REQ-P-001 |
| CI-DOCTEST | `cargo test --doc` | REQ-F-001, REQ-F-005, REQ-Q-002 |
| CI-DOC | `RUSTDOCFLAGS="-D warnings" cargo doc --no-deps` | REQ-Q-002 |
| Design analysis | Review invariants and bounds in `docs/design.md` | REQ-P-001 through REQ-P-003 |
| Source review | Review `src/lib.rs` and the pull-request diff | REQ-F-003, REQ-S-001, REQ-P-002 |

## Assumptions and constraints

- The comparator defines a deterministic total order for the duration of a
  call. As with standard-library sorting, violating that contract makes the
  ordering result unspecified, but does not compromise memory safety.
- Memory allocation must succeed. Allocation failure behavior follows the Rust
  standard library and target environment.
- This general-purpose library is not classified as safety-critical software.
  Use in a safety-critical system requires system-specific classification,
  hazard analysis, tool qualification, coverage objectives, and independent
  assurance beyond this repository.

