# README.md Guidelines

## Philosophy

Keep it short. A solo dev readme only needs to answer three questions:
- What is this?
- How do I install / run it?
- How do I use it?

No formal versioning, no changelog, no company info. Write in plain, casual English.

---

## Structure

### 1. Title and one-liner
One sentence describing what the project does. No more.

### 2. Requirements
A short bullet list of what needs to be installed on the machine before using the project.
Only list non-obvious dependencies (skip things like "a terminal").

### 3. Installation
The exact commands to clone and set up the project, nothing else.
Use a code block.

### 4. Usage
Show the most common commands or use cases with short code examples.
If the tool has subcommands, a small list with a one-line description for each is enough.

### 5. Configuration (optional)
Only include this section if the project has a config file or environment variables.
A small table or list with the variable name, default value, and a short description is sufficient.

### 6. Notes (optional)
Any quirk, known limitation, or useful tip that does not fit elsewhere.
Keep it to a few bullet points.

---

## Rules

- **Length**: aim for a readme that fits on one screen. If it does not, cut it.
- **Tone**: casual and direct. Write as if explaining to a friend.
- **Language**: English only.
- **Tense**: use the present tense and active voice ("Run this command", not "This command should be run").
- **Code blocks**: always specify the language for syntax highlighting.
- **No badges**: no CI badges, no license shields, no coverage indicators.
- **No changelog**: no version history, no release notes.
- **No contribution section**: this is a solo project.
- **No license section**: add a LICENSE file if needed, do not repeat it in the readme.
- **No emojis**.

---

## Checklist

- [ ] One-liner describes the project clearly
- [ ] Requirements list is minimal and accurate
- [ ] Installation commands are copy-pasteable and tested
- [ ] At least one usage example is shown
- [ ] Configuration variables are documented if any exist
- [ ] Readme fits roughly on one screen