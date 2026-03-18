# Session: Terminal Setup for Local Desktop (2026-03-18)

## Outcome

Jörn's local Ubuntu desktop runs CC in **Alacritty** with **tmux** for session persistence.

## Terminal Selection (tested all, with reasons for rejection)

| Terminal | Dealbreaker | Notes |
|----------|-------------|-------|
| **Alacritty** | None — selected | 0.3% CPU, Shift+Enter via keybinding, no tabs (uses OS windows) |
| Ghostty | 30%+ CPU while typing | Nice features but GPU rendering is too expensive with CC's TUI |
| gnome-terminal | No Shift+Enter, flickers/freezes on long chats | CPU renderer can't keep up with CC's rapid redraws |
| Kitty | Ugly non-native tab bar | Good otherwise (0.3% CPU, native Shift+Enter), but tabs rendered inside terminal look bad on light themes |
| WezTerm | Idle CPU drain, unmaintained (no stable release since 2024) | Not tested on this machine |

## Key discovery: CLAUDE_CONFIG_DIR

CC on the host cannot find `~/.claude/.claude.json` (which contains feature flags like `codeDiffFooterEnabled`) without `CLAUDE_CONFIG_DIR` being set. This causes the footer to be incomplete (no token count, no version). The container sets this via devcontainer config; the host needs it in `.bashrc`:

```bash
export CLAUDE_CONFIG_DIR="$HOME/.claude"
```

## Key discovery: Shift+Enter in tmux

No terminal's native Shift+Enter works inside tmux — tmux doesn't fully support the Kitty keyboard protocol. The fix is a terminal-level keybinding that sends the CSI u escape sequence `\x1b[13;2u` directly, which tmux passes through. In Alacritty:

```toml
[[keyboard.bindings]]
key = "Return"
mods = "Shift"
chars = "\u001b[13;2u"
```

Combined with tmux config `set -g extended-keys always`.

## Key discovery: dtach doesn't work with CC

CC's TUI eats the dtach detach character. Tested with multiple detach keys — none work. tmux works because its Ctrl+B prefix intercepts at a different layer.

## Key discovery: notify-send broken in snap confinement

`notify-send --app-name` is ignored when running inside snap-confined processes (shows "Unknown App"). Fix: use `gdbus` to call `org.freedesktop.Notifications.Notify` directly.

## Key discovery: PS1 colors

Ubuntu's `.bashrc` only enables colored PS1 for `xterm-color|*-256color`. Terminals with custom TERM values (xterm-ghostty, xterm-kitty) need to be added to the case statement.

## Files modified

### Host-only (not in repo):
- `~/.bashrc` — CLAUDE_CONFIG_DIR, PS1 colors (xterm-ghostty, xterm-kitty), `dc` function quieted
- `~/.tmux.conf` — CC fixes (extended-keys always, allow-passthrough, mode-style, set-titles, status off)
- `~/.claude/settings.json` — bypassPermissions, effortLevel high, notification hook, plugins
- `~/.claude/.claude.json` — copied from container (feature flags)
- `~/.claude/notify.sh` — gdbus notification script
- `~/.config/alacritty/alacritty.toml` — font, light theme, Shift+Enter keybinding, Ctrl+Shift+N
- `~/.config/ghostty/config` — kept for reference
- `~/.config/kitty/kitty.conf` — kept for reference

### In repo (need commit):
- `.devcontainer/post-create.sh` — tmux config with CC fixes, extended-keys always

## tmux config (identical on host, container, and post-create.sh)

```
set -g mouse on
set -g status off
set -g set-titles on
set -g set-titles-string "[#S] #{pane_title}"
set -g @scroll-down-exit-copy-mode off
set -g allow-passthrough on
set -sg escape-time 0
set -g extended-keys always
set -as terminal-features 'xterm*:extkeys'
set -as terminal-features 'xterm-kitty:extkeys'
set -g set-clipboard on
set -g history-limit 250000
set -g focus-events on
set -g default-terminal "tmux-256color"
set -ag terminal-overrides ",xterm-256color:RGB"
set -g mode-style "bg=#a8d1ff,fg=#000000"
```

## Typical launch commands

```bash
# Host, no tmux:
cd ~/workspaces/msc-math && claude -n msc-math

# Host, with tmux:
cd ~/workspaces/msc-math && tmux new -s msc 'claude -n msc-math'

# Container:
cd ~/workspaces/msc-math && dc
# then inside:
tmux new -s msc 'claude -n msc-math'
```
