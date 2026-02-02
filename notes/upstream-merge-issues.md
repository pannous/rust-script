# Upstream Merge Issues - 2026-02-02

## Test Failures After Merge

After merging 113 commits from upstream rust-lang/rust, 4 tests are failing:

1. test_for_key_in (compile)
2. test_map_literal (compile)
3. test_map_struct (compile)
4. test_put_no_import (compile)

### Root Cause

All failures are due to map literal syntax `{key: value}` not working.

**Error message:**
```
error: struct literal body without path
  --> probes/tests/test_put_no_import.rust:10:13
   |
10 |     let x = {a: 1, b: 2};
   |             ^^^^^^^^^^^^ struct name missing for struct literal
```

### Analysis

The map literal conversion code exists in:
- `compiler/rustc_parse/src/parser/diagnostics.rs:985` - `maybe_suggest_struct_literal()`
- This function should convert `{a: 1, b: 2}` to `HashMap::from([("a", Val::from(1)), ...])`

However, debug output shows that when parsing `let x = {a: 1, b: 2};`, the error occurs in expression parsing context, not in block parsing context where `maybe_suggest_struct_literal` would handle it properly.

The parse flow:
1. Parser sees `let x = ` and needs an expression
2. Sees `{a: 1, b: 2}`
3. Tries to parse as struct expression (fails - no path)
4. Error is raised BEFORE block parsing code with map literal support is reached

### Solution Needed

Map literal support needs to be added in the expression parsing code path, not just in block statement parsing. The error happens when parsing struct expressions with empty paths in expression position.

Files to investigate:
- `compiler/rustc_parse/src/parser/expr.rs` - where struct expressions are parsed
- May need to add map literal conversion in `parse_expr_struct` or earlier in expression parsing

### Debug Output Added

Added eprintln! debug statements in:
- `compiler/rustc_parse/src/parser/diagnostics.rs:1007` - struct_expr/block_tail results
- `compiler/rustc_parse/src/parser/stmt.rs:766` - parse_block_common entry
- These can be removed once issue is fixed

### Next Steps

1. Find where struct expressions with empty paths are being parsed in expression context
2. Add map literal conversion logic there (similar to `maybe_suggest_struct_literal`)
3. Or redirect to block parsing when seeing `{ident: expr}` pattern in expression context
4. Remove debug eprintln! statements
5. Rebuild and test
6. Commit fix

## Test Results
- **109 tests passed**
- **4 tests failed** (all map literal related)
- No regressions in other features
