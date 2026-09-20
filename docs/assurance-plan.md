# Software Assurance and Release Plan

## Intent and tailoring

This repository adopts lightweight practices inspired by NASA NPR 7150.2D and
the NASA Software Engineering Handbook. It is not a compliance certification.
The project is an independent, non-safety-critical open-source library, so
center-specific processes, formal classification, independent verification and
validation, tool accreditation, and safety-critical coverage obligations are
out of scope.

## Applied practices

| NASA handbook topic | Project evidence |
|---|---|
| SWE-061, coding standards | `docs/coding-standard.md`; rustfmt; compiler and Clippy gates |
| SWE-062, unit testing | In-source deterministic tests and doctests |
| SWE-066, perform testing | `docs/test-plan.md`; CI on every change |
| SWE-052, bidirectional traceability | Requirement-to-test and procedure-to-requirement tables in `docs/requirements.md` |
| SWE-135, static analysis | Compiler diagnostics and Clippy with warnings denied |
| Configuration management | Git history, protected CI evidence, versioned Cargo manifest |

## Risk controls

| Risk | Control |
|---|---|
| Stability is broken while reversing a descending run | Descending detection is strict; dedicated equal-value regression test |
| Comparator panics after partial mutation | All comparisons operate on indices before permutation application |
| An element is lost or duplicated | Indices begin as a complete permutation and are changed only by permutation-preserving operations |
| Instrumentation overflows | All report counters saturate |
| Platform or compiler regression | Three-OS stable matrix plus declared MSRV |
| Unreviewed unsafe behavior | `unsafe_code = "forbid"` and Clippy gate |

## Release checklist

1. Confirm the requirement and design baselines describe the release behavior.
2. Confirm the Assurance workflow passes on the release commit.
3. Review dependency inventory (`cargo tree`); version 0.1.0 expects none.
4. Review public API documentation and semver impact.
5. Update `CHANGELOG.md` and package version.
6. Create a signed or GitHub-attributed tag and release notes.
7. Retain the commit's GitHub Actions logs as the test record.
