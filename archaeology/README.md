# Archaeology

Files recovered from `msc-viterbo`, an abandoned predecessor repo. **Everything here is untrusted.** See `CLAUDE.md` for the full policy.

See `INDEX.md` for per-file metadata.

## Known-broken items

1. **HK2019 QP solver** — misses optima on 2D+ faces, returns plausible but wrong values
2. **Trivialization formula** — `tau_n(V) = (<V,Jn>, <V,Kn>)` not a bijection on 2-face tangent spaces
3. **Billiard orbit validation** — only checked even-indexed segments; pentagon returned 2.127 instead of 3.441
4. **Triangle x triangle discrepancy** — billiard returns 3.0, HK2017 returns 1.5; unresolved
5. **Normalization convention mismatch** — some files use `sys = c^2/(2*vol)`, others `sys = c^2/(4*vol)`
