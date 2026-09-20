# Project Coding Standard

This project selects the following coding rules as its implementation of the
NASA-inspired coding-standard practice.

1. Use stable Rust 2021 with a minimum supported Rust version (MSRV) of 1.74.
2. Forbid all `unsafe` code at both crate and Cargo-lint levels.
3. Deny compiler warnings, Clippy `all`, and Clippy `pedantic` in continuous
   integration. A suppression requires a nearby rationale and review.
4. Format all Rust sources with the default stable `rustfmt` configuration.
5. Document every public item and compile all documentation examples.
6. Avoid third-party runtime dependencies unless a reviewed requirement
   justifies one.
7. Prefer explicit bounds, checked indexing, saturating instrumentation, and
   small single-purpose functions.
8. Tests shall be deterministic, self-contained, and named for a traced
   requirement where directly applicable.
9. A change is acceptable only when format, static analysis, unit/integration
   tests, doctests, and documentation checks pass.
10. Public behavior changes require corresponding updates to requirements,
    design, tests, and release notes.

The CI workflow is the automated compliance mechanism. Pull-request review is
the independent human check when contributions are made through GitHub.

