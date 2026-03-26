# LICCA setup and workflow (Jörn reference)

Official docs (uni-augsburg login required): <https://collab.dvb.bayern/display/UniARZHPCKB>

Key pages:
- Connect to Cluster: <https://collab.dvb.bayern/display/UniARZHPCKB/Connect+to+Cluster>
- Slurm 101: <https://collab.dvb.bayern/spaces/UniARZHPCKB/pages/392035519/Slurm+101>
- Serial Job / Multithreaded Jobs / GPU Jobs: search the knowledge base
- FAQ: <https://collab.dvb.bayern/spaces/UniARZHPCKB/pages/392035481/FAQ+and+Troubleshooting>

For anything not listed below, check the official docs. Do NOT paraphrase them into this file.

## Verified on LICCA (2026-03-23)

```
User:    stoehljo
Home:    /hpc/gpfs2/home/u/stoehljo
Login:   ssh stoehljo@licca-li-01.rz.uni-augsburg.de
OS:      Ubuntu 24.04.3, kernel 6.8.0-88-generic
SLURM:   25.11.0
Rust:    1.94.0 (via rustup, not a system module)
Repo:    ~/msc-math (cloned from GitHub)
Target:  CARGO_TARGET_DIR=/hpc/gpfs2/scratch/u/stoehljo/cargo-target
```

Test job 9704889 completed on `test` partition in 2s.

## Partitions (from login banner)

```
partition       :  free/ max
 test           :   128/ 128
 epyc           :  4597/5120
 epyc-mem       :   440/ 512
 epyc-gpu-test  :    80/ 128
 epyc-gpu       :   844/ 896
 epyc-gpu-sxm   :   128/ 128
 xeon-gpu       :    64/  64
```

## Result retrieval

Two-hop scp via university SSH gateway (no VPN needed):

```bash
# From devcontainer:
scp -J stoehljo@xlogin.uni-augsburg.de \
    stoehljo@licca-li-01.rz.uni-augsburg.de:~/msc-math/experiments/<experiment>/results.jsonl \
    /workspaces/msc-math/experiments/<experiment>/
```

- `xlogin.uni-augsburg.de` is the official university SSH gateway
  (source: https://www.uni-augsburg.de/de/organisation/einrichtungen/rz/it-services/uaux/wlan/secure-shell/)
- Verified reachable from devcontainer (2026-03-23): connection accepted, password auth required
- Asks for RZ password twice (once for xlogin, once for LICCA)