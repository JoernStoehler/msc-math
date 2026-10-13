# Agent-platform reference

A locally searchable, refreshable primary-source corpus for ChatGPT ordinary chats and Work, Codex CLI/app/cloud, OpenAI API and Agents SDK, Claude chat/Cowork, Claude Code and Agent SDK, and Agent Skills. Curated notes distinguish product surfaces, document corrections, and route agents to evidence.

Read `REVIEW.md` for outcome evidence and limitations. Start at `SKILL.md` and `references/INDEX.md`. Read corrected source views in `corpus/views/`; normalized vendor articles remain under `corpus/articles/`, and immutable upstream response bytes are in `corpus/snapshots/`. Never hand-edit generated mirrors or views.

## Validate and search

```bash
python -m pip install -r requirements.txt
python scripts/validate.py --require-mirrors
python -m pytest -q tests
rg -n -i 'project instructions' references corpus/views
```

## Refresh

```bash
python scripts/discover.py
python scripts/refresh.py sync --jobs 6
python scripts/build_views.py --evaluate > evidence/architecture-evaluation.json
python scripts/inventory.py
python scripts/validate.py --require-mirrors
python -m pytest -q tests
```

`references/maintenance-workflows.md` documents failure recovery and correction review. `VERIFICATION.md` records the final coverage, measurements and evidence limits. `references/skill-deployment.md` has deployment routes; no account settings, installed skill, or scheduled job was changed by this package.

`HANDOFF.md` is the original assignment, retained for provenance, not the current state. `evidence/legacy-seed-attempt.json` is the historical failed seed attempt; `corpus/manifest.jsonl` is the only current source manifest.

Known corrective incidents can use `python scripts/consult.py goal`, `questions`, `handoff`, `research`, `traces` or `models` to read one relevant reference. Examples contain disclosed-case task artifacts and proposed interventions, not additional user-installed skills.

## Repository ownership

Jörn authorized this repository as the maintained source on 13 October 2026. [The adoption record](evidence/repository-adoption-20261013.json) identifies the immutable received ZIP, retained inventory and bounded source changes. The corpus, curation, demonstrations and negative evidence remain together for offline portability. Deployment outside this repository remains a separate action.
