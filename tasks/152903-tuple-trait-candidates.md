# Issue 152903: tuple trait candidate diagnostics

## Summary

- Goal: improve the `E0277` help output when rustc lists many tuple trait impl candidates, so it does not dump a long sequence of tuple arities.
- Confidence: 9/10.

## Reproduction

1. Read the issue and confirmed it is a diagnostics enhancement, not a trait solver bug.
2. Added `tests/ui/traits/tuple-diagnostic-issue-152903.rs` with a minimal reproducer:
   `testing((1, Foo));` where `testing<T: Debug>(...)`.
3. Reproduced the current behavior with `just cur` and `just dev`, which showed:
   `()`, then several tuple impl shapes like `(T,)`, `(U, T)`, and `and 5 others`.

## Analysis

1. Traced the candidate rendering to
   `compiler/rustc_trait_selection/src/error_reporting/traits/fulfillment_errors.rs`
   in `report_similar_impl_candidates`.
2. Verified the problem is in diagnostics formatting for impl candidates, not in candidate discovery.
3. Observed that for tuple obligations the relevant candidates are all tuple impls, and the current fallback prints them one by one after lexicographic sorting.

## Fix

1. Kept the existing behavior for small candidate lists and non-tuple candidates.
2. Added a focused summarization path for large candidate lists when:
   - all candidate self types are tuples,
   - all candidates are for the same trait,
   - tuple arities form a contiguous range,
   - and there are enough tuple arities that listing them individually is noisy.
3. The new help output compresses these cases to:
   - `()`
   - `tuples with 1 to 12 elements`

## Validation

1. Ran `just b`.
2. Ran `x test tests/ui/traits/tuple-diagnostic-issue-152903.rs --bless`.
3. Ran `x test tests/ui/on-unimplemented/suggest_tuple_wrap.rs tests/ui/traits/tuple-diagnostic-issue-152903.rs`.
4. Ran `x fmt --check`.
5. Ran `just rui`.

## Notes

- `just ts ... --keep-stage-std=1` initially hit an old sysroot problem (`can't find crate for std`), so I reran the focused test with plain `x test ... --bless` to rebuild the stage1 std sysroot before continuing.
