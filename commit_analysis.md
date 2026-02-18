# Upstream Rust Compiler Commits Analysis (320 commits)

## 1. **Stabilized Features** (Features moved from nightly to stable)

- **`assert_matches`** - Stabilize the `assert_matches!` macro for pattern matching assertions
- **`atomic_try_update`** - Stabilize atomic compare-and-swap update operations
- **`core::hint::cold_path`** - Stabilize hint for marking cold execution paths
- **Const ControlFlow predicates** - Stabilize const evaluation of ControlFlow methods
- **New inclusive range type and iter** - Stabilize new range type and iterator functionality
- **`int_from_ascii` for `NonZero<T>`** - Implement ASCII parsing for non-zero integers

## 2. **New Language Features** (New syntax, semantics, or capabilities)

- **Pattern types in libcore** - Start using pattern types (experimental type system feature)
- **Heterogeneous & homogeneous try blocks** - Implement `try { }` blocks with type-flexible error handling
- **MVP for opaque generic const arguments (mGCA)** - Major work on generic const arguments including:
  - Support for type const syntax (`type const <IDENT> = <EXPR>`)
  - Associated const obligations in well-formedness checking
  - Support for directly represented negated literals
  - Require `#[type_const]` on free consts
  - Transform args for inherent type_consts
- **Truthy/falsy values** - Allow non-boolean values in conditions
- **Optional chaining** - Support for `?.` and `??` operators (related to your fork!)
- **`unreachable_cfg_select_predicates` lint** - New lint for unreachable cfg predicates

## 3. **Standard Library Additions** (New APIs, types, methods)

- **`BinaryHeap::from_raw_vec`** - Construct BinaryHeap from raw Vec without validation
- **`NonZero::<T>::from_str_radix`** - Parse non-zero integers with custom radix
- **`std::path` normalize methods** - Introduce path normalization at top of std::path
- **`TrustedLen` for `ArrayWindows`** - Implement trusted length trait for array windows iterator
- **`TrustedRandomAccess` for `ArrayWindows`** - Implement trusted random access for array windows
- **`FusedIterator` for `ArrayWindows`** - Implement fused iterator for array windows
- **Stdio FD constants** - Implement standard file descriptor constants
- **`const { 'Σ'.len_utf8() }`** - Enable const UTF-8 length computation

## 4. **Major Internal Refactors** (Significant compiler architecture changes)

### Query System Overhaul (largest refactor category):
- **Move `rustc_query_system::plumbing` to `rustc_query_impl::execution`** - Major reorganization (parts 1 & 2)
- **Move `rustc_query_system::cache` to `rustc_middle::traits`** - Rename `Cache` as `WithDepNodeCache`
- **Move `rustc_middle::query::values` to `rustc_query_impl`** - Query value handling reorganization
- **Move `rustc_query_system::query::job` parts to `rustc_middle::job`** - Job tracking reorganization
- **Move `try_get_cached` from `rustc_query_system` to `rustc_middle`** - Cache lookup reorganization
- **Remove `trait QueryDispatcher`** - Simplify query dispatch mechanism
- **Remove `QueryDispatcherUnerased::Dispatcher` associated type** - Simplification
- **Clarify names of `QueryVTable` functions** - Better naming for query execution
- **Clean up query macros for `cache_on_disk_if`** - Macro simplification
- **Rename `JobOwner` to `ActiveJobGuard`** - Better naming
- **Include `key_hash` in `ActiveJobGuard`** - Add hash for active jobs
- **Derive `Clone` for `QueryJob` and `QueryLatch`** - Enable cloning
- **Move needs-drop check for `arena_cache` queries out of macro code** - Simplification
- **Provide cycle handler for `GenericPredicates`** - Prevent cycles in trait resolution
- **Prevent cycles when lowering from supertrait elaboration** - Cycle prevention

### Diagnostics Infrastructure Overhaul (major migration):
- **Remove fluent infrastructure** - Major removal of localization system:
  - Remove `DiagMessage::FluentIdentifier`
  - Remove slugs from `#[derive(Diagnostic)]` macro
  - Remove fallback bundle
  - Remove fluent tests in `rustc_error`
  - Remove `rustc_fluent_macro`
  - Remove tidy fluent file checks
  - Remove `ui-fulldeps` tests for slugs
