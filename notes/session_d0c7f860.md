# Claude Conversation (d0c7f860)

## 👤 Human

fix 
TypeError: Cannot read properties of undefined (reading 'prototype')
    at file:///Users/me/.bun/install/global/node_modules/@anthropic-ai/claude-code/cli.js:634:25327
    at file:///Users/me/.bun/install/global/node_modules/@anthropic-ai/claude-code/cli.js:8:402

---

## 🤖 Assistant

This is a bug in the Claude Code CLI itself, not your Rust project. The error occurs in the bundled `cli.js` file when something is `undefined` where an object with a `prototype` was expected.

Let me check the current version and try upgrading:

---

## 🤖 Assistant

→ `Bash`: `bun pm ls -g 2>/dev/null | grep claude-code || echo "checking version another way"`

---

## 🤖 Assistant

→ `Bash`: `bun upgrade -g @anthropic-ai/claude-code 2>&1 || bun update -g @anthropic-ai/claude-code 2>&1`

---

## 🤖 Assistant

Upgraded from **2.0.17** → **2.1.12**. The error was likely in the older version. Try running `claude` again - the issue should be resolved.

---

