---
name: create-github-issues
description: 'Create GitHub issues. Use when you need to create issues. DO NOT USE when working on issue implementation.'
argument-hint: 'What issue should be created or drafted?'
user-invocable: true
disable-model-invocation: false
---

# Creating GitHub Issues

Create GitHub issues, following repository conventions.

## Workflow

For each issue to be created:

1. Use the `AskQuestions` tool to ask the user for any missing metadata before creating the issue:
   - the issue should have a parent issue?
   - the issue belongs to a milestone?
   - who's the assignee? ask if assignee should be "lucas-labs", "none" or other (user can specify a GitHub username or team)
   - labels? ask the user if they decided what labels should it have or if the AI should determine labels from existing labels in the repo.
2. Read the relevant parent issue if one exists.
3. If implementation details are unclear, use a read-only subagent to dig into the codebase and return:
   - current behavior
   - relevant files and modules
   - constraints that should shape the issue scope
   - risks or edge cases worth including in acceptance criteria
4. Draft the issue title and body.
5. Use the `AskQuestions` tool asking the user if they want to create the issue on Github, or it was just a draft. A third option MUST allow the user to enter possible  instructions for modifying the draft before creation.
6. When the user confirms creation, create the issue on GitHub.
7. Once the issue was created, write a table with issue number, title, milestone, labels, assignee, and url link.

## Title Conventions

Follow conventional commit style for titles, using emojis to visually distinguish issue types.
Examples:

- `feat(scope): ✨ short description`
- `refactor: 🔨 short description`
- `fix: 🚑 short description`
- `docs: 📝 short description`

Keep titles short, action-oriented, and descriptive.

## Body Conventions

Follow the following structure:

```md
## Summary

Short description of the problem or desired capability.

## Why it matters?

- Outcome-focused reason one.
- Outcome-focused reason two.
- Optional reason three.

## Acceptance Criteria

- Clear observable outcome one.
- Clear observable outcome two.
- Clear observable outcome three.

<!-- in case you need it or feel it's important, you can add a free "Important Notes" section,
     but separate it with "---" from the rest of the body, e.g.: -->

---
## Important Notes
<!-- free-form notes, complete as needed, but keep it concise and only include when it really adds value from a developer's perspective -->
```

## Acceptance Criteria Rules
- Write observable outcomes, not implementation steps.
- Cover the happy path and the important failure or edge path.
- Keep criteria scoped to one issue-sized chunk of work.
- Include documentation updates when the issue introduces new functionality or changes existing behavior that is already documented.
