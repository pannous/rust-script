# Claude Conversation (be7de39d)

## 👤 Human

From https://github.com/rust-lang/rust
   2b112efcccf..2641353c7e1  automation/bors/auto       -> upstream/automation/bors/auto
 + c7b75f95905...b9795620245 automation/bors/auto-merge -> upstream/automation/bors/auto-merge  (forced update)
 + e8af78d927e...79459778fb9 automation/bors/try        -> upstream/automation/bors/try  (forced update)
 + b980d187525...2d251910b5b automation/bors/try-merge  -> upstream/automation/bors/try-merge  (forced update)
 + a741008f5b0...9b1f8ff42d1 beta                       -> upstream/beta  (forced update)
   ba2a7d33741..fffc4fcf96b  main                       -> upstream/main
 + a1d39623125...ff2e9cec42a perf-tmp                   -> upstream/perf-tmp  (forced update)
 + ded5c06cf21...254b59607d4 stable                     -> upstream/stable  (forced update)
 + a1d39623125...ff2e9cec42a try-perf                   -> upstream/try-perf  (forced update)  Configure to ignore all that the main branch and delete all other branches but main master locally

---

## 🤖 Assistant

I'll configure git to only fetch the main branch from upstream and clean up the other remote-tracking branches.

---

## 🤖 Assistant

→ `Bash`: `git remote -v`

---

## 🤖 Assistant

→ `Bash`: `git config --get-all remote.upstream.fetch`

---

## 🤖 Assistant

→ `Bash`: `git config remote.upstream.fetch '+refs/heads/main:refs/remotes/upstream/main'`

---

## 🤖 Assistant

→ `Bash`: `git branch -r | grep upstream | grep -v '/main$'`

---

## 🤖 Assistant

→ `Bash`: `git branch -r | grep upstream | grep -v '/main$' | xargs -n1 git branch -d -r`

---

## 🤖 Assistant

→ `Bash`: `git config --get remote.upstream.fetch && echo "---" && git branch -r | grep upstream`

---

## 🤖 Assistant

Done. Upstream is now configured to only fetch `main`. 

Current state:
- **Fetch refspec**: `+refs/heads/main:refs/remotes/upstream/main`
- **Remote branches**: only `upstream/main` remains
- Deleted 11 unnecessary tracking branches (automation/bors/*, beta, stable, perf-tmp, try-perf, etc.)

Future `git fetch upstream` will only pull the main branch.

---

