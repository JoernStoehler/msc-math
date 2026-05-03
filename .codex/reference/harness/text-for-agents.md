# Text For Agents

Use this file when writing or reviewing files, comments, prompts, task packets,
and other text that future agents will read.

Optimize for these properties, in order:

1. **Correct, corrigible:** Verify claims against code or data. Cite sources or
   commands when a future agent needs to check the claim.
2. **Observable, measurable:** State checks the reader can run.
3. **Unambiguous:** Each sentence should have one reading.
4. **Complete:** Include assumptions, preconditions, and the reason behind
   non-obvious decisions.
5. **Actionable:** The reader should know what to do next.
6. **Simple and concrete:** Prefer familiar patterns, examples, and literal
   terms.

## Vague-Phrase Check

Words such as "appropriate", "properly", "ensure", "good", "consider",
"reasonable", "necessary", "efficient", and "robust" often hide missing
criteria.

Treat them as search triggers, not banned tokens. Rewrite only when the phrase
has multiple plausible readings that would change future agent behavior.
Preserve precise project terms when replacing the word would change the meaning,
and state the observable condition when the word is a task criterion.
