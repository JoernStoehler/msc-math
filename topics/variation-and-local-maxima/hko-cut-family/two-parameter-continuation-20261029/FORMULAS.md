### Word 1: `(0, 1, 6, 7, 3, 12, 9)`

Status at shutdown: symbolic producer checks passed; no separate assembled audit of these two-parameter formulas was completed. This is a retained partial mechanism, not global capacity or optimality.

Free columns (zero-based): `[5, 6]`.

```text
c = 0
r = [(2/5*tau^3 - 2*tau)*b,
     (2/5*tau^3 - 2*tau)]
H = [
  [(6/5*tau^3 - 2*tau)*a*b + (4/5*tau^3 - 4*tau)*b^2 + (4/5*tau^3 - 4*tau)*b, (-2/5*tau^3 + 2*tau)*a + (2/5*tau^3 - 6*tau)*b + (2/5*tau^3 - 2*tau)],
  [(-2/5*tau^3 + 2*tau)*a + (2/5*tau^3 - 6*tau)*b + (2/5*tau^3 - 2*tau), (-8*tau)],
]
```

At the selected point the exact positive weights in word order are:

```text
(10296/687959*tau^2 + 107712/687959)
(17325/687959*tau^2 + 58509/687959)
(17325/687959*tau^2 + 160825/1375918)
(126175/5503672*tau^2 + 730275/5503672)
(-27621/687959*tau^2 + 152163/687959)
35500/687959
(-264775/5503672*tau^2 + 1299025/5503672)
```
Numerically, `Q=0.146373249541823` and minimum weight is `0.051601912`.

### Word 2: `(0, 6, 5, 2, 19, 11, 8, 4)`

Free columns (zero-based): `[5, 6, 7]`.

