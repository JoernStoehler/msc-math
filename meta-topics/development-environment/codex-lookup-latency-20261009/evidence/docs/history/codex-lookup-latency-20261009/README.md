# Codex lookup timing evidence, 9 October 2026

Native subagent tests used no parent conversation history and no task-specific discovery hints. The copy question was preserved verbatim. The adjacent `/raw` question tests another command. All failures are retained; elapsed time is task start to final completion or interruption, not just shell execution. Model and effort are recorded per trace. No live clipboard test was performed.

| Agent | Seconds | Lookup calls | <=20s final | New source route loaded |
| --- | ---: | ---: | --- | --- |
| /root/fresh_copy_question | 38.886 | 6 | False | False |
| /root/fresh_copy_retest | 25.225 | 3 | False | False |
| /root/copy_routing_test | 37.762 | 5 | False | False |
| /root/raw_routing_test | 20.177 | 2 | False | False |
| /root/copy_direct_source_test | 26.113 | 3 | False | False |
| /root/raw_direct_source_test | 24.162 | 4 | False | False |
| /root/copy_entrypoint_test | 16.742 | 2 | True | True |
| /root/raw_entrypoint_test | 19.586 | 2 | True | True |
| /root/copy_final_test | 30.640 | 3 | False | True |
| /root/raw_final_test | 16.416 | 2 | True | True |
| /root/copy_completion_test | 29.392 | 4 | False | True |
| /root/raw_completion_test | 12.046 | 1 | True | True |
| /root/copy_recursive_test_a | 27.749 | 5 | False | True |
| /root/copy_recursive_test_b | 21.128 | 2 | False | True |

Full tool-call inputs, timing anchors, token counters and retained native trace locations are in [tests.json](tests.json). Counters cover repeated requests, including cached input; they are not prices or unique context size.

The first two tests failed. Answer-specific documentation and installed-skill changes were tried and then removed. The retained changes are in project AGENTS.md: direct navigation to the cached CLI source with contextual recursive rg, command-consumer symbols instead of guessed files, and completed short lookup answers delivered as final returns rather than separate readiness messages. No model, config, review-gate, cached implementation or daemon change was made.

The last two identical-question copy tests used the retained project instructions with the original installed OpenAI Docs skill restored. Earlier regressions remain in the table. This is bounded evidence, not a universal latency guarantee, proof that every workflow failure is fixed, or an isolated causal experiment. Backend latency and agent variation remain uncontrolled.

## Current disposition, 10:45 UTC

All project AGENTS amendments described above were withdrawn after owner objection; those paragraphs describe historical state. The installed skill and project review skill remain unchanged. Two additional unchanged-guidance checks and the current unactivated decision comparison are retained in [comparison.md](comparison.md). The 20-second criterion is still unmet.
