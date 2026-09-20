//! `DietCokeSort`: a crisp, stable, adaptive sorting algorithm with zero sugar
//! and zero tolerance for disorder.
//!
//! `DietCokeSort` discovers monotonic runs (Classic), crisps short runs with
//! stable binary insertion (Lime), merges an index permutation (Feisty Cherry),
//! and only then rearranges the input (Caffeine Free). Comparisons stay separate
//! from mutation, so a panicking comparator cannot shake up the input.
//!
//! # Example
//!
//! ```
//! use diet_coke_sort::{DietCoke, SortFlavor};
//!
//! let mut values = [5, 1, 4, 1, 3];
//! let report = DietCoke.sort(&mut values);
//!
//! assert_eq!(values, [1, 1, 3, 4, 5]);
//! assert_eq!(report.len(), 5);
//! ```

#![forbid(unsafe_code)]

use std::cmp::Ordering;

/// The target size used when crisping short natural runs: 32 elements, not
/// fluid ounces.
///
/// A fixed bound keeps insertion work linear in the input length while giving
/// merges enough locally ordered data to work efficiently.
pub const CRISP_RUN: usize = 32;

/// A zero-cost interface implemented by every sorting flavor.
///
/// The associated report lets instrumented Diet Coke flavors return
/// [`SortReport`] while [`CocaColaSort`] returns `()` after delegating to Rust.
/// That is static dispatch: no trait object, runtime tag, or mystery syrup.
pub trait SortFlavor<T> {
    /// The evidence returned after this flavor finishes sorting.
    type Report;

    /// Sorts `values` according to this flavor's documented total order.
    fn sort(&self, values: &mut [T]) -> Self::Report;
}

/// The original `DietCokeSort` flavor for every type implementing [`Ord`].
///
/// This is the standard pour for integers, strings, and arbitrary-precision
/// numeric types. It performs no numeric conversion or arithmetic, so every
/// digit stays in its own lane.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DietCoke;

/// The lime-forward `f64` flavor.
///
/// It uses [`f64::total_cmp`] to provide the IEEE 754 `totalOrder` sequence,
/// including infinities, subnormals, signed zero, and every NaN bit pattern.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DietCokeWithLime;

/// The caffeine-free lime `f32` flavor.
///
/// It uses [`f32::total_cmp`] for the IEEE 754 `totalOrder` sequence. Fewer
/// bits, fewer jitters, same deterministic treatment of special values.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct DietCokeWithLimeCaffeineFree;

/// The full-sugar fallback that calls Rust's standard stable slice sort.
///
/// There is no [`SortReport`]; this flavor returns `()`. It is dependable and
/// highly tuned, just not wearing the artisanal silver can.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CocaColaSort;

/// The nutrition label captured during one `DietCokeSort` execution.
///
/// Counts saturate at [`usize::MAX`] instead of overflowing.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct SortReport {
    len: usize,
    initial_runs: usize,
    merges: usize,
    comparisons: usize,
    swaps: usize,
}

impl SortReport {
    /// Returns the serving size, measured in elements rather than cans.
    #[must_use]
    pub const fn len(self) -> usize {
        self.len
    }

