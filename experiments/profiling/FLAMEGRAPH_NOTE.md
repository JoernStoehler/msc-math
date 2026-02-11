# Flamegraph Generation Note

## Status

Flamegraph generation via `cargo flamegraph` was attempted but encountered limitations in the devcontainer environment:

1. **Perf permissions**: The container has read-only `/proc/sys/kernel/perf_event_paranoid`, preventing perf from sampling user processes without elevated privileges
2. **Kernel version mismatch**: Initial perf installation targeted kernel 6.8.0-100, but the running kernel is 6.8.0-94
3. **Long profiling duration**: Even with sudo, the profiling run exceeded 10 minutes on simple test cases (hypercube + crosspolytope × 10 iterations)

## Attempted Solutions

- Installed correct kernel tools: `linux-tools-6.8.0-94-generic`
- Ran with sudo: `sudo -E cargo flamegraph`
- Adjusted profile settings: Added `[profile.release] debug = true` for symbol information

## Alternative Approach

Instead of runtime flamegraph profiling, hotspot analysis was performed via:
1. **Code inspection**: Manual analysis of HK2017 algorithm structure
2. **Complexity analysis**: O(m³) SVD, exponential enumeration
3. **Timing measurements**: Empirical timing data from `time-capacity` binary

See `PROFILE_REPORT.md` for detailed hotspot breakdown based on code analysis.

## Recommendation for Future Profiling

For production profiling with proper flamegraphs:
1. Run on bare-metal Linux system (not container) with unrestricted perf access
2. Use shorter profiling runs (1-2 iterations per polytope)
3. Profile individual functions in isolation (e.g., `solve_kkt` only)
4. Consider alternative profilers: `valgrind --tool=callgrind`, `perf record` + `perf report`
