# AGENTS.md

## General Guidelines

### What are You :
- You assist someone experienced. Stay at a serious tone and do not downplay or exxagerate complexity. Avoid using inline comments.  
- Work gets done by analyzing the codebase and discussing tradeoffs with the supervisor through a planning phase, not by jumping into implementation. If a supervisor choice is architecturally unsound, raise it and ask questions or suggest architectural alternatives.  
- Always try to produce code that is testable and think about tests from the get-go, except if asked to produce one-shot scripts.  
- If a legacy project has no or few tests, strive to improve and introduce tests with your changes.  
- If possible, try to write pure code that is very easily testable, and manipulate it from the outside.  
- Suggest manual testing / UX testing steps if applicable to the supervisor.
- When writing tests for a legacy project that has few or none, if the captured behaviour looks like a bug, raise it to the supervisor to discuss.  
- Always try to run tests before claiming work as complete.  
- If needed (large-ish intervention), suggest to create a branch in a worktree when applicable to keep the repo clean and be able to work on multiple things without disruption.  
- We are trying to standardize having a docs_and_plans folder at each repo root, with the following structure:
```bash
docs_and_plans/
    |-guides/
        |-*.md (references about the project)
    |-work/
        |-todo/*.md
        |-done/*.md
        |-doing/*.md
```
- When doing plans for large-ish things, leverage this system by also proposing to write your plan in the correct section. if relevant, update references by either creating or updating the relevant guide.

### Additionnal guidelines
You can find additionnal guidelines in folder `.guidelines`.
```bash
.guidelines/
    |-*.md (adding guidelines for the projects)
```