    /// Returns `true` when somebody forgot to restock the fridge.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.len == 0
    }

    /// Returns the number of natural or Lime-crisped runs found initially.
    #[must_use]
    pub const fn initial_runs(self) -> usize {
        self.initial_runs
    }

    /// Returns the number of pairwise merges—the algorithm's flavor blending.
    #[must_use]
    pub const fn merges(self) -> usize {
        self.merges
    }

    /// Returns the comparator-call count. Every sip is accounted for.
    #[must_use]
    pub const fn comparisons(self) -> usize {
        self.comparisons
    }

    /// Returns the swaps used for the clean finish. No sticky residue included.
    #[must_use]
    pub const fn swaps(self) -> usize {
        self.swaps
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Run {
    start: usize,
    end: usize,
}

/// Stably sorts a slice in ascending order—the classic silver-can pour.
///
/// This sort is deterministic and adaptive. An already sorted input takes
/// linear time. In the worst case it performs `O(n log n)` comparisons, uses
/// `O(n)` auxiliary index storage, and applies at most `n - 1` element swaps.
///
/// # Examples
///
/// ```
/// let mut callsigns = ["Gemini", "Apollo", "Mercury"];
/// diet_coke_sort::sort(&mut callsigns);
/// assert_eq!(callsigns, ["Apollo", "Gemini", "Mercury"]);
/// ```
pub fn sort<T: Ord>(values: &mut [T]) -> SortReport {
    sort_by(values, Ord::cmp)
}

impl<T: Ord> SortFlavor<T> for DietCoke {
    type Report = SortReport;

    fn sort(&self, values: &mut [T]) -> Self::Report {
        sort(values)
    }
}

impl SortFlavor<f64> for DietCokeWithLime {
    type Report = SortReport;

    fn sort(&self, values: &mut [f64]) -> Self::Report {
        sort_by(values, f64::total_cmp)
    }
}

impl SortFlavor<f32> for DietCokeWithLimeCaffeineFree {
    type Report = SortReport;

    fn sort(&self, values: &mut [f32]) -> Self::Report {
        sort_by(values, f32::total_cmp)
    }
}

impl<T: Ord> SortFlavor<T> for CocaColaSort {
    type Report = ();

    fn sort(&self, values: &mut [T]) -> Self::Report {
        values.sort();
    }
}

/// Stably sorts with a custom comparator: the Freestyle machine of this API.
///
/// The comparator must define a total order. Equal elements retain their
/// original relative order. All comparisons finish before `values` is changed;
/// consequently, a comparator panic leaves the input byte-for-byte logically
/// unchanged (subject to any interior mutation performed by the comparator).
///
/// # Examples
///
/// ```
/// let mut values = [1, 4, 2, 3];
/// diet_coke_sort::sort_by(&mut values, |left, right| right.cmp(left));
/// assert_eq!(values, [4, 3, 2, 1]);
/// ```
pub fn sort_by<T, F>(values: &mut [T], mut compare: F) -> SortReport
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = values.len();
    if len < 2 {
        return SortReport {
            len,
            initial_runs: usize::from(len == 1),
            ..SortReport::default()
        };
    }

    let mut comparisons = 0;
    let mut order: Vec<usize> = (0..len).collect();
    let mut runs = discover_runs(values, &mut order, &mut compare, &mut comparisons);
    let initial_runs = runs.len();
    let mut scratch = vec![0; len];
    let mut merges = 0_usize;

    while runs.len() > 1 {
        let mut next_runs = Vec::with_capacity(runs.len().div_ceil(2));
        let mut pairs = runs.chunks_exact(2);

        for pair in &mut pairs {
            let left = pair[0];
            let right = pair[1];
            debug_assert_eq!(left.end, right.start);

            merge_indices(
                values,
                &order,
                &mut scratch,
                left,
                right.end,
                &mut compare,
                &mut comparisons,
            );
            order[left.start..right.end].copy_from_slice(&scratch[left.start..right.end]);
            next_runs.push(Run {
                start: left.start,
                end: right.end,
            });
            merges = merges.saturating_add(1);
        }

        if let [remaining] = pairs.remainder() {
            next_runs.push(*remaining);
        }
        runs = next_runs;
    }

    let swaps = apply_permutation(values, &order, &mut scratch);

    SortReport {
        len,
        initial_runs,
        merges,
        comparisons,
        swaps,
    }
}

