# Render check — 11 October 2026

Root rendered the repaired passage with installed headless Google Chrome and
inspected the resulting images. The first render exposed missing display
fractions and a duplicate title despite passing link/MathML-node checks.
The writer corrected Pandoc math extensions and title handling. A subsequent
narrow-view check exposed a combined equation too wide for the reading column;
the final source splits it into two displays without changing either formula.

Final repaired HTML SHA-256: `cdb30037816a2f2e72d21a554b57bda9ffdd204a40d4d2b9b9f6668dcbe3b952`.
Desktop and narrow captures show the actual file with the figure collapsed.
For the figure check, a scratch copy added a base URL pointing to the original
file directory and set the existing details element open; the embedded image
and caption were inspected. This checks rendering of the expanded contents,
not an automated browser click. No user browser/session was changed.

The final equation fractions, single heading, table and source links are visible
in the desktop capture; the first equations fit the narrow reading column. The
stratified figure displays with its historical same-sample caption. This does
not establish human reading acceptance or PDF pagination; the narrow capture
covers the top portion, while the desktop capture covers the passage.

Commands used `google-chrome --headless --no-sandbox --disable-gpu
--hide-scrollbars --user-data-dir=/tmp/msc-math/pilot-render-20261011/...`
with file URLs and requested windows1100×2700 and420×1600. Actual image
sizes below govern the observation; browser minimum dimensions may differ.

| Capture | Actual pixels | SHA-256 |
| --- | --- | --- |
| [broken-display-math.png](render-checks/broken-display-math.png) | 1100×2400 | `4f4256d9ce207081b52c9fd2d8a4c7248ffd8e6939590bb688e2db15c6203ba9` |
| [desktop.png](render-checks/desktop.png) | 1100×2700 | `58eb056d5eb54b28a5ab238d0ecf96dffd92d08ec6b6611fe9d2405d1ef38ab1` |
| [expanded-figure.png](render-checks/expanded-figure.png) | 1100×2000 | `18d542a0eca0d9679041e35e8bc021b03bcb3067003099fb798c5d9ee1652fb9` |
| [narrow.png](render-checks/narrow.png) | 420×1600 | `24b0f1c734d9340579dd7939b96fc2936200c1c68d8606825a8586018e078fb4` |