- **Convert to inline diagnostics** - Massive migration across crates:
  - `rustc_lint`
  - `rustc_parse`
  - `rustc_const_eval`
  - `rustc_mir_build`
  - `rustc_passes`
- **Remove `SubdiagMessage` in favour of `DiagMessage`** - Unification
- **Convert `impl Subdiagnostic` to derives** - Use macros instead of manual impls
- **Remove empty subdiagnostics** - Cleanup

### Attribute Parser Migration (systematic port to new parser):
- **Port to new attribute parser**:
  - `#![test_runner]`
  - `rustc_object_lifetime_default`
  - `rustc_no_mir_inline`
  - `rustc_trivial_field_reads`
  - `rustc_never_type_options`
  - `rustc_capture_analysis`
  - `rustc_conversion_suggestion`
  - `rustc_deprecated_safe_2024`
  - `rustc_expected_cgu_reuse`
  - `rustc_no_implicit_bounds`
  - `rustc_reservation_impl`
  - `rustc_strict_coherence`
  - `rustc_insignificant_dtor`
  - `rustc_outlives`
  - `rustc_evaluate_where_clauses`
  - `rustc_delayed_bug_from_inside_query`
  - `rustc_regions`
  - `rustc_intrinsic_const_stable_indirect`
  - `rustc_intrinsic`
  - `rustc_symbol_name`
  - `rustc_def_path`

### Other Major Refactors:
- **Modularize custom fork code to reduce merge conflicts** - Your custom fork refactor!
- **Move `impl Interner for TyCtxt` to its own submodule** - Better organization
- **Move ADT related code to sub module for type info** - Type reflection organization
- **Uplift `Predicate::allow_normalization` to `rustc_type_ir`** - Move to shared IR
- **Move target machine factory error reporting into codegen backend** - Better separation
- **Use cg_ssa's produce_final_output_artifacts in cg_clif** - Code sharing
- **Remove tm_factory field from CodegenContext** - Simplification
- **Streamline `rustc_span::HashStableContext`** - Hash stability improvements
- **Tweak query key trait bounds** - Trait bound refinement
- **Borrowck: simplify diagnostics for placeholders** - Borrow checker simplification

## 5. **Diagnostics Improvements** (Better error messages)

- **Fix suggestion on `CallToDeprecatedSafeFnRequiresUnsafe`** - Better unsafe hints
- **Fix help on `AmbiguousMissingKwForItemSub`** - Clearer keyword suggestions
- **Fix suggestion on `AutoTraitItems`** - Better auto-trait diagnostics
- **Add help message suggesting explicit reference cast for From/TryFrom** - Better conversion hints
- **Modernize diagnostic for indeterminate trait object lifetime bounds** - Clearer lifetime messages
- **Fix diagnostics in closure signature mismatch** - Better closure error messages
- **Add note for `?Sized` params in int-ptr casts diag** - Better cast diagnostics
- **Add note when inherent impl for alias type defined outside crate** - Better type alias hints
- **rustc_parse: improve error diagnostic for "missing let"** - Better let binding errors
- **rustc_parse_format: improve diagnostics for unsupported debug `=` syntax** - Better format string errors
- **Fix error spans for asm!() args that are macros** - Better inline assembly diagnostics
- **borrowck: suggest `&mut *x` for pattern reborrows** - Better reborrow suggestions
- **Suppress unused_mut lint if mutation fails due to borrowck error** - Reduce noise
- **Split `parse_inner_attr` errors by case** - More specific attribute errors
- **Split `ComparisonOrShiftInterpretedAsGenericSugg`** - More specific generic suggestions
- **Use `display_source_code()` in `ReferenceConversion`** - Show source in diagnostics
- **Cover more cases for parentheses in `&(impl Trait1 + Trait2)`** - Better trait bound formatting

## 6. **Bug Fixes** (Notable fixes)

