# Agent instructions

## Approve a plan before editing

- Before changing any file, propose a numbered list of small steps
- For each step, say what it changes and how it will be checked
- Wait for the engineer to approve the list before starting step 1

## One step per approval

- Complete exactly one step per go-ahead
- After each step, report what changed and what was checked
- Show the step list with finished steps marked and the next step named
- Stop and wait for approval before starting another step
- If new work appears, add it as a separate step and ask the engineer

## Break steps down on request

- When asked to zoom in on or break down a step, split it into small numbered
  substeps, such as 3.1, 3.2, and 3.3
- For each substep, say what it changes and how it will be checked
- Reply with only the substep list, then wait for approval before starting
- Allow substeps to be broken down in the same way

## Keep scope narrow

- Do only what was asked; avoid unrequested features, refactors, and fixes
- Keep changes minimal and focused on one change
- Mention unrelated problems separately in one line

## Keep code simple

- Write the smallest correct solution in plain, readable code
- Match nearby names, layout, and patterns
- Keep functions short with one job and use descriptive names
- Use early returns instead of deep nesting
- Add no abstractions, configuration options, or generic helpers until there
  is a second real use
- Add no speculative fallbacks, retries, or feature flags
- Add a dependency only if it removes substantial code or risk, and explain why
- Delete dead code instead of commenting it out
- Do not reformat untouched lines
- Write a simple, clear, concise, one-sentence comment above each line of code
- Skip comments for closing-brace-only lines and do not add "End" comments

## Keep commits simple

- Keep commit messages to one concise, plain-language line describing the change
