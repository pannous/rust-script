# Claude Conversation (259ef3d3)

## 👤 Human

Let's check the feasibility of adding any ergonomic list syntax for Rust. [1,2,3] (1,2,3) {1,2,3} Are probably all highly problematic because of existing syntax patterns, right?

---

## 🤖 Assistant

You're correct - all three are already taken in Rust:

**`[1, 2, 3]`** - Array literal syntax. Creates a fixed-size `[T; N]` array.

**`(1, 2, 3)`** - Tuple syntax. Creates a `(i32, i32, i32)` tuple.

**`{1, 2, 3}`** - Block expression. Parses as a block containing three expression statements, evaluating to `3` (the last expression). Also conflicts with struct initialization `Foo { x, y, z }`.

**Potential alternatives that might be less problematic:**

| Syntax | Status |
|--------|--------|
| `#[1, 2, 3]` | `#` is used for attributes, but `#[...]` expects an identifier |
| `@[1, 2, 3]` | `@` is reserved but unused |
| `~[1, 2, 3]` | `~` was used in old Rust for `Box`, now unused |
| `vec![1, 2, 3]` | Already exists (macro) |
| `list![1, 2, 3]` | Macro approach - no syntax change needed |

The macro approach (`vec![]`, `list![]`) is how Rust handles this today without grammar changes.

Would you like me to explore what it would take to add a new sigil-prefixed syntax like `@[...]` or `~[...]` in your fork? Or are you thinking about something else entirely (like making `vec![]` less verbose in script mode)?

---

## 👤 Human

@ is reserved for what?

---

## 🤖 Assistant

I misspoke - `@` is actually **actively used**, not just reserved. It's the pattern binding operator:

```rust
match value {
    n @ 1..=5 => println!("matched {} in range", n),
    Some(x @ Point { y: 0, .. }) => println!("point on x-axis: {:?}", x),
    _ => {}
}
```

It binds a name to a value while simultaneously matching a pattern.

**However** - `@` only has meaning in pattern position, not expression position. So `@[1, 2, 3]` as an *expression* might still be unambiguous and parseable, since there's no current expression starting with `@`.

The truly unused/reserved sigils in Rust are:
- `~` (was `Box` in pre-1.0 Rust)
- `$` outside macros (used for macro metavariables inside macro definitions)

Want me to investigate whether `@[...]` would actually parse cleanly in expression contexts, or explore a different direction?

---

## 👤 Human

yes

---

## 🤖 Assistant

→ `Task` (Explore)

---

## 🤖 Assistant

**Good news: `@[...]` is feasible with no conflicts.**

## Summary

| Aspect | Finding |
|--------|---------|
| `@` in expressions | **Not used** - only exists in pattern binding (`x @ 1..=5`) |
| `can_begin_expr()` | `@` is **not** in the list of tokens that can start expressions |
| Infix operator | `@` is **not** handled by `AssocOp::from_token()` |
| Parse paths | Pattern parsing and expression parsing are **completely separate** |

## Implementation would require:

1. **`compiler/rustc_parse/src/parser/expr.rs`** ~line 537-594 in `parse_expr_prefix()`:
   ```rust
   token::At => {
       // Parse @[...] expression form
       make_it!(this, attrs, |this, _| this.parse_expr_at_bracket(lo))
   }
   ```

2. **`compiler/rustc_ast/src/token.rs`** ~line 670 - add `At` to `can_begin_expr()`

3. **New `ExprKind` variant** for the AST representation

4. **Decide semantics** - what does `@[1, 2, 3]` desugar to? `Vec::from([1, 2, 3])`? A new collection type?

Would you like me to create a plan for implementing this, or first discuss what `@[...]` should actually *mean* semantically?

---

## 👤 Human

make it desugar to vec![] Which is the same as Vec::From, right?

---

## 🤖 Assistant

Close but not identical:

- **`vec![1, 2, 3]`** expands to something like `<[_]>::into_vec(Box::new([1, 2, 3]))` or a push loop
- **`Vec::from([1, 2, 3])`** uses the `From<[T; N]>` impl

Semantically equivalent, but `vec![]` is the idiomatic choice. Desugaring `@[1, 2, 3]` → `vec![1, 2, 3]` makes sense.

Let me create an implementation plan.

---

## 🤖 Assistant

→ `Task` (Plan)

---

## 🤖 Assistant

Now I have the details. Let me write the plan.

---

## 🤖 Assistant

→ `Write`: `/Users/me/.claude/plans/twinkling-skipping-pumpkin.md`

---

## 🤖 Assistant

Let me verify the exact locations by reading the key sections.

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/compiler/rustc_ast/src/token.rs`

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/compiler/rustc_parse/src/parser/expr.rs`

---

## 🤖 Assistant

Let me also check how tokens are matched and how `parse_delim_args` works.

---

## 🤖 Assistant

→ `Grep`: `parse_delim_args`

---

## 🤖 Assistant

→ `Grep`: `token::At`

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/compiler/rustc_parse/src/parser/expr.rs`

---

## 🤖 Assistant

Let me check how look-ahead works and how `exp!` is used for token matching.

---

## 🤖 Assistant

→ `Grep`: `look_ahead.*token::`

---

## 🤖 Assistant

→ `Grep`: `self\.token == token::`

---

## 🤖 Assistant

→ `Grep`: `exp!\(At\)`

---

## 🤖 Assistant

Now I have all the details. Let me update the plan with exact code.

---

## 🤖 Assistant

→ `Edit`: `/Users/me/.claude/plans/twinkling-skipping-pumpkin.md`

---

## 🤖 Assistant

→ `Read`: `/Users/me/.claude/plans/twinkling-skipping-pumpkin.md`

---

## 🤖 Assistant

→ `Write`: `/Users/me/.claude/plans/twinkling-skipping-pumpkin.md`

---

## 🤖 Assistant

The plan is ready. This is a small, focused change - about 15 lines of new code across 2 files:

1. **token.rs** - Add `At` to tokens that can start expressions
2. **expr.rs** - Add parsing logic to desugar `@[...]` to `vec![...]`

---

## 🤖 Assistant

→ `ExitPlanMode`

---