- **Fix issue #152482, #152004, #151124, #152347** - Various ICE fixes
- **Fix copy-paste bug in report_sub_sup_conflict** - Used wrong trace cause
- **Fix rustdoc macro call highlighting** - Correctly detect macro calls
- **Fix bound var resolution for trait aliases** - Trait alias type resolution
- **Fix ICE in vtable iteration for trait reference in const eval** - Const eval crash fix
- **Fix `SourceFile::normalized_byte_pos`** - File position calculation
- **Fix passing/returning structs with 64-bit SPARC ABI** - SPARC ABI correctness
- **Fix lockfile** - Cargo.lock issues
- **prevent incorrect layout error** - Layout computation fix
- **GVN: Do not unify dereferences if they are references** - Optimization soundness
- **GVN: Do not introduce new dereferences if they borrow from non-SSA locals** - Optimization safety
- **GVN: Remove invalidate_derefs** - Fix optimization bug
- **Handle race when coloring nodes concurrently as both green and red** - Concurrency fix
- **Weaken `assert_dep_node_not_yet_allocated_in_current_session` for multiple threads** - Multi-threading fix
- **Correctly implement client side request cancellation support** - LSP cancellation
- **Avoid bogus THIR span for `let x = offset_of!(..)`** - Better span tracking
- **std: Don't panic when removing nonexistent UEFI var** - UEFI robustness
- **libm: Fix acoshf and acosh for negative inputs** - Math library correctness
- **libm: Fix tests for lgamma** - Math library testing
- **Infer expected len in `include_bytes!()` to avoid mismatches** - Macro correctness
- **Fix not complete `.not` in condition** - Boolean operator fix
- **fix linking of postcard test** - Test infrastructure fix
- **Allow `deprecated(since = "CURRENT_RUSTC_VERSION")`** - Deprecation system fix
- **check stalled coroutine obligations eagerly** - Async/generator fix
- **Fix rustdoc links for BoundRegionKind/BoundTyKind** - Documentation links

## 7. **Platform/Target Changes** (New targets, platform-specific changes)

### New Targets:
- **`s390x-unknown-none-softfloat`** - New bare-metal s390x target with software floating point
- **Add `avr_target_feature`** - AVR target feature support
- **Require sram target feature on AVR** - AVR memory requirement

### Target Improvements:
- **Bump tvOS, visionOS and watchOS Aarch64 targets to tier 2** - Apple platform promotion
- **Disable profiler runtime on tvOS and watchOS** - Apple platform fixes
- **Fix `@needs-backends` compiletest directive** - Better backend testing
- **Set crt_static_allow_dylibs to true for Emscripten target** - Emscripten linking
- **hexagon: Make `fma` label local to avoid symbol collision** - Hexagon platform fix
- **Implement __sync builtins for thumbv6-none-eabi** - ARM Cortex-M0 atomics

### Target Refactors:
- **Renamed RustcAbi::X86Softfloat to Softfloat** - Made platform-agnostic
- **Allow for variant aliases in target_spec_enum!** - Target spec flexibility

## 8. **CI/Infrastructure** (Build system, CI changes)

### Build System:
- **`./rebuild.sh` and `./rebuild.sh cache`** - Your custom build scripts!
- **Update to LLVM 22 rc 3** - LLVM version bump
- **Update to Xcode 26.2** - macOS toolchain update
- **Update wasi-sdk used in CI/releases** - WebAssembly toolchain
- **Always use default Clang on macOS** - macOS build simplification
- **Remove AR=ar env var** - Build cleanup
- **Remove USE_XCODE_CLANG** - Build simplification
- **Remove -Alinker-messages** - Build output cleanup
- **Unset WINAPI_NO_BUNDLED_LIBRARIES** - Windows build fix
- **Only set CFG_COMPILER_HOST_TRIPLE when building rustc** - Build optimization
- **Remove a couple of unnecessary stage checks** - Build simplification
- **bootstrap: always propagate `CARGO_TARGET_{host}_LINKER`** - Build consistency
- **Set `codegen-units=1` for benchmarks** - Benchmark optimization

### CI Updates:
- **ci: Pin rustc on native PowerPC job** - CI stability
- **ci: Temporarily disable native PPC and s390x jobs** - CI maintenance
- **ci: Update all docker images to latest version** - Container updates
- **ci: Enable verbose output for josh-sync** - Better CI logging
- **triagebot: Switch to `check-commits = "uncanonicalized"`** - Commit checking
- **compiletest: `-Zunstable-options` for json targets** - Test infrastructure
- **Disable the `run-make/translation` test for now** - Test maintenance
- **Ignore all debuginfo tests for LLDB that we do not run in CI** - Test cleanup