fn discover_runs<T, F>(
    values: &[T],
    order: &mut [usize],
    compare: &mut F,
    comparisons: &mut usize,
) -> Vec<Run>
where
    F: FnMut(&T, &T) -> Ordering,
{
    let len = order.len();
    let mut runs = Vec::with_capacity(len.div_ceil(CRISP_RUN));
    let mut start = 0;

    while start < len {
        let mut end = start + 1;

        if end < len {
            let descending =
                counted_compare(values, order[start], order[end], compare, comparisons)
                    == Ordering::Greater;
            end += 1;

            if descending {
                while end < len
                    && counted_compare(values, order[end - 1], order[end], compare, comparisons)
                        == Ordering::Greater
                {
                    end += 1;
                }
                order[start..end].reverse();
            } else {
                while end < len
                    && counted_compare(values, order[end - 1], order[end], compare, comparisons)
                        != Ordering::Greater
                {
                    end += 1;
                }
            }
        }

        let crisp_end = start.saturating_add(CRISP_RUN).min(len);
        if end < crisp_end {
            stable_binary_extend(values, order, start, end, crisp_end, compare, comparisons);
            end = crisp_end;
        }

        runs.push(Run { start, end });
        start = end;
    }

    runs
}

fn stable_binary_extend<T, F>(
    values: &[T],
    order: &mut [usize],
    start: usize,
    sorted_end: usize,
    target_end: usize,
    compare: &mut F,
    comparisons: &mut usize,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    for index in sorted_end..target_end {
        let key = order[index];
        let mut low = start;
        let mut high = index;

        // Upper-bound insertion places the new item after existing equals,
        // preserving the source order of equivalent elements.
        while low < high {
            let middle = low + (high - low) / 2;
            if counted_compare(values, order[middle], key, compare, comparisons)
                == Ordering::Greater
            {
                high = middle;
            } else {
                low = middle + 1;
            }
        }

        if low != index {
            order.copy_within(low..index, low + 1);
            order[low] = key;
        }
    }
}

fn merge_indices<T, F>(
    values: &[T],
    order: &[usize],
    scratch: &mut [usize],
    left: Run,
    end: usize,
    compare: &mut F,
    comparisons: &mut usize,
) where
    F: FnMut(&T, &T) -> Ordering,
{
    let mut left_cursor = left.start;
    let mut right_cursor = left.end;
    let mut output = left.start;

    while left_cursor < left.end && right_cursor < end {
        let take_left = counted_compare(
            values,
            order[left_cursor],
            order[right_cursor],
            compare,
            comparisons,
        ) != Ordering::Greater;

        if take_left {
            scratch[output] = order[left_cursor];
            left_cursor += 1;
        } else {
            scratch[output] = order[right_cursor];
            right_cursor += 1;
        }
        output += 1;
    }

    if left_cursor < left.end {
        let remaining = left.end - left_cursor;
        scratch[output..output + remaining].copy_from_slice(&order[left_cursor..left.end]);
        output += remaining;
    }
    if right_cursor < end {
        scratch[output..end].copy_from_slice(&order[right_cursor..end]);
    }
}

fn counted_compare<T, F>(
    values: &[T],
    left: usize,
    right: usize,
    compare: &mut F,
    comparisons: &mut usize,
) -> Ordering
where
    F: FnMut(&T, &T) -> Ordering,
{
    *comparisons = comparisons.saturating_add(1);
    compare(&values[left], &values[right])
}

fn apply_permutation<T>(values: &mut [T], order: &[usize], targets: &mut [usize]) -> usize {
    for (destination, &source) in order.iter().enumerate() {
        targets[source] = destination;
    }

    let mut swaps = 0_usize;
    for position in 0..values.len() {
        while targets[position] != position {
            let destination = targets[position];
            values.swap(position, destination);
            targets.swap(position, destination);
            swaps = swaps.saturating_add(1);
        }
    }
    swaps
}

#[cfg(test)]
mod tests {
    use super::{
        sort, sort_by, CocaColaSort, DietCoke, DietCokeWithLime,
        DietCokeWithLimeCaffeineFree, SortFlavor, CRISP_RUN,
    };
    use std::cell::Cell;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct Tagged {
        key: i32,
        source_position: usize,
    }

    #[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
    struct WideUnsigned([u64; 8]);

