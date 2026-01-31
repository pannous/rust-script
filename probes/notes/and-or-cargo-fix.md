# and/or Operator Fix for Cargo Test

## Problem
The `and` and `or` operators worked when running tests directly with rustc:
```bash
./tests/test_and_or.rust  # ✓ Works
```

But failed with cargo test:
```bash
cargo test --test test_and_or  # ✗ Error: no method named `is_truthy` found
```

## Root Cause
In `compiler/rustc_builtin_macros/src/script_harness.rs`, there was a check that skipped
injecting extension code (including the `Truthy` trait) when compiling in test mode:

```rust
let parsed_extensions = if !sess.opts.test {
    transformer::parse_extensions(&sess.psess, call_site)
} else {
    ThinVec::new()  // Skip extensions in test mode
};
```

The comment claimed "tests use cargo which provides dependencies properly", but this
was incorrect for our custom syntax extensions which need to be injected at compile time.

## Solution
Always inject extensions, even in test mode:

```rust
let parsed_extensions = transformer::parse_extensions(&sess.psess, call_site);
```

## Additional Fixes Required
- Added `AllowConstBlockItems` parameter to `parse_item()` calls
- Added `in_block: false` to `FnParseMode` initialization 
- Handled new `ConstBlock` item kind in pattern matching
- Added `Dummy` and `DynIncompatibleTrait` to attribute checking patterns

## Result
Both direct rustc and cargo test now work correctly with and/or operators.
