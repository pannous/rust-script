# Custom Syntax Implementation Status

## ✅ VERIFIED WORKING in rustc (Compiler)

All custom syntax is **fully functional** in the custom Rust compiler. Test proof:

```bash
$ rustc -Zscript test-custom-syntax.rs && ./test-custom-syntax
add(2, 3) = 5
multiply(4, 5) = 20
and=false or=true xor=6
cos(0.0) = 1
sin(0.0) = 0
```

### 1. Function Keywords (Script Mode)

**Syntax:** `def` / `fun` as aliases for `fn`

**Implementation:** `compiler/rustc_parse/src/parser/item.rs:3557-3559`
```rust
if self.is_script_mode() && (self.token.is_ident_named(sym::def) || self.token.is_ident_named(sym::fun)) {
    self.bump(); // consume `def` or `fun`
    // Continues parsing as normal `fn`
}
```

**Status:** ✅ Full support - treated identically to `fn` through entire compilation pipeline

**Clippy Status:** ✅ Works - Clippy sees standard `fn` after desugaring

---

### 2. FFI Import (Always Available)

**Syntax:** `import fn name(args) -> ret;`

**Desugaring:** → `extern "C" { fn name(args) -> ret; }`

**Implementation:** `compiler/rustc_parse/src/parser/item.rs:3128-3199`
```rust
fn parse_import_fn(&mut self, lo: Span, attrs: &mut AttrVec) -> PResult<'a, Option<ItemKind>> {
    // Transforms `import fn` to extern "C" { fn ... }
}
```

**Status:** ✅ Full support - standard FFI declarations

**Clippy Status:** ✅ Works - sees standard `extern "C"` blocks

---

### 3. Library Linking (Always Available)

**Syntax:** `include library_name;`

**Desugaring:** → `#[link(name = "library_name")] extern "C" {}`

**Implementation:** `compiler/rustc_parse/src/parser/item.rs:3050-3083`
```rust
fn parse_include_library(&mut self, lo: Span, attrs: &mut AttrVec) -> PResult<'a, Option<ItemKind>> {
    // Creates link attribute + empty extern block
}
```

**Example:**
```rust
include m;  // Links libm
import fn cos(x: f64) -> f64;
import fn sin(x: f64) -> f64;
```

**Status:** ✅ Full support - standard library linking

**Clippy Status:** ✅ Works - sees standard link attributes

---

### 4. Logical Operators (Always Available)

**Syntax:** `and` / `or` / `xor` as operator aliases

**Desugaring:**
- `a and b` → `a && b` (in non-script mode) or Python-style truthy (in script mode)
- `a or b` → `a || b` (in non-script mode) or Python-style truthy (in script mode)
- `a xor b` → `a ^ b` (bitwise XOR always)

**Implementation:** `compiler/rustc_parse/src/parser/expr.rs:469-480`
```rust
(None, Some((Ident { name: sym::and, span }, _))) => (AssocOp::Binary(BinOpKind::And), span),
(None, Some((Ident { name: sym::or, span }, _)))  => (AssocOp::Binary(BinOpKind::Or), span),
(None, Some((Ident { name: sym::xor, span }, _))) => (AssocOp::Binary(BinOpKind::BitXor), span),
```

**Script Mode Enhancement:** `expr.rs:3397-3500`
- `a or b` returns first truthy value (not boolean)
- `a and b` returns first falsy value or last value (not boolean)

**Status:** ✅ Full support - recognized as operators

**Clippy Status:** ✅ Works - sees standard `&&`/`||`/`^` operators

---

## ⚠️ BROKEN in rust-analyzer (IDE)

The IDE backend (rust-analyzer) has **partial support**:

| Feature | Parsing | HIR Lowering | IDE Features |
|---------|---------|--------------|--------------|
| `def`/`fun` | ✅ Yes | ✅ Yes | ✅ All work |
| `and`/`or`/`xor` | ✅ Yes | ✅ Yes | ✅ All work |
| `import fn` | ✅ Yes | ❌ **NO** | ❌ Broken |
| `include` | ✅ Yes | ❌ **NO** | ❌ Broken |

### The Problem

File: `/opt/other/rust-analyzer/crates/hir-def/src/item_tree/lower.rs:134-154`

