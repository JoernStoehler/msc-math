# Worktrees And Git

Use this file for repo-local Git and worktree behavior.

## Base And Checkout

- Use local `main` as the base, not `origin/main`.
- Use `/workspaces/msc-math` on `main` only when the task deliberately targets
  the root checkout or Jörn explicitly grants main-checkout work.
- Create a worktree when the task asks for isolated edits or when parallel
  sessions will edit overlapping tracked files.
- Use local `main` unless Jörn names a different base:
  `git worktree add -b <branch> .codex/worktrees/<branch> main`

## Subagents

- Every subagent prompt names the required cwd.
- `spawn_agent` cannot set cwd; subagents must anchor commands and edits from
  their own tools.

## Commits And Merge

- Agents may commit without asking. Ask about merge approval, not commit
  permission.
- Before merging to `main`, run the `pre-merge` skill and get explicit approval
  from Jörn.
- After merge, remove the worktree with
  `git worktree remove .codex/worktrees/<branch>` and delete the branch with
  `git branch -d <branch>`.
- Destructive operations such as force-push, branch deletion on `main`,
  `git reset --hard`, and checkout-based reverts require explicit approval.

## LFS

- Git LFS tracks `.jsonl` files through `.gitattributes`.
- `git add`, `commit`, and `push` work normally.
- A pre-commit hook blocks non-LFS files larger than 10 MB.

