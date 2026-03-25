Issue: rust-lang/rust#154247

Summary:
- `unused_braces` was warning on 2024-edition argument blocks whose braces intentionally shorten the lifetime of temporaries such as `MutexGuard`.

Steps taken:
1. Read the GitHub issue and confirmed the reported behavior: a block-wrapped method argument in Edition 2024 can be semantically meaningful because block tail-expression temporaries now drop at the end of that block.
2. Reproduced the false positive locally with a new UI test using `Mutex::lock()` in both function-argument and method-argument positions.
3. Inspected `compiler/rustc_lint/src/unused.rs` and found that `unused_braces` was implemented as an AST early lint with no type information, so it could not see drop-sensitive temporaries.
4. Chose a split fix:
   - keep the existing early lint for most contexts;
   - skip 2024 function/method argument blocks in the early pass;
   - re-check those cases in a new late lint that has type information.
5. Implemented the late check so it still warns for truly unnecessary braces in 2024, but suppresses the warning when the wrapped expression contains a non-place value with significant drop that is used in a borrow-/projection-sensitive way.
6. Added UI coverage for both sides:
   - `unused_braces-issue-154247.rs` ensures the false positive is gone;
   - `unused_braces-edition-2024.rs` ensures ordinary unnecessary braces in 2024 still warn.
7. Rebuilt the compiler, fixed compile-time issues in the new late-lint path, ran formatting checks, and validated the tests.

Validation:
- `just b`
- `x test tests/ui/lint/unused_braces.rs tests/ui/lint/unused_braces_borrow.rs tests/ui/lint/unused_braces-edition-2024.rs tests/ui/lint/unused_braces-issue-154247.rs --bless --keep-stage-std=1`
- `x fmt --check`
- `just rui`

Final solution:
- The fix avoids emitting `unused_braces` too early for 2024 argument blocks, then uses a late-lint pass to recover the warning only when the braces are actually redundant. This keeps the lint useful in Edition 2024 without warning on blocks that are preserving temporary drop order.

Confidence: 8/10
