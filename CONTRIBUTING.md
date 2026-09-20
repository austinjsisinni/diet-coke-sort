# Contributing

Thank you for helping keep FizzSort crisp.

## Change process

1. Open an issue for a behavioral or architectural change so its requirement
   and tradeoffs can be agreed before implementation.
2. Create a focused branch and keep commits reviewable.
3. Update requirements, design, tests, and `CHANGELOG.md` with behavior changes.
4. Run the full acceptance gate documented in `README.md`.
5. Open a pull request describing the affected requirement IDs, risks, and test
   evidence.

All contributions must preserve stable sorting, safe Rust, deterministic tests,
the comparator-panic guarantee, and MSRV support unless the project baseline is
explicitly revised.

By contributing, you agree that your contribution is licensed under the MIT
License included in this repository.