```text
c = ((9/32*tau^3 - 341/128*tau)*a*b)/((-21/2048*tau^2 + 199/2048)*a^2 + (9/64*tau^2 - 341/256)*a*b + (-987/2048*tau^2 + 9349/2048)*b^2 + (-55/1024*tau^2 + 521/1024)*a + (377/1024*tau^2 - 3571/1024)*b + (-9/128*tau^2 + 341/512))
r = [((9/256*tau^3 - 341/1024*tau)*a^3 + (1453/4096*tau^3 - 13763/4096*tau)*a^2*b + (-305/2048*tau^3 + 2889/2048*tau)*a*b^2 + (-987/4096*tau^3 + 9349/4096*tau)*b^3 + (377/4096*tau^3 - 3571/4096*tau)*a^2 + (-377/2048*tau^3 + 3571/2048*tau)*a*b + (377/4096*tau^3 - 3571/4096*tau)*b^2)/((55/16384*tau^2 - 521/16384)*a^2 + (-377/8192*tau^2 + 3571/8192)*a*b + (323/2048*tau^2 - 6119/4096)*b^2 + (9/512*tau^2 - 341/2048)*a + (-987/8192*tau^2 + 9349/8192)*b + (377/16384*tau^2 - 3571/16384)),
     ((-1597/4096*tau^3 + 15127/4096*tau)*a*b + (-305/2048*tau^3 + 2889/2048*tau)*b^2 + (233/4096*tau^3 - 2207/4096*tau)*b)/((55/16384*tau^2 - 521/16384)*a^2 + (-377/8192*tau^2 + 3571/8192)*a*b + (323/2048*tau^2 - 6119/4096)*b^2 + (9/512*tau^2 - 341/2048)*a + (-987/8192*tau^2 + 9349/8192)*b + (377/16384*tau^2 - 3571/16384)),
     ((55/1024*tau^3 - 521/1024*tau)*a^2 + (665/1024*tau^3 - 6299/1024*tau)*a*b + (9/64*tau^3 - 341/256*tau)*a)/((-21/4096*tau^2 + 199/4096)*a^2 + (9/128*tau^2 - 341/512)*a*b + (-987/4096*tau^2 + 9349/4096)*b^2 + (-55/2048*tau^2 + 521/2048)*a + (377/2048*tau^2 - 3571/2048)*b + (-9/256*tau^2 + 341/1024))]
H = [
  [((-233/1024*tau^3 + 2207/1024*tau)*a^4 + (-699/2048*tau^3 + 6621/2048*tau)*a^3*b + (2207/2048*tau^3 - 20905/2048*tau)*a^2*b^2 + (-305/512*tau^3 + 2889/512*tau)*a*b^3 + (-987/2048*tau^3 + 9349/2048*tau)*b^4 + (-269/512*tau^3 + 637/128*tau)*a^3 + (341/512*tau^3 - 1615/256*tau)*a^2*b + (341/512*tau^3 - 1615/256*tau)*a*b^2 + (-305/1024*tau^3 + 2889/1024*tau)*b^3 + (377/2048*tau^3 - 3571/2048*tau)*a^2 + (-987/2048*tau^3 + 9349/2048*tau)*a*b + (377/2048*tau^3 - 3571/2048*tau)*b^2)/((55/16384*tau^2 - 521/16384)*a^2 + (-377/8192*tau^2 + 3571/8192)*a*b + (323/2048*tau^2 - 6119/4096)*b^2 + (9/512*tau^2 - 341/2048)*a + (-987/8192*tau^2 + 9349/8192)*b + (377/16384*tau^2 - 3571/16384)), ((267/4096*tau^3 - 2529/4096*tau)*a^3 + (3715/4096*tau^3 - 35189/4096*tau)*a^2*b + (-1885/4096*tau^3 + 17855/4096*tau)*a*b^2 + (-2817/4096*tau^3 + 26683/4096*tau)*b^3 + (555/4096*tau^3 - 5257/4096*tau)*a^2 + (-377/4096*tau^3 + 3571/4096*tau)*a*b + (233/2048*tau^3 - 2207/2048*tau)*b^2 + (-377/4096*tau^3 + 3571/4096*tau)*a + (233/4096*tau^3 - 2207/4096*tau)*b)/((55/16384*tau^2 - 521/16384)*a^2 + (-377/8192*tau^2 + 3571/8192)*a*b + (323/2048*tau^2 - 6119/4096)*b^2 + (9/512*tau^2 - 341/2048)*a + (-987/8192*tau^2 + 9349/8192)*b + (377/16384*tau^2 - 3571/16384)), ((987/4096*tau^3 - 9349/4096*tau)*a^3 + (251/512*tau^3 - 4755/1024*tau)*a^2*b + (-305/512*tau^3 + 2889/512*tau)*a*b^2 + (305/2048*tau^3 - 2889/2048*tau)*b^3 + (305/512*tau^3 - 2889/512*tau)*a^2 + (-413/1024*tau^3 + 489/128*tau)*a*b + (-915/2048*tau^3 + 8667/2048*tau)*b^2 + (-377/4096*tau^3 + 3571/4096*tau)*a + (305/2048*tau^3 - 2889/2048*tau)*b)/((55/16384*tau^2 - 521/16384)*a^2 + (-377/8192*tau^2 + 3571/8192)*a*b + (323/2048*tau^2 - 6119/4096)*b^2 + (9/512*tau^2 - 341/2048)*a + (-987/8192*tau^2 + 9349/8192)*b + (377/16384*tau^2 - 3571/16384))],
  [((267/4096*tau^3 - 2529/4096*tau)*a^3 + (3715/4096*tau^3 - 35189/4096*tau)*a^2*b + (-1885/4096*tau^3 + 17855/4096*tau)*a*b^2 + (-2817/4096*tau^3 + 26683/4096*tau)*b^3 + (555/4096*tau^3 - 5257/4096*tau)*a^2 + (-377/4096*tau^3 + 3571/4096*tau)*a*b + (233/2048*tau^3 - 2207/2048*tau)*b^2 + (-377/4096*tau^3 + 3571/4096*tau)*a + (233/4096*tau^3 - 2207/4096*tau)*b)/((55/16384*tau^2 - 521/16384)*a^2 + (-377/8192*tau^2 + 3571/8192)*a*b + (323/2048*tau^2 - 6119/4096)*b^2 + (9/512*tau^2 - 341/2048)*a + (-987/8192*tau^2 + 9349/8192)*b + (377/16384*tau^2 - 3571/16384)), ((-843/1024*tau^3 + 7985/1024*tau)*a*b + (-341/512*tau^3 + 1615/256*tau)*b^2 + (521/2048*tau^3 - 4935/2048*tau)*b)/((55/16384*tau^2 - 521/16384)*a^2 + (-377/8192*tau^2 + 3571/8192)*a*b + (323/2048*tau^2 - 6119/4096)*b^2 + (9/512*tau^2 - 341/2048)*a + (-987/8192*tau^2 + 9349/8192)*b + (377/16384*tau^2 - 3571/16384)), ((-233/4096*tau^3 + 2207/4096*tau)*a^2 + (-4791/4096*tau^3 + 45381/4096*tau)*a*b + (987/2048*tau^3 - 9349/2048*tau)*b^2 + (-377/4096*tau^3 + 3571/4096*tau)*a + (-2351/4096*tau^3 + 22269/4096*tau)*b + (305/2048*tau^3 - 2889/2048*tau))/((55/16384*tau^2 - 521/16384)*a^2 + (-377/8192*tau^2 + 3571/8192)*a*b + (323/2048*tau^2 - 6119/4096)*b^2 + (9/512*tau^2 - 341/2048)*a + (-987/8192*tau^2 + 9349/8192)*b + (377/16384*tau^2 - 3571/16384))],
  [((-377/1024*tau^3 + 3571/1024*tau)*a^3 + (-767/1024*tau^3 + 7265/1024*tau)*a^2*b + (233/256*tau^3 - 2207/256*tau)*a*b^2 + (-233/1024*tau^3 + 2207/1024*tau)*b^3 + (-233/256*tau^3 + 2207/256*tau)*a^2 + (631/1024*tau^3 - 5977/1024*tau)*a*b + (699/1024*tau^3 - 6621/1024*tau)*b^2 + (9/64*tau^3 - 341/256*tau)*a + (-233/1024*tau^3 + 2207/1024*tau)*b)/((-21/4096*tau^2 + 199/4096)*a^2 + (9/128*tau^2 - 341/512)*a*b + (-987/4096*tau^2 + 9349/4096)*b^2 + (-55/2048*tau^2 + 521/2048)*a + (377/2048*tau^2 - 3571/2048)*b + (-9/256*tau^2 + 341/1024)), ((89/1024*tau^3 - 843/1024*tau)*a^2 + (915/512*tau^3 - 8667/512*tau)*a*b + (-377/512*tau^3 + 3571/512*tau)*b^2 + (9/64*tau^3 - 341/256*tau)*a + (449/512*tau^3 - 4253/512*tau)*b + (-233/1024*tau^3 + 2207/1024*tau))/((-21/4096*tau^2 + 199/4096)*a^2 + (9/128*tau^2 - 341/512)*a*b + (-987/4096*tau^2 + 9349/4096)*b^2 + (-55/2048*tau^2 + 521/2048)*a + (377/2048*tau^2 - 3571/2048)*b + (-9/256*tau^2 + 341/1024)), ((199/512*tau^3 - 1885/512*tau)*a^2 + (521/512*tau^3 - 4935/512*tau)*a*b + (521/512*tau^3 - 4935/512*tau)*a)/((-21/4096*tau^2 + 199/4096)*a^2 + (9/128*tau^2 - 341/512)*a*b + (-987/4096*tau^2 + 9349/4096)*b^2 + (-55/2048*tau^2 + 521/2048)*a + (377/2048*tau^2 - 3571/2048)*b + (-9/256*tau^2 + 341/1024))],
]
```

