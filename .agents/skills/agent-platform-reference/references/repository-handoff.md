# Continue a repository on another agent surface

Use this when work moves between a workstation, coding CLI, cloud coding agent, or Claude chat/Cowork. The outcome is a destination agent that can run the project and show its result without rediscovering hidden machine assumptions.

## Cheapest consultation

Read this page when choosing the destination or preparing the handoff. Inspect the repo's instruction entry points, dependency/build configuration and relevant skill references; search the full corpus only for unresolved surface behavior. Reading all product documentation is unnecessary.

## Establish the actual destination

Claude chat/Cowork is not Claude Code. Cowork cloud execution uses a temporary per-session sandbox; connected-device access depends on allowed folders and an online desktop app. A local-folder Project stays on that computer. The reviewed documentation does not guarantee automatic `CLAUDE.md`/`AGENTS.md` loading, CLI skill discovery, Git checkout access or port forwarding in Cowork. Do not turn that absence of a guarantee into a universal capability prohibition. Inspect the tools and account surface before choosing a transfer route.

Sources: `claude-chat-cloud-cowork`, `claude-chat-cowork-merge`, and `evidence/handoff-source-review.md` with captured official architecture and Projects pages. The merged-experience Help page expressly says “Add from GitHub” is not supported; that is a specific UI route, not proof that every possible Git tool is unavailable.

## Prepare the portable contract

1. Read repository instructions and startup commands. Inspect absolute paths, environment variables, home-directory config, cache flags, symlinks and skill/tool dependencies reached by those commands. A clean Git checkout omits untracked files and personal skills.
2. Patch the owning configuration/script and instructions together. Derive project locations from the checkout or current environment; use writable cache paths for the destination. Do not merely warn about a broken workstation path while leaving executable flags or `AGENTS.md` stale.
3. Put required skill resources in the repo (when permitted) or make their installation explicit and repeatable. In Claude Code, `.claude/skills` is a documented project discovery path. In Cowork, the handoff prompt must explicitly point to relevant files; don't claim copying them installs a Chat Skill.
4. Update `AGENTS.md` with commands and environment assumptions. For a Claude Code recipient, inspect its actual instruction selection and add/repair a `CLAUDE.md` bridge when necessary. For chat/Cowork, explicitly tell the recipient to read the repo contract; a bridge file alone is not evidence it will be auto-loaded.
5. Verify changed files reached the transfer boundary. For a Git route, commit, push when authorized, and compare the intended commit to the remote ref; clean local status alone is insufficient. For upload, create and inspect the exact archive and commit identity, excluding credentials and machine caches. Do not assume a remote or connector is reachable at the destination.
6. Verify in a fresh checkout with a different home/path and without personal skills. Run the actual startup/test command with available prerequisites; state separately what couldn't run. Check that the contract and its resource links are present in the transferred content.
7. Supply the result through a destination-supported presentation route. A listening server or `localhost` URL is not a user-visible demo in another machine's sandbox. Use an exportable HTML/report, supported artifact, or actual deployed URL; explicitly identify a required interactive service if static export loses essential behavior. Do not deploy without task authorization.

## Handoff prompt content

Identify the exact transferred revision/input; tell the new agent to read the repository contract and required resource files; give the bootstrap/build/test commands, pending objective and completion checks; describe known destination limits and how to present results. Require verification of effective tools, paths and loaded instructions before assuming workstation behavior. Include remaining blockers instead of representing a partial bootstrap as successful migration.

## Evidence boundary

A relocated local checkout tests filesystem/build portability. It does not establish live Cowork Git access, automatic instruction discovery, actual Cargo compilation, account permissions or UI presentation. Record which of those was tested. This reference supplies the decision procedure; it does not grant tools, persist sandbox storage, or change the user's account.
