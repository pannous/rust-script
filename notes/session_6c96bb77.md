# Claude Conversation (6c96bb77)

## 👤 Human

help: add a semicolon
  |
2 | put!("Hello, world!");
  |                      +

error: aborting due to 1 previous error

What happened to our modification of the language which allows optional semicolons to be dropped?

---

## 🤖 Assistant

Let me investigate what happened to the optional semicolon modification.

---

## 🤖 Assistant

→ `Task` (Explore)

---

## 🤖 Assistant

The semicolon inference feature exists and should work for newline-separated statements. The issue might be specific to the `put!` macro context. Let me check where this error is occurring.

---

## 🤖 Assistant

→ `Grep`: `put!`

---

## 🤖 Assistant

→ `Grep`: `macro_rules!\s*put`

---

## 🤖 Assistant

Let me check the script_harness.rs and understand the context where this error is happening.

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/compiler/rustc_builtin_macros/src/script_harness.rs`

---

## 🤖 Assistant

I see the issue. The semicolon inference was implemented for **statements** inside blocks, but not for **item-level** macro calls at the top level of a file.

When you write:
```rust
put!("Hello, world!")
```

At the top level (before the script harness wraps it in main), this is parsed as an **item**, not a statement. The semicolon inference only kicks in for statements within function bodies.

Let me check how item-level macro calls are parsed.

---

## 🤖 Assistant

→ `Grep`: `parse.*item.*mac|MacCall.*semi|expect_semi.*item`

---

## 🤖 Assistant

→ `Grep`: `parse_item_kind_mac`

---

## 🤖 Assistant

→ `Grep`: `MacCall|eat_semi|expect_semi|StmtStyle`

---

## 🤖 Assistant

Found it. Let me look at `eat_semi_for_macro_if_needed` more closely.

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/compiler/rustc_parse/src/parser/item.rs`

---

## 🤖 Assistant

The issue is in `eat_semi_for_macro_if_needed` at line 2438. It only checks for an actual semicolon via `self.eat(exp!(Semi))` but doesn't consider the newline-based inference.

Let me check how the statement parser handles this for comparison.

---

## 🤖 Assistant

→ `Grep`: `can_infer_semi_from_newline`

---

## 🤖 Assistant

The fix is straightforward. The `eat_semi_for_macro_if_needed` function needs to also check `can_infer_semi_from_newline()`. Let me apply the fix.

---

## 🤖 Assistant

→ `Edit`: `/opt/other/rust/compiler/rustc_parse/src/parser/item.rs`

---

## 🤖 Assistant

Now let me rebuild and test.

---

## 🤖 Assistant

→ `Bash`: `./rebuild.sh 2>&1 | tail -30`

---

## 🤖 Assistant

→ `Bash`: `unset CARGO_INCREMENTAL && ./rebuild.sh 2>&1 | tail -30`

---

## 🤖 Assistant

→ `Bash`: `CARGO_INCREMENTAL=0 ./x.py build --stage 1 compiler 2>&1 | tail -30`

---

## 🤖 Assistant

→ `Bash`: `RUSTC_WRAPPER="" CARGO_INCREMENTAL=0 ./x.py build --stage 1 compiler 2>&1 | tail -40`

---

## 🤖 Assistant

→ `Bash`: `cat /opt/other/rust/config.toml | head -50`

---

## 🤖 Assistant

→ `Bash`: `env -i HOME="$HOME" PATH="$PATH" ./x.py build --stage 1 compiler 2>&1 | tail -40`

---

## 🤖 Assistant

→ `Bash`: `killall sccache 2>/dev/null; export SCCACHE_IGNORE_SERVER_IO_ERROR=1; CARGO_INCREMENTAL=0 ./x.py bui...`

---

## 🤖 Assistant

→ `Bash`: `rm -rf /opt/other/rust/build/bootstrap/debug/incremental 2>/dev/null; rm -rf ~/.cargo/target/increme...`

---

## 🤖 Assistant

→ `Bash`: `cat /opt/other/rust/build/bootstrap/debug/rustc`

---

## 🤖 Assistant

→ `Bash`: `ls -la /opt/other/rust/*.toml 2>/dev/null; grep -r "sccache" /opt/other/rust/bootstrap.toml 2>/dev/n...`

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/bootstrap.toml`

---

## 🤖 Assistant

The issue is that `incremental = true` in bootstrap.toml conflicts with sccache. Let me temporarily disable it.

---

## 🤖 Assistant

→ `Edit`: `/opt/other/rust/bootstrap.toml`

---

## 🤖 Assistant

→ `Bash`: `./x.py build --stage 1 compiler 2>&1 | tail -50`

---

## 🤖 Assistant

Build succeeded. Now let me test the fix.

---

## 🤖 Assistant

→ `Bash`: `cat > /tmp/test_semi.rs << 'EOF'
#!/usr/bin/env rust
put!("Hello, world!")
EOF
./build/host/stage1/b...`

---

## 🤖 Assistant

The fix isn't working. Let me investigate. The issue might be how `can_infer_semi_from_newline()` works - it compares line indices of the previous token and current token. But at EOF, we might need special handling.

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/compiler/rustc_parse/src/parser/diagnostics.rs`

