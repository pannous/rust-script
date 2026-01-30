# Claude Conversation (5d8b5366)

## 👤 Human

/opt/other/rust/ ll /opt/other/rust/build/host/stage1/bin/rustc

ls: /opt/other/rust/build/host/stage1/bin/rustc: No such file or directory

How come the binary gets deleted during rebuild.sh ? Can't it be replaced last second?

---

## 🤖 Assistant

Let me investigate the rebuild script and the build process to understand what's happening.

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/rebuild.sh`

---

## 🤖 Assistant

→ `Bash`: `cat /opt/other/rust/x.py | head -100`

---

## 🤖 Assistant

→ `Grep`: `stage1.*bin.*rustc|rm.*rustc|remove.*rustc`

---

## 🤖 Assistant

→ `Grep`: `stamp|invalidat|clean|delete.*stage|remove.*stage`

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/src/bootstrap/src/utils/build_stamp.rs`

---

## 🤖 Assistant

→ `Grep`: `clear_if_dirty|remove_dir_all.*stage`

---

## 🤖 Assistant

→ `Grep`: `clear_if_dirty`

---

## 🤖 Assistant

Let me look at the cargo.rs file where `clear_if_dirty` is called with the rustc binary:

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/src/bootstrap/src/core/builder/cargo.rs`

---

## 🤖 Assistant

→ `Grep`: `sysroot.*stage|copy.*rustc|link.*rustc|uplift`

---

## 🤖 Assistant

Now I understand the mechanism. Let me check how the sysroot copying works and where the binary disappears:

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/src/bootstrap/src/core/build_steps/compile.rs`

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/src/bootstrap/src/core/build_steps/compile.rs`

---

## 🤖 Assistant

→ `Grep`: `remove_dir_all.*sysroot|delete.*sysroot|clean.*sysroot`

---

## 🤖 Assistant

Found it. Let me look at the key section at line 1963:

---

## 🤖 Assistant

→ `Read`: `/opt/other/rust/src/bootstrap/src/core/build_steps/compile.rs`

---

## 🤖 Assistant

Now I understand the issue completely. Here's what's happening:

**The sysroot is deleted at line 1963** in `compile.rs` with:
```rust
let _ = fs::remove_dir_all(&sysroot);
```

This happens in the `Sysroot::run()` function, which removes the **entire** sysroot directory (including `build/host/stage1/bin/rustc`) to "avoid caching bugs" as the comment says. This happens **every time** the sysroot step runs, which is early in the build process.

The binary is then rebuilt/copied at the **end** of the `Assemble::run()` function (lines 2563-2570) where it copies `rustc-main` to the sysroot bin directory.

**Why can't it be replaced at the last second?**

The problem is the build order:
1. `Sysroot::run()` executes first, deletes the entire sysroot dir
2. Various compilation steps happen
3. `Assemble::run()` copies the rustc binary back

This is intentional - the Rust bootstrap system deletes the sysroot to prevent stale artifacts from causing subtle bugs. The comment at line 1960-1962 explains this.

**Workarounds:**

1. **Use `./x.py check`** for faster iteration when you don't need the binary
2. **Keep a symlink/wrapper** pointing to a stable location like `./rustc` that points to wherever the current build is
3. **The binary exists during the build** in `build/host/stage1/rustc/release/rustc-main` - you could reference that directly (though it's not fully assembled with libs)

Would you like me to create a small wrapper script that always finds the current rustc binary regardless of build state?

---

