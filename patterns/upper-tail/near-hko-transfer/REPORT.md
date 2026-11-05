# Conditional rounding misses capacity on an open set above ratio one

17 October 2026. The unchanged conditional-coupling LP followed by one row/column opposite response fails exact capacity recovery on a fresh rational pentagon product with exact systolic ratio greater than one. Its two LP optima are unique and nondegenerate, so this is a failure of the mathematical extraction rule on an open neighborhood of planar product inputs, rather than a choice among tied LP optimizers. The capacity error at the displayed body is **0.0859128%**, below the separate 1% practical-error threshold. This settles an exactness obstruction; it supplies no lower bound of 1% on practical error and no global systolic bound.

The best extracted candidate already finds the correct A closure. Its fixed B conditional mixes the correct B vertex with a losing one, and the unchanged rule stops after responding in A. The exact mixture identifies where the objective is lost.

## Frozen challenge and separation from truth

The [plan](PLAN.md) selected a local challenge around the exact HKO normal presentation in the preceding [conditional-coupling packet](../conditional-coupling/hko-exact.json). It replaces every occurrence of $\sqrt5$ by $2236068/1000000$, then adds independent row-coordinate perturbations at scales $1/1000$ and $1/100$. Six SHA256-derived directions produce twelve bodies, in seed-major/scale-minor order. [freeze.py](freeze.py) constructs all twelve without importing scoring code or historical capacity labels. Their membership never depends on the resulting ratios.

[sample.json](sample.json), the plan, and unchanged imported pilot, rounding and shadow sources were hashed and sent to the coordinator before scoring. [frozen-manifest.json](frozen-manifest.json) records these bindings. The sample SHA256 is `83f89a57fd12bf6a072c8d551edfdbba83e26f30e101521d6708c075f3b2d3a0`.

[analyze.py](analyze.py) uses two separate commands. `predict` runs the unchanged `pilot`, `round_coupling`, and two signed `shadow_word` constructions, preserving the original exact reconstruction and first-max tie conventions. Both LP certificates and all twenty one-response candidates per body are retained. All twelve predictions were written to [predictions.json](predictions.json), SHA256 `466172b4064e7a23c8325e1fc8f54af6227103c45ed7692003458b0c69566702`, and sent to the coordinator before the `oracle` command began. No reconstruction failed.

Only then did the exact oracle enumerate all five A closure vertices, all five B closure vertices, and both shadow words. The imported [six-facet reduction](../../../topics/products/product-qp-six-facet-reduction.tex) and [two-shadow theorem](../mixed-area-order/REPORT.md) provide completeness. The oracle computes exact volume from the planar halfplane intersections, hence exact $\operatorname{sys}=c^2/(2\operatorname{area}(A)\operatorname{area}(B))$. None of these oracle values enters selection. The planned first-strict-miss stop occurred at the tenth body; both seed-five bodies remain predicted but **unscored**. No extra control was substituted or scanned.

## Outcome and exact obstruction

[results.json](results.json) retains every scored input, LP, candidate, oracle objective and winning pair. All ten scored products have exact ratio greater than one, with displayed ratios between $1.022068\ldots$ and $1.045812\ldots$. The first nine have exact recovery. These are constructed local cases and a stopped prefix, not a population success-rate estimate.

The first miss is seed four, scale $1/100$, named `near_hko_s4_e100`:

| Quantity | Value (rounded for display) |
|---|---:|
| True capacity | $4.942255298349177$ |
| Rounded capacity upper bound | $4.946501329713695$ |
| True systolic ratio | $1.028881001955570$ |
| $C_{\rm round}/C_{\rm true}-1$ | $0.000859128294310502$ |
| Candidate count | $20$ |

The exact winning objective is

$$Q_* = \frac{235970890203063841411016855771}{2332456764724528410000000000000}.$$

Each shadow has the **same unique** maximizing vertex pair, with A support $\{0,2,3\}$ and B support $\{1,2,4\}$. Uniqueness here concerns the closure-vertex pair for each of these two words, not the complete family of minimizing billiard trajectories. In both retained LP optima, neither winning factor vertex appears as a normalized row/column conditional. The pure conditional vertices in each factor instead have indices $\{0,2,4\}$ in the packet's closure enumeration; the missing winners have indices A $1$ and B $3$. Other conditionals lie strictly inside closure edges.

For a fixed word, a bilinear objective on $C_A\times C_B$ attains its maximum at a vertex pair. If that maximizing pair is unique, any maximizing arbitrary pair must equal it: expand both factors as convex combinations of vertices, and every positive pair coefficient must have maximal objective. Thus any fixed conditional different from the corresponding unique winning factor makes exact recovery impossible by one opposite response. This applies to all retained row and column candidates in both words. The actual true A vertex does appear **as an opposite response**, so absence among conditionals does not mean that the response mechanism cannot find either winning factor individually.

The best candidate is positive-shadow row zero. Its response is exactly $\alpha_*$, the true A vertex. Its fixed B conditional is

$$\widehat\gamma=\lambda\gamma_*+(1-\lambda)\gamma_{\rm bad},\qquad
\lambda=\frac{288743526589183882173667635863415637485770895105}{468709669366553352197129050722759042012776652914}
=0.616039\ldots,$$

where $\gamma_*$ has support $\{1,2,4\}$ and $\gamma_{\rm bad}$ has support $\{1,3,4\}$. Both coefficients are strictly positive. With $Q_{\rm bad}=Q(\alpha_*,\gamma_{\rm bad})=0.1009422135\ldots<Q_*$, the exact loss identity is

$$Q_*-Q_{\rm round}=(1-\lambda)(Q_*-Q_{\rm bad})>0.$$