### Documentation:
- **improve build docs** - Better build documentation
- **Update CI documentation for branch names** - CI docs update
- **Fix typos and capitalization across documentation** - Documentation cleanup
- **Fix typos and grammar in top-level and src/doc documentation** - More doc cleanup
- **Use relative paths for std links in rustc-docs** - Better doc linking
- **Document `-Zcache-proc-macros`** - Proc macro caching docs
- **Document feature gate checking** - Feature gate system docs
- **Add cross-links between feature gate docs** - Better doc navigation
- **Clarify and reorganize feature-gate-check doc** - Feature gate clarity
- **Update translation documentation** - Translation system docs
- **Update diagnostics documentation** - Diagnostics system docs
- **Update documentation of rustc_macros** - Macro system docs

### Dependency Updates:
- **Update cargo submodule** (multiple times) - Cargo updates
- **Bump salsa** - Salsa dependency update
- **Bump time from 0.3.44 to 0.3.47** - Security/bug fix update
- **meta: Upgrade all dependencies to latest compatible versions** - Mass dependency update
- **meta: Switch to workspace dependencies** - Cargo workspace improvement
- **meta: Sort Cargo.toml `[features]` table after `[dependencies]`** - Cargo.toml organization
- **Remove unused `fluent-syntax` dependency from tidy** - Dependency cleanup

### Project Organization:
- **Update books** - Rust book updates
- **Remove compiler adhoc group** - Triagebot cleanup
- **Remove project-exploit-mitigations adhoc group** - Triagebot cleanup
- **Remove project-stable-mir adhoc group** - Triagebot cleanup
- **Remove project-const-traits adhoc group** - Triagebot cleanup
- **Remove types adhoc group** - Triagebot cleanup
- **Remove rustdoc adhoc group** - Triagebot cleanup
- **Add reddevilmidzy to review group** - Reviewer addition
- **re-add TaKO8Ki to triagebot review queue** - Reviewer re-addition
- **Add 'C-date-reference-triage' to exclude_labels** - Triage automation
- **Add label for date reference triage** - Triage labels

## 9. **Type System & Const Generics Work**

- **Support unions in type info reflection** - Type reflection for unions
- **Support enums in type info reflection** - Type reflection for enums
- **Support structs in type info reflection** - Type reflection for structs
- **Add generics info for structs in type info** - Generic type reflection
- **Simplify writing of tuple type info** - Tuple type reflection optimization
- **Erase type lifetime before writing type ID** - Type ID computation
- **Introduce helper `ty::Generics::has_own_self`** - Generics helper method
- **Use `.map.collect` to aggregate in `.to_ty` of tuples** - Type conversion optimization
- **Fix bound var resolution for trait aliases** - Trait alias type resolution

## 10. **Code Quality & Cleanup**

- **Remove now-unnecessary indirection** - Code simplification (multiple instances)
- **cleanup: Perform some simplifications possible with the MSRV bump** - Use newer features
- **Bump the libm MSRV to 1.67** - Minimum version bump
- **Format heterogeneous try blocks** - Formatting improvement
- **style: remove unneeded trailing commas** - Style cleanup
- **Simplify redundant description** - Documentation cleanup
- **Remove some empty subdiagnostics** - Dead code removal
- **Remove a couple of unused errors** - Dead code removal
- **Remove unused comments related to SyntaxErrorKind** - Comment cleanup
- **Remove outdated fixme** - Comment cleanup
- **Remove out-of-date comment** - Comment cleanup
- **Fix typo** (multiple) - Typo fixes
- **Allow more capitalized words** - Spellcheck update
- **deduplicate `Tag` enum** - Code deduplication
- **remove `Closure` generics** - Simplification
- **remove `'static` in many places** - Lifetime simplification
- **expand `define_reify_functions!`** - Macro expansion
- **keep const blocks around** - Const evaluation improvement

## 11. **Testing Improvements**

- **Add tests for issues 152004, 151124, 152347** - Regression tests
- **Add regression test for macro call highlighting** - Rustdoc test
- **Add regression test for inherent type_const normalization** - Type system test
- **Reorganize tests that no longer crash** - Move from crashes/ to ui/
- **Move tests from tests/crashes to tests/ui now that they don't ICE** - Test organization
- **Add ui test insufficient-suggestion-issue-141679.rs** - Diagnostics test
- **Regression test for `let x = offset_of!(..) else { .. }` span** - Span test
- **Add miscompiled test cases for GVN** - Optimization correctness tests
- **add regression test** - Generic regression test
- **Add selection test case** - Selection test
- **Add test case for select non-first branch** - Selection edge case
- **cmse: additional argument passing tests** - ARM security extension tests
- **symcheck: Add tests for symbol checker** - Symbol checking tests
- **symcheck: Check for core symbols with new mangling** - Mangling test
- **symcheck: Enable wasm by default** - WebAssembly symbol checking
- **add tests for s390x-unknown-none-softfloat** - New target tests
- **Add extra test to ensure highlighting for macros is working** - Rustdoc test
- **improve tests** - Test quality improvement
- **use `test` instead of `unix` to be platform-independent** - Cross-platform tests
- **libm-test: Remove exception for fmaximum_num tests** - Math test improvement
- **add test for simd from array repeat codegen** - SIMD codegen test

