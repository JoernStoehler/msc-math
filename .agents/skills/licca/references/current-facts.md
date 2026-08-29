# Current LICCA Facts

Confirmed by Jörn on 2026-07-11:

- Normal checkout: `"$HOME/msc-math"`.
- Rust target cache: `/hpc/gpfs2/scratch/u/stoehljo/cargo-target`.
- System `python3` is available (3.12.3 when last version-checked); `uv` is not.
- Login uses the Augsburg gateway route below.

```bash
ssh -t \
  -o IdentitiesOnly=yes \
  -o PubkeyAuthentication=no \
  -o PreferredAuthentications=password,keyboard-interactive \
  -o 'ProxyCommand=ssh -o IdentitiesOnly=yes -o PubkeyAuthentication=no -o PreferredAuthentications=password,keyboard-interactive -W %h:%p stoehljo@xlogin.uni-augsburg.de' \
  stoehljo@licca-li-01.rz.uni-augsburg.de
```

The no-pubkey options avoid too many authentication failures from offered SSH
keys. The route prompts for the password at the gateway and again at LICCA.

For `scp`, reuse the same options and `ProxyCommand`. Match the local destination
to the shell running `scp`. The host and current Docker Sandbox checkout use
`/workspaces/msc-math/`; verify the active checkout before retrieval.

Example retrieval from the host, or from Docker Sandbox after verifying LICCA
reachability:

```bash
scp \
  -o IdentitiesOnly=yes \
  -o PubkeyAuthentication=no \
  -o PreferredAuthentications=password,keyboard-interactive \
  -o 'ProxyCommand=ssh -o IdentitiesOnly=yes -o PubkeyAuthentication=no -o PreferredAuthentications=password,keyboard-interactive -W %h:%p stoehljo@xlogin.uni-augsburg.de' \
  stoehljo@licca-li-01.rz.uni-augsburg.de:~/artifact.tgz \
  /workspaces/msc-math/.worktrees/<worktree>/
```
