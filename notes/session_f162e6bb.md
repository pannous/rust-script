# Claude Conversation (f162e6bb)

## 👤 Human

Rust Rover is missing the option to Debug  #[test]
// #[ignore] // works but it's too slow
fn test_parse_all_samples() {

---

## 🤖 Assistant

This is typically a RustRover/IDE issue rather than a code issue. Common causes:

1. **Missing `#[test]` attribute** - Make sure it's actually present (not just in a comment)

2. **Invalidate caches** - RustRover → File → Invalidate Caches → Invalidate and Restart

3. **Re-sync Cargo** - Click the refresh icon in the Cargo tool window or run "Reload Cargo Projects"

4. **Check run configuration** - The gutter icon (▶️) next to the test should have a dropdown. If it only shows "Run", you may need to:
   - Right-click the test → More Run/Debug → Debug
   - Or create a manual Run Configuration: Run → Edit Configurations → + → Cargo → set command to `test --package <pkg> --lib test_parse_all_samples`

5. **Ensure debug symbols** - In `Cargo.toml`:
   ```toml
   [profile.dev]
   debug = true
   ```

6. **Check if test is detected** - Run from terminal to verify:
   ```bash
   cargo test test_parse_all_samples --no-run
   ```

If the test runs from terminal but RustRover still doesn't show Debug, it's likely an indexing issue. Which of these have you tried?

---

## 👤 Human

[profile.dev]
incremental = true
# Maximum build speed optimizations
opt-level = 0          # No optimization for faster compile
#debug = 0              # Minimal debug info for faster compile (override with CARGO_PROFILE_DEV_DEBUG=true)
#debug = false         # fast by default (IDE overrides with CARGO_PROFILE_DEV_DEBUG=true or use [profile.fast])
debug = true           # IDE needs debug symbols for breakpoints
split-debuginfo = "unpacked"  # Faster on macOS
codegen-units = 16     # Rust default (256 was wasteful)

[profile.test]
inherits = "dev"       # Use dev profile for tests

---

## 🤖 Assistant

I see the issue - the Debug option is missing from the gutter dropdown. This is a known RustRover quirk.

**Quick fixes:**

1. **Use keyboard shortcut** - Place cursor on the test function and press `^⇧D` (Ctrl+Shift+D) instead of `^⇧R`

2. **Right-click the gutter icon** - Sometimes right-click shows more options than left-click

3. **Create debug config manually:**
   - Click "Modify Run Configuration..."
   - In the dialog, there should be a "Debug" button next to "Run" at the bottom
   - Or duplicate the config and it may then show both Run/Debug options

4. **Check Native Debugger plugin:**
   - RustRover → Settings → Plugins → Installed
   - Ensure "Native Debugging Support" is enabled
   - Restart if you enable it

5. **Alternative approach** - In the top toolbar, after running once, the configuration appears in the dropdown. Use the bug icon (🪲) next to the play button to debug it.

The missing Debug option in the gutter menu is often a UI bug - the debug capability is usually still there via other entry points.

---