    #[test]
    fn flavor_interface_routes_the_classic_and_full_sugar_pours() {
        let mut diet_values = [i128::MAX, 0, i128::MIN, -1, 1];
        let diet_report = DietCoke.sort(&mut diet_values);
        assert_eq!(diet_values, [i128::MIN, -1, 0, 1, i128::MAX]);
        assert_eq!(diet_report.len(), diet_values.len());

        let mut cola_values = [5_i32, 1, 4, 2, 3];
        let _: () = CocaColaSort.sort(&mut cola_values);
        assert_eq!(cola_values, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn diet_coke_with_lime_totally_orders_every_f64_class_without_rounding() {
        let mut values = [
            f64::NAN,
            f64::from_bits(1),
            -0.0,
            f64::MAX,
            f64::NEG_INFINITY,
            f64::from_bits(0xfff8_0000_0000_0001),
            1.0,
            f64::from_bits(1.0_f64.to_bits() + 1),
            f64::INFINITY,
            f64::MIN_POSITIVE,
            -f64::MAX,
            0.0,
            -f64::MIN_POSITIVE,
            f64::from_bits(0x7ff0_0000_0000_0001),
            f64::from_bits(0x8000_0000_0000_0001),
        ];
        let mut expected = values;
        expected.sort_unstable_by(f64::total_cmp);

        let report = DietCokeWithLime.sort(&mut values);

        let actual_bits: Vec<_> = values.into_iter().map(f64::to_bits).collect();
        let expected_bits: Vec<_> = expected.into_iter().map(f64::to_bits).collect();
        assert_eq!(actual_bits, expected_bits);
        assert_eq!(report.len(), actual_bits.len());
    }

    #[test]
    fn caffeine_free_lime_totally_orders_f32_without_rounding() {
        let mut values = [
            f32::NAN,
            f32::from_bits(1),
            -0.0,
            f32::MAX,
            f32::NEG_INFINITY,
            f32::from_bits(0xffc0_0001),
            1.0,
            f32::from_bits(1.0_f32.to_bits() + 1),
            f32::INFINITY,
            f32::MIN_POSITIVE,
            -f32::MAX,
            0.0,
            -f32::MIN_POSITIVE,
            f32::from_bits(0x7f80_0001),
            f32::from_bits(0x8000_0001),
        ];
        let mut expected = values;
        expected.sort_unstable_by(f32::total_cmp);

        let report = DietCokeWithLimeCaffeineFree.sort(&mut values);

        let actual_bits: Vec<_> = values.into_iter().map(f32::to_bits).collect();
        let expected_bits: Vec<_> = expected.into_iter().map(f32::to_bits).collect();
        assert_eq!(actual_bits, expected_bits);
        assert_eq!(report.len(), actual_bits.len());
    }

    #[test]
    fn classic_flavor_preserves_arbitrary_width_integer_precision() {
        let mut values = [
            WideUnsigned([u64::MAX; 8]),
            WideUnsigned([0; 8]),
            WideUnsigned([0, 0, 0, 0, 0, 0, 0, 1]),
            WideUnsigned([0, 0, 0, 1, 0, 0, 0, 0]),
        ];

        DietCoke.sort(&mut values);

        assert_eq!(
            values,
            [
                WideUnsigned([0; 8]),
                WideUnsigned([0, 0, 0, 0, 0, 0, 0, 1]),
                WideUnsigned([0, 0, 0, 1, 0, 0, 0, 0]),
                WideUnsigned([u64::MAX; 8]),
            ]
        );
    }

    #[test]
    fn req_f_001_sorts_nominal_input() {
        let mut values = [8, 1, 6, 3, 5, 7, 4, 2];
        let report = sort(&mut values);

        assert_eq!(values, [1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(report.len(), values.len());
        assert!(report.comparisons() > 0);
    }

    #[test]
    fn req_f_002_preserves_equal_element_order() {
        let mut values = [
            Tagged {
                key: 2,
                source_position: 0,
            },
            Tagged {
                key: 1,
                source_position: 1,
            },
            Tagged {
                key: 2,
                source_position: 2,
            },
            Tagged {
                key: 1,
                source_position: 3,
            },
        ];

        sort_by(&mut values, |left, right| left.key.cmp(&right.key));

        let positions: Vec<_> = values.iter().map(|value| value.source_position).collect();
        assert_eq!(positions, [1, 3, 0, 2]);
    }

    #[test]
    fn req_f_004_handles_empty_and_singleton_inputs() {
        let mut empty: [i32; 0] = [];
        let empty_report = sort(&mut empty);
        assert!(empty_report.is_empty());
        assert_eq!(empty_report.initial_runs(), 0);

        let mut singleton = [42];
        let singleton_report = sort(&mut singleton);
        assert_eq!(singleton, [42]);
        assert_eq!(singleton_report.initial_runs(), 1);
        assert_eq!(singleton_report.comparisons(), 0);
    }

    #[test]
    fn req_f_005_supports_custom_ordering() {
        let mut values = [1, 4, 2, 5, 3];
        sort_by(&mut values, |left, right| right.cmp(left));
        assert_eq!(values, [5, 4, 3, 2, 1]);
    }

    #[test]
    fn req_s_002_comparator_panic_leaves_input_unchanged() {
        let original = [9, 4, 7, 1, 3, 8, 2, 6, 5];
        let mut values = original;
        let calls = Cell::new(0_usize);

        let result = catch_unwind(AssertUnwindSafe(|| {
            sort_by(&mut values, |left, right| {
                calls.set(calls.get() + 1);
                assert!(calls.get() < 4, "injected comparator failure");
                left.cmp(right)
            });
        }));

        assert!(result.is_err());
        assert_eq!(values, original);
    }

    #[test]
    fn req_q_001_report_is_deterministic() {
        let input: Vec<_> = (0..97).map(|value| (value * 37) % 23).collect();
        let mut first = input.clone();
        let mut second = input;

        let first_report = sort(&mut first);
        let second_report = sort(&mut second);

        assert_eq!(first, second);
        assert_eq!(first_report, second_report);
    }

    #[test]
    fn req_p_001_recognizes_an_already_sorted_stream() {
        let mut values: Vec<_> = (0..(CRISP_RUN * 4)).collect();
        let report = sort(&mut values);

        assert_eq!(report.initial_runs(), 1);
        assert_eq!(report.merges(), 0);
        assert_eq!(report.comparisons(), values.len() - 1);
        assert_eq!(report.swaps(), 0);
    }

    #[test]
    fn strict_descending_run_does_not_reverse_equal_values() {
        let mut values = [
            Tagged {
                key: 3,
                source_position: 0,
            },
            Tagged {
                key: 2,
                source_position: 1,
            },
            Tagged {
                key: 2,
                source_position: 2,
            },
            Tagged {
                key: 1,
                source_position: 3,
            },
        ];

        sort_by(&mut values, |left, right| left.key.cmp(&right.key));

        let positions: Vec<_> = values.iter().map(|value| value.source_position).collect();
        assert_eq!(positions, [3, 1, 2, 0]);
    }

    #[test]
    fn deterministic_stress_matches_standard_stable_sort() {
        let mut state = 0xD1E7_C0DE_u64;

        for len in 0..300 {
            let mut actual = Vec::with_capacity(len);
            for _ in 0..len {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                actual.push(i32::try_from((state >> 32) % 31).expect("generated value is small"));
            }
            let mut expected = actual.clone();
            expected.sort_unstable();

            let report = sort(&mut actual);
            assert_eq!(
                actual, expected,
                "failed deterministic case of length {len}"
            );
            assert!(report.swaps() <= len.saturating_sub(1));
        }
    }

    #[test]
    fn comparison_equality_is_stable_across_many_runs() {
        let mut values: Vec<_> = (0..257)
            .rev()
            .map(|source_position| Tagged {
                key: i32::try_from(source_position % 7).expect("small test key"),
                source_position,
            })
            .collect();
        let mut expected = values.clone();
        expected.sort_by_key(|value| value.key);

        sort_by(&mut values, |left, right| left.key.cmp(&right.key));

        assert_eq!(values, expected);
    }
}
