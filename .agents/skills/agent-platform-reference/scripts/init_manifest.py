#!/usr/bin/env python3
"""Reproducible initial source inventory. It does not mark sources as downloaded."""
import json
from pathlib import Path
root = Path(__file__).resolve().parents[1]
# These URLs were selected after examining official pages and the Claude Code llms.txt index.
sources = [
('chatgpt-custom-instructions','openai','chatgpt','https://help.openai.com/en/articles/8096356-chatgpt-custom-instructions','html',1),
('chatgpt-projects','openai','chatgpt','https://help.openai.com/en/articles/10169521-projects-in-chatgpt','html',1),
('chatgpt-memory','openai','chatgpt','https://help.openai.com/en/articles/8590148-memory-in-chatgpt','html',1),
('chatgpt-work-codex','openai','chatgpt','https://help.openai.com/en/articles/20001275-chatgpt-work-and-codex','html',1),
('chatgpt-model-selection','openai','chatgpt','https://help.openai.com/en/articles/20001354-gpt-6-and-other-models-in-chatgpt','html',1),
('chatgpt-skills','openai','chatgpt','https://help.openai.com/en/articles/20001066-skills-in-chatgpt','html',1),
('chatgpt-plugins','openai','chatgpt','https://help.openai.com/en/articles/20001256-plugins-in-chatgpt','html',2),
('chatgpt-gpts','openai','chatgpt','https://help.openai.com/en/articles/8554407-memory-in-chatgpt','html',1),
('chatgpt-personality','openai','chatgpt','https://help.openai.com/en/articles/11899719-customizing-you','html',2),
('chatgpt-characteristics','openai','chatgpt','https://help.openai.com/en/articles/20001038-characteristics-in-chatgpt','html',2),
('chatgpt-release-notes','openai','chatgpt','https://help.openai.com/en/articles/6825453-chatgpt-release-notes','html',2),
('codex-agents-md','openai','codex','https://developers.openai.com/codex/guides/agents-md','html',1),
('codex-skills','openai','codex','https://developers.openai.com/codex/skills','html',1),
('codex-skill-evals','openai','codex','https://developers.openai.com/blog/eval-skills','html',2),
('codex-prompt-rethink','openai','codex','https://developers.openai.com/blog/rethinking-skills-and-prompts-for-gpt-6-astra','html',1),
('openai-agents-api','openai','api','https://developers.openai.com/api/docs/guides/agents','html',2),
('openai-agents-config','openai','api','https://developers.openai.com/api/docs/guides/agents-api/configuration','html',1),
('openai-skills-api','openai','api','https://developers.openai.com/api/docs/guides/tools-skills','html',1),
('openai-plugin-skills','openai','api','https://developers.openai.com/plugins/concepts/skills','html',2),
('agent-skills-spec','standard','agent-skills','https://agentskills.io/specification','html',1),
('claude-code-index','anthropic','claude-code','https://code.claude.com/docs/llms.txt','plain',1),
('claude-code-overview','anthropic','claude-code','https://code.claude.com/docs/en/overview.md','markdown',2),
('claude-code-features','anthropic','claude-code','https://code.claude.com/docs/en/features-overview.md','markdown',1),
('claude-code-memory','anthropic','claude-code','https://code.claude.com/docs/en/memory.md','markdown',1),
('claude-code-skills','anthropic','claude-code','https://code.claude.com/docs/en/skills.md','markdown',1),
('claude-code-context','anthropic','claude-code','https://code.claude.com/docs/en/context-window.md','markdown',1),
('claude-code-subagents','anthropic','claude-code','https://code.claude.com/docs/en/sub-agents.md','markdown',1),
('claude-code-hooks','anthropic','claude-code','https://code.claude.com/docs/en/hooks-guide.md','markdown',2),
('claude-code-mcp','anthropic','claude-code','https://code.claude.com/docs/en/mcp.md','markdown',2),
('claude-code-debug','anthropic','claude-code','https://code.claude.com/docs/en/debug-your-config.md','markdown',1),
('claude-code-goal','anthropic','claude-code','https://code.claude.com/docs/en/goal.md','markdown',2),
('claude-code-changelog','anthropic','claude-code','https://code.claude.com/docs/en/changelog.md','markdown',1),
('claude-code-sessions','anthropic','claude-code','https://code.claude.com/docs/en/sessions.md','markdown',2),
('claude-code-agent-sdk','anthropic','claude-code','https://code.claude.com/docs/en/headless.md','markdown',2),
]
path=root/'corpus'/'manifest.jsonl'
if path.exists():
    raise SystemExit('Manifest already exists; refusing to overwrite')
with path.open('w',encoding='utf-8') as f:
    for id,vendor,product,url,fmt,priority in sources:
        rec=dict(id=id,vendor=vendor,product=product,url=url,format=fmt,priority=priority,path=f'corpus/articles/{vendor}/{product}/{id}.md',status='pending',last_checked_utc=None,last_success_utc=None,etag=None,last_modified=None,sha256=None,bytes=None)
        f.write(json.dumps(rec,sort_keys=True)+'\n')
print(f'Created {len(sources)} source records at {path}')