[obstruction.json](obstruction.json) retains the full rational values and exact weights. The oracle also proves $\gamma_*$ is the unique B best response to the already extracted $\alpha_*$. Completing that response would recover capacity on this same witness. That observation explains the unremoved mixture; it is not a tested replacement method or a guarantee for a repeated-response policy. No patched selector or new cohort was run.

## Why the failure persists locally

The independent checker verifies the following exact properties of the miss:

| LP property | Positive shadow | Reversed negative shadow |
|---|---:|---:|
| Equality rank / positive basis columns | $17/17$ | $17/17$ |
| Positive primal minimum (display approximation) | $0.02071909$ | $0.02071909$ |
| Smallest nonbasic dual slack | $0.00008227488$ | $0.003430836$ |
| Nonbasic columns | $8$, all strict | $8$, all strict |

Each selected $17\times17$ basis determinant is nonzero; exact determinants, minima and basis indices are in `obstruction.json`. Dual slack is zero on the positive columns. Strict positive nonbasic slack forces every optimal coupling to use only the basis, whose invertibility then makes it unique. The two retained LP solutions therefore require no favorable optimizer tie choice.

For rank-two five-row factors, homogeneous conditional-closure matrices have dimension $(5-2)(5-2)=9$: their columns and rows lie in the respective three-dimensional normal kernels. Normalization adds one independent equality because a normalized positive closure coupling exists. The full equality rank is consequently $25-9+1=17$ nearby. The selected basis remains invertible; its primal and dual solutions vary continuously, with positive basic entries and strictly positive nonbasic slacks persisting. Thus the same bases remain the unique mathematical LP optima in a neighborhood.

The checker also proves strict angular-ray determinants and no angular-cut crossings, so both words persist locally. Every pair determinant and every triple affine determinant within either normal factor is nonzero. All primal facet rows are genuine, and the closure supports, bounded polygons and their area formulas persist in the same geometry chamber. The finite response optimum is the maximum of continuously varying candidate values; ties among responses do not affect its value. The exact true capacity likewise varies continuously as the maximum over the fixed finite closure-pair family. Since $Q_*-Q_{\rm round}>0$ and $\operatorname{sys}-1>0$ strictly at this body, both inequalities hold in some open neighborhood of the twenty planar normal coordinates.

This is an existence argument for an open set of failure of the **exact LP plus one-response rule** above ratio one. It gives no numerical neighborhood radius, no assertion for nonproduct four-polytopes, and no promise that floating-point LP solving and certificate reconstruction succeed on every nearby machine input. It also does not imply that 1% errors occur arbitrarily nearby.

## Verification and reproduction

[verify.py](verify.py) imports no producer, numerical dependency or optimizer. It independently reconstructs the twelve frozen geometries; uses rational Gaussian elimination for all closure vertices; checks genuine facets and boundedness; verifies all twenty-four LP primal/dual certificates; replays all 240 conditional responses with exact tie convention; evaluates all 500 authorized pair-word oracle objectives on the ten scored products by the actual four-dimensional HK sum; and verifies volumes, ratios, the unique winner, mixture identity, basis invertibility and local strictness. [verification.json](verification.json) records PASS. The coordinator separately checked the ten capacities by a support-prefix oracle and independently checked stability.

From the patterns checkout, verify the retained packet with standard Python:

```sh
python3 -B patterns/upper-tail/near-hko-transfer/verify.py \
  --repo "$PWD" --directory patterns/upper-tail/near-hko-transfer
```

A fresh replay writes only to new scratch. Use an existing Python environment containing NumPy and SciPy for `predict`; the retained execution used Python 3.13.3, NumPy 2.5.3 and SciPy 1.18.1 from an already cached environment, without installation or network access:

```sh
RUN=/tmp/msc-math/near-hko-transfer/replay
mkdir -p "$RUN"
cp patterns/upper-tail/near-hko-transfer/{PLAN.md,freeze.py,analyze.py,verify.py} "$RUN/"
python3 -B "$RUN/freeze.py" --repo "$PWD" --directory "$RUN"
OPENBLAS_NUM_THREADS=1 OMP_NUM_THREADS=1 "$PYTHON_WITH_SCIPY" \
  -B "$RUN/analyze.py" predict --repo "$PWD" --directory "$RUN"
python3 -B "$RUN/analyze.py" oracle --repo "$PWD" --directory "$RUN"
python3 -B "$RUN/verify.py" --repo "$PWD" --directory "$RUN"
```

Set `PYTHON_WITH_SCIPY` to the path of that existing interpreter. Fresh numerical LP implementations may choose different exact optimizer certificates on the other bodies; optimizer dependence is retained, not hidden by choosing an alternative optimum. The specific failure's two exact LP optima are unique. Predictions refuse replacement if `predictions.json` already exists. Verification writes `verification.json` and `obstruction.json` in the chosen packet directory. The source manifest binds imported code and proof dependencies; `manifest.json` binds retained packet bytes.

Scientific prediction and oracle CPU time totaled $1.65$ seconds, below the 180-second allowance; independent checking took approximately one second. All new files began in `/tmp/msc-math/near-hko-transfer/20261017T1719Z/` and were deliberately retained with the pattern owner. There was no network, installation, broad producer, fitting, tuning, manuscript edit or extra body evaluation. The scientific stop is this strict support-selection obstruction. Inspected sources were the local repository/research/product guides, collaboration rationale, exact HKO source, unchanged conditional-coupling/rounding/shadow implementations and reports, and linked sparse/two-shadow proof sources. No external-literature or exhaustive-novelty claim is made.

[review: /root/intake_review]