At the selected point the exact positive weights in word order are:

```text
(5711255949601/211484711619848*tau^2 + 21084374019229/211484711619848)
(4641360049887/211484711619848*tau^2 + 30296287895147/211484711619848)
(5711255949601/211484711619848*tau^2 + 21084374019229/211484711619848)
(-1238407319936/26435588952481*tau^2 + 12149998051887/52871177904962)
(-55669680000/26435588952481*tau^2 + 720212711000/26435588952481)
(-55669680000/26435588952481*tau^2 + 720212711000/26435588952481)
(-1238407319936/26435588952481*tau^2 + 12149998051887/52871177904962)
(4641360049887/211484711619848*tau^2 + 30296287895147/211484711619848)
```
Numerically, `Q=0.146386929474032` and minimum weight is `0.026132449`.

### Word 3: `(0, 4, 6, 15, 2, 19, 8)`

Free columns (zero-based): `[5, 6]`.

```text
c = 0
r = [(-1/5*tau^3 - tau)*a,
     (2/5*tau^3 - 2*tau)]
H = [
  [(-2/5*tau^3 - 2*tau)*a^2 + (2/5*tau^3 - 6*tau)*a*b + (-8/5*tau^3)*a, (4/5*tau^3 - 4*tau)*a + (-4/5*tau^3 + 4*tau)*b + (6/5*tau^3 - 2*tau)],
  [(4/5*tau^3 - 4*tau)*a + (-4/5*tau^3 + 4*tau)*b + (6/5*tau^3 - 2*tau), ((-23040*tau^3 + 12800*tau)*a + (157440*tau^3 - 83200*tau)*b + (-60160*tau^3 + 32000*tau))/((10400*tau^2 - 5600)*a)],
]
```