---

## 🤖 Assistant

Let me check how the check handles EOF and test with more lines.

---

## 🤖 Assistant

→ `Bash`: `cat > /tmp/test_semi2.rs << 'EOF'
#!/usr/bin/env rust
put!("Hello")
put!("World")
EOF
./build/host/s...`

---

## 🤖 Assistant

The first `put!("Hello")` parsed correctly (newline inferred before the second `put!`), but the last one at EOF failed. The issue is that EOF might not have a proper line index. Let me fix this.

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/compiler/rustc_parse/src/parser/item.rs`

---

## 🤖 Assistant

→ `Grep`: `Eof|token::Eof|TokenKind::Eof`

---

## 🤖 Assistant

Let me look at how stmt.rs handles EOF for semicolon inference.

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/compiler/rustc_parse/src/parser/stmt.rs`

---

## 🤖 Assistant

I see the pattern. At EOF, the statement parser doesn't require a semicolon at all (`if self.token != token::Eof`). Let me update the fix.

---

## 🤖 Assistant

→ `Edit`: `/opt/other/rust/compiler/rustc_parse/src/parser/item.rs`

---

## 🤖 Assistant

→ `Bash`: `./x.py build --stage 1 compiler 2>&1 | tail -20`

---

## 🤖 Assistant

→ `Bash`: `./build/host/stage1/bin/rustc -Z script /tmp/test_semi.rs -o /tmp/test_semi && /tmp/test_semi`

---

## 🤖 Assistant

The parsing error is gone. Now I just need to build the standard library too.

---

## 🤖 Assistant

→ `Bash`: `./x.py build --stage 1 library 2>&1 | tail -10`

---

## 🤖 Assistant

→ `Bash`: `./build/host/stage1/bin/rustc -Z script /tmp/test_semi.rs -o /tmp/test_semi && /tmp/test_semi`

---

## 🤖 Assistant

Excellent! Let me test more cases.

---

## 🤖 Assistant

→ `Bash`: `./build/host/stage1/bin/rustc -Z script /tmp/test_semi2.rs -o /tmp/test_semi2 && /tmp/test_semi2`

---

## 🤖 Assistant

Let me test that we haven't broken cases that should still require a semicolon.

---

## 🤖 Assistant

→ `Bash`: `cat > /tmp/test_semi_err.rs << 'EOF'
#!/usr/bin/env rust
put!("A") put!("B")
EOF
./build/host/stage1...`

---

## 🤖 Assistant

Same-line macros still require semicolons (correctly). Let me test brace-style macros.

---

## 🤖 Assistant

→ `Bash`: `cat > /tmp/test_brace.rs << 'EOF'
#!/usr/bin/env rust
put!{"Hello"}
put!{"World"}
EOF
./build/host/s...`

---

## 🤖 Assistant

Let me run a few probe tests to verify we haven't broken anything.

---

## 🤖 Assistant

→ `Bash`: `./build/host/stage1/bin/rustc -Z script probes/test_simple.rs -o /tmp/test_simple && /tmp/test_simpl...`

---

## 🤖 Assistant

→ `Bash`: `./build/host/stage1/bin/rustc -Z script probes/test_put.rs -o /tmp/test_put && /tmp/test_put`

---

## 🤖 Assistant

That test has unrelated syntax issues. Let me test a few more.

---

## 🤖 Assistant

→ `Bash`: `./build/host/stage1/bin/rustc -Z script probes/test_fib.rs -o /tmp/test_fib && /tmp/test_fib`

---

## 🤖 Assistant

→ `Bash`: `./build/host/stage1/bin/rustc -Z script probes/test_printf.rs -o /tmp/test_printf && /tmp/test_print...`

---

## 🤖 Assistant

→ `Bash`: `./build/host/stage1/bin/rustc -Z script probes/test_main.rs -o /tmp/test_main && /tmp/test_main`

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/probes/test_main.rs`

---

## 🤖 Assistant

→ `Edit`: `/opt/other/rust/probes/test_main.rs`

---

## 🤖 Assistant

→ `Bash`: `./build/host/stage1/bin/rustc -Z script probes/test_main.rs -o /tmp/test_main && /tmp/test_main`

---

## 🤖 Assistant

Let me restore the incremental setting and commit the fix.

---

## 🤖 Assistant

→ `Edit`: `/opt/other/rust/bootstrap.toml`

---

## 🤖 Assistant

→ `Bash`: `git diff compiler/rustc_parse/src/parser/item.rs probes/test_main.rs`

---

## 🤖 Assistant

→ `Bash`: `git add compiler/rustc_parse/src/parser/item.rs probes/test_main.rs && git commit -m "$(cat <<'EOF'
...`

---

## 🤖 Assistant

Fixed. The issue was that the semicolon inference from newlines was only implemented for **statements** within function bodies (in `stmt.rs`), but not for **item-level** macro calls at the top level of files.

**The fix** (`compiler/rustc_parse/src/parser/item.rs:2438-2446`):
- At EOF, don't require a semicolon (consistent with statement parsing)
- If the next token is on a different line than the macro call, infer the semicolon

Now scripts like this work:
```rust
#!/usr/bin/env rust
put!("Hello, world!")
```

---

