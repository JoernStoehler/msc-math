# Independent local retrieval check

Model: gpt-6-luna, fresh context, no web and no file mutation. The agent received only the skill path and four operational questions: ChatGPT personalization inside GPTs/Projects; current Claude Code instruction files vs Codex; autonomous tool loops/persistent sessions across APIs/SDKs; and measured Codex acquisition cost. It did not receive expected answers or the parent diagnosis.

Result: all four answers recovered relevant local references and source IDs. GPT and Project scope were separated; Claude AGENTS.md support was qualified by version/configuration; API/SDK ownership of loops/state/compute was distinguished; acquisition cost was qualified by time vs bytes/full vs sparse with the pinned tag. It consistently labeled documentation statements separately from runtime probes.

Limit: one qualitative agent check, not a comparative model benchmark or proof of general task-success improvement. The deterministic A/B/C retrieval and correction-visibility evaluation is separately reproducible in architecture-evaluation.json.