## 12. **Compiler Backend & Codegen**

- **Remove support for ScalarPair unadjusted arguments** - ABI cleanup
- **Don't compute FnAbi for LLVM intrinsics in backends** - Optimization
- **Reduce usage of FnAbi in codegen_llvm_intrinsic_call** - Simplification
- **extract `TyAndLayout::peel_transparent_wrappers` helper** - Layout helper
- **cmse: don't use `BackendRepr` when checking return type** - ARM CMSE fix
- **Stop having two different alignment constants** - Deduplication
- **Don't import `TyCtxt` from `crate::ty::context`** - Import cleanup
- **Indicate that `bidirectional_lang_item_map!` declares functions** - Macro documentation
- **Drop dump test for type info reflection** - Test removal

## 13. **Const Evaluation & MIR**

- **Also duplicate `#[expect]` attribute in `#[derive]`-ed code** - Lint attribute handling
- **Allow only a single accepter per attribute** - Attribute validation
- **Prevent const stability attrs from being applied to macros** - Const stability fix
- **Restrict the set of things that const stability can be applied** - Const stability validation
- **Remove accidental const stability marker on a struct** - Const stability fix
- **Split ol mapper into more specific to/kernel/from mapper** - Optimization refactor
- **Add some conversion trait impls** - Trait implementations
- **use `mem::conjure_zst` directly** - ZST handling
- **make `with_api!` take explicit type paths** - Macro improvement
- **simplify some other generics** - Generic simplification

## 14. **Safety & Security**

- **FCW for derive helper attributes that will conflict with built-in attributes** - Future compatibility warning
- **Replace some `feature(core_intrinsics)` with stable hints** - Reduce unsafe surface
- **Remove unnecessary `unsafe(non_update_types)`** - Safety cleanup
- **Prevent incorrect layout error** - Memory safety
- **RwLock: refine documentation to emphasize non-reentrancy guarantees** - Concurrency safety docs
- **BikeshedGuaranteedNoDrop trait: add comments that it can be observed on stable** - Safety documentation

## 15. **Lints & Warnings**

- **add `unreachable_cfg_select_predicates` lint** - New lint for cfg predicates
- **make the lint more sophisticated** - Lint improvement
- **Suppress unused_mut lint if mutation fails due to borrowck error** - Reduce false positives
- **Allow unstable_name_collisions** - Lint exemption

## Summary Statistics

**Most Active Areas:**
1. **Query System Refactor** (~25 commits) - Major architectural cleanup
2. **Diagnostics Infrastructure** (~30 commits) - Removal of fluent, inline diagnostics
3. **Attribute Parser Migration** (~20 commits) - Systematic modernization
4. **Generic Const Arguments (mGCA)** (~10 commits) - Major feature development
5. **Type Info Reflection** (~7 commits) - New type system capability

**Most Significant Changes:**
- **mGCA implementation** - Major new language feature for const generics
- **Try blocks** - New control flow construct
- **Query system overhaul** - Massive internal refactor for maintainability
- **Fluent removal** - Complete removal of localization infrastructure
- **Attribute parser migration** - Systematic modernization of ~20 attributes

**Breaking Changes:**
- Remove ScalarPair unadjusted arguments (ABI change)
- Remove support for old diagnostic infrastructure (internal)
- Various const stability restrictions (compile-time)

**Your Fork's Relevance:**
Several upstream commits relate to features you've already implemented:
- Optional chaining (`?.`, `??`) - you have this!
- Truthy/falsy values - you have this!
- Try blocks - you have similar features
- String operations - you have extensive string methods
- Math operators (`**`, `≈`) - you have these!

This analysis shows Rust is actively investing in const generics, improving diagnostics, and doing major internal cleanup while maintaining stability.