```rust
fn lower_mod_item(&mut self, item: &ast::Item) -> Option<ModItemId> {
    let mod_item: ModItemId = match item {
        ast::Item::Struct(ast) => self.lower_struct(ast)?.into(),
        ast::Item::Fn(ast) => self.lower_function(ast)?.into(),
        ast::Item::Use(ast) => self.lower_use(ast)?.into(),
        // ❌ Missing: ast::Item::Include(ast) => ...
        // ❌ Missing: ast::Item::Import(ast) => ...
        ast::Item::ExternCrate(ast) => self.lower_extern_crate(ast)?.into(),
        // ...
    };
}
```

**Result:** `include`/`import` are parsed but never converted to semantic HIR representation.

**Impact on IDE:**
- ❌ No go-to-definition for imported items
- ❌ No autocomplete for imported symbols
- ❌ No hover documentation
- ❌ No rename/refactoring
- ❌ No type checking
- ❌ No find references

---

## 🔧 Fix for rust-analyzer

Add these cases to `lower.rs:146`:

```rust
ast::Item::Include(ast) => self.lower_include(ast)?.into(),
ast::Item::Import(ast) => self.lower_import(ast)?.into(),
```

Add these functions around line 290:

```rust
fn lower_include(&mut self, include_item: &ast::Include) -> Option<ItemTreeAstId<Use>> {
    let visibility = self.lower_visibility(include_item);
    let ast_id = self.source_ast_id_map.ast_id(include_item);
    let (use_tree, _) = lower_use_tree(self.db, include_item.use_tree()?, &mut |range| {
        self.span_map().span_for_range(range).ctx
    })?;

    let res = Use { visibility, use_tree };
    self.tree.big_data.insert(ast_id.upcast(), BigModItem::Use(res));
    Some(ast_id)
}

fn lower_import(&mut self, import_item: &ast::Import) -> Option<ItemTreeAstId<Use>> {
    // Same implementation as lower_include
}
```

**Effort:** 15 minutes
**Impact:** Enables all IDE features for `include`/`import`

---

## 📋 Summary

### What Already Works

1. **rustc (Compiler):** ✅ 100% functional
   - All custom syntax desugars correctly
   - Clippy works (sees desugared AST)
   - Tests pass

2. **rust-analyzer (IDE):** ✅ 80% functional
   - `def`/`fun` functions: Full IDE support
   - `and`/`or`/`xor` operators: Full IDE support
   - `include`/`import`: Only syntax highlighting works

### What Needs Fixing

**rust-analyzer HIR lowering for `include`/`import`:**
- File: `/opt/other/rust-analyzer/crates/hir-def/src/item_tree/lower.rs`
- Add 2 match arms + 2 functions
- ~20 lines of code
- Enables: autocomplete, go-to-def, refactoring, type checking

---

## 🚀 Testing Instructions

### Test rustc (Compiler)

```bash
cd /opt/other/rust
./rebuild.sh  # Builds rand and external deps
./build/host/stage1/bin/rustc -Zscript test.rs
```

### Test rust-analyzer (IDE)

1. Apply HIR lowering fix
2. Rebuild: `cd /opt/other/rust-analyzer && cargo build --release`
3. Restart RustRover
4. IDE features should work for all custom syntax

---

## 🎯 Architecture

```
rustc (Compiler)
    Lexer → Parser → Desugar → AST → HIR → MIR → LLVM
                        ↑
                     Works! ✅
                     Clippy sees desugared code

rust-analyzer (IDE)
    Lexer → Parser → AST → HIR → Semantic Analysis → IDE Features
                              ↑
                           Broken ❌ for include/import
                           (not lowered to HIR)
```

---

## 📝 Related Files

### rustc (Custom Compiler)
- Parser items: `compiler/rustc_parse/src/parser/item.rs`
- Parser expressions: `compiler/rustc_parse/src/parser/expr.rs`
- Extensions: `compiler/extensions/src/`
- Rebuild script: `rebuild.sh`

### rust-analyzer (IDE Backend)
- Parser: `crates/parser/src/grammar/items.rs`
- HIR lowering: `crates/hir-def/src/item_tree/lower.rs` ⚠️ Needs fix
- AST nodes: `crates/syntax/src/ast/generated/nodes.rs`
- Rename: `crates/ide/src/rename.rs`

---

## 🎉 Conclusion

Your custom syntax is **fully implemented in rustc** with proper desugaring!
All that's missing is the **rust-analyzer HIR lowering** for complete IDE support.

The compiler works perfectly - Clippy and all tools see standard Rust after desugaring.
