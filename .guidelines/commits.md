# Commit Guidelines

## Philosophy

Commits are for you, not a team. Write them so that future-you can understand what happened and why. Keep it simple.

---

## Format

```
Short summary in present tense

Optional longer explanation if needed.
```

That's it. No ticket numbers, no tags, no formal structure.

---

## The Summary Line

- **Length**: 50 characters or less.
- **Tense**: Present tense, imperative mood ("Add feature", not "Added feature" or "Adds feature").
- **Capitalize**: Start with a capital letter.
- **No period**: Do not end with a period.
- **Be specific**: "Fix login bug" is better than "Fix bug".

### Good examples
```
Fix port parsing in connect command
Update README with workflow examples
Remove unused docker-compose template
```

### Bad examples
```
updated stuff
Fixed a thing.
WIP
asdf
```

---

## The Body (optional)

Add a body if the summary line is not enough. Leave one blank line between the summary and the body.

Use the body to explain:
- **Why** you made the change (the summary already says what changed).
- Any side effects or trade-offs.
- References to issues or discussions (if relevant).

Keep it short. A few sentences is usually enough.

### Example with body
```
Fix SSH connection timeout on remote hosts

The default timeout was too short for high-latency connections.
Increased to 30 seconds and added a fallback to local agent.
```

---

## Rules

- **One thing per commit**: If you catch yourself writing "and" in the summary, split it into two commits.
- **Test before committing**: Make sure the code works. Do not commit broken code.
- **Commit often**: Small, frequent commits are easier to understand and revert than large ones.
- **No "WIP" commits**: If you need to save progress, use a branch. Clean it up before merging.
- **No generated content**: Do not commit large diffs of auto-generated files (lock files, build artifacts, etc.) unless absolutely necessary.

---

## When to Commit

- After completing a logical change (a function, a fix, a refactor).
- Before switching tasks or taking a break.
- Before trying something risky (so you can revert easily).

---

## Checklist

- [ ] Summary is 50 characters or less
- [ ] Summary uses present tense imperative mood
- [ ] Commit contains only one logical change
- [ ] Code compiles and runs without errors
- [ ] Commit message explains why, not just what
