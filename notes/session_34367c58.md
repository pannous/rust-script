# Claude Conversation (34367c58)

## 👤 Human

How to use watchman with git?

---

## 🤖 Assistant

Watchman integrates with Git primarily through the **fsmonitor** feature, which speeds up `git status`, `git add`, and other commands by only checking files Watchman reports as changed.

## Setup

```bash
# Install watchman
brew install watchman

# Enable fsmonitor in your repo
git config core.fsmonitor true
git config core.untrackedcache true
```

## Verify it's working

```bash
git status  # First run initializes watchman
watchman watch-list  # Should show your repo
```

## For large repos (like this Rust fork)

```bash
# Update the index to use fsmonitor
git update-index --fsmonitor
```

## Custom triggers

You can also set up triggers to run commands on file changes:

```bash
# Watch for changes and run a command
watchman-make -p '**/*.rs' -t build

# Or set up a trigger directly
watchman -- trigger /opt/other/rust rebuild '*.rs' -- ./rebuild.sh
```

## Troubleshooting

```bash
watchman watch-del-all    # Reset all watches
watchman shutdown-server  # Restart watchman
```

The main benefit for this repo would be faster `git status` given the many submodules and files. Want me to enable fsmonitor for this repository?

---

## 👤 Human

/opt/other/rust/                                                                                      
  git config core.fsmonitor true                                                                                              
  git config core.untrackedcache true
/opt/other/rust/ watchman watch-list                                                                  
{
    "version": "2026.01.12.00",
    "roots": []
}

---

## 🤖 Assistant

The watchman daemon is running but hasn't started watching your repo yet. Git's fsmonitor will trigger it on the next `git status`:

```bash
git status
```

Then check again:

```bash
watchman watch-list
```

If it still shows empty roots, you can manually add the watch:

```bash
watchman watch /opt/other/rust
```

After that, `git status` should be noticeably faster on subsequent runs since Watchman will track which files changed rather than Git scanning everything.

---