At the selected point the exact positive weights in word order are:

```text
1/40*tau^2 + 1/8
1/40*tau^2 + 1/8
(-198000/14439281*tau^2 + 2735964/14439281)
(176125/14439281*tau^2 + 1176125/14439281)
(-13564281/288785620*tau^2 + 12021131/57757124)
(176125/14439281*tau^2 + 1176125/14439281)
(-198000/14439281*tau^2 + 2735964/14439281)
```
Numerically, `Q=0.146385289309598` and minimum weight is `0.087891846`.

## Pairwise crossings

With branch numbering as above, specializing to `a=99/125` gives these roots in `0.2<b<0.4`:

| branches | root b | difference Q_i-Q_j at selected b |
|---|---:|---:|
| 1 and 2 | 0.2740433566747741? | -1.3679932209e-05 |
| 1 and 3 | 0.2827060182282709? | -1.20397677754e-05 |
| 2 and 3 | 0.2832213158782087? | +1.64016443366e-06 |

The selected point has `b=0.28315594803123156...`; branch 2 is slightly above branch 3 there (`Q_2-Q_3≈1.64016e-6`). Their crossing at fixed `a=99/125` is nearby, `b≈0.2832213158782087`. This explains the local switch in the finite-bank optimization as a candidate branch crossover. The first branch remains well below branch 2 at this point. This comparison does not establish a global capacity maximizer or a global optimum in `(a,b)`.

At the numerical finite-bank point reported as `(a,b)≈(0.792663,0.282252)`,
the exact branch formulas give `Q_2-Q_3≈2.29255e-9`. Fixing the decimal
`b=282252/10^6` exactly and isolating the real root of the branch-crossing
numerator in `0.7<a<0.9` gives
`a≈0.7926630625077051`. Thus the observed switch of the last two active words
has a precise algebraic explanation: the finite-bank optimizer lies essentially
on their `Q_2=Q_3` curve. The first word remains lower by about `1.20e-5` in
action at the displayed decimal point. This supports a local ridge mechanism
for the finite bank; it does not establish global capacity coverage or a true
maximizer of the systolic ratio.

## Reproduction

Run from the repository root with the existing Sage interpreter:

```sh
/workspaces/msc-math/.local-environments/miniforge/envs/sage/bin/python \
  topics/variation-and-local-maxima/hko-cut-family/two-parameter-continuation-20261029/branch-formulas.py \
  > /tmp/msc-math/viterbo-continuation-20261029/two-parameter/formulas/branches.json \
  2> /tmp/msc-math/viterbo-continuation-20261029/two-parameter/formulas/branches.log
```

The solve uses only the two- or three-dimensional reduced Hessians after eliminating five closure equations. The complete JSON output retains the exact branch matrices and specialized real-root isolations in the scratch directory.
