# Extreme-placement scalar inequality for the nuclear-ball route

Let the point-factor regular polygon have odd side count `n`, the partner have
odd side count `m>=n`, and set

\[
 b=\pi/n,\qquad v=\tan(\pi/(2m)),\qquad
 g_n=\frac{\tan b}{\cos(b/2)}.
\]

In the exceptional obtuse placement from the symmetric-partner lemma, two
vertices of the translated point triangle have Minkowski gauge `1/2` in the
width body `W_n`; the apex has gauge at most `g_n/2`. If `p` is the shortest
partner side divided by its perimeter `R_m=sec(\pi/m)`, the resulting tensor
gauge is at most

\[
 \frac{R_m}{2s_m}\{1-p(1-g_n)\}
 =\frac{1-p(1-g_n)}{1-v^2},
 \quad s_m=(1+\cos(\pi/m))^{-1}.
\]

Here `R_m/(2s_m)=1/(1-v^2)`. Thus it is enough that

\[
 p(1-g_n)\ge v^2. \tag{E}
\]

The maintained odd-angular-width note proves `p>=v(1-v)` for every partner
closure triangle (its Eq. (10)). Since `v>0`, this lower bound implies (E) if

\[
 g_n\le \frac{1-2v}{1-v}. \tag{E'}
\]
The right side decreases with `v`.

## Case `n>=7`

Since `m>=n>=7`, the maintained rational bounds give

\[
 v\le\tan(\pi/14)<23/100,
 \quad g_n\le g_7<\frac{49/100}{9/10}=\frac{49}{90}.
\]
The first strict inequality is Eq. (9) of `odd-regular-angular-width.md`; the
bound `tan(\pi/7)<49/100` is Eq. (rational) of `research.tex`, and
`cos(\pi/14)>9/10` follows already from `cos x>=1-x^2/2` and `pi<22/7`.
Therefore

\[
 \frac{1-2v}{1-v}>\frac{54}{77}>\frac{49}{90}>g_n,
\]
where `54/77>49/90` follows by cross multiplication. This proves (E) for all
`n>=7`, `m>=n`.

## Case `n=5`, `m>=9`

Now `v<=tan(\pi/18)<9/50`. For the last bound, use `pi<22/7`, hence
`x=pi/18<11/63`, and `tan x<=x/(1-x^2/2)`; the latter upper bound is less than
`9/50` by rational comparison. Hence

\[
 \frac{1-2v}{1-v}>\frac{32}{41}>\frac79.
\]
The exact pentagon value satisfies

\[
 g_5^2=\frac{\tan^2(\pi/5)}{\cos^2(\pi/10)}=14-6\sqrt5<\frac{49}{81},
\]
so `g_5<7/9`. For the strict rational comparison, `sqrt(5)>2233/1000`
(follow by squaring), which gives `14-6sqrt(5)<301/500<49/81`.
Thus (E') and then (E) hold for all `n=5`, odd `m>=9`.

## Boundary of this sufficient estimate

At `n=5,m=7`, the universal estimate `p>=v(1-v)` does not establish (E');
indeed `(1-2v)/(1-v)<g_5`. The independently certified full tensor result
for `5x7` already covers this pair. At `5x5`, this estimate also fails and
the exact pentagon tensor identities handle the known edge-contact case.

Consequently the extreme-placement branch is settled by this scalar argument
for all odd `m>=n>=7`, and for `n=5,m>=9`; the two omitted pairs are precisely
the separately established `5x5` and `5x7` cases. The least odd `m` covered by
the generic minimum-side inequality when `n=5` is `m=9`.

## Source pointers

- The placement and gauge decomposition: `topics/products/pentagon-arbitrary-partner/odd-regular-symmetric-partner.md`, “Obtuse templates: one extreme shape and smaller disks.”
- The minimum partner side bound: `topics/products/pentagon-affine-products/odd-regular-angular-width.md`, Eq. (10).
- Rational bounds for `tan(pi/14)` and `tan(pi/7)`: `odd-regular-angular-width.md`, Eq. (9), and `topics/products/pentagon-affine-products/research.tex`, Eq. (rational).
- The `5x5` identities and `5x7` certificate are documented in `topics/products/pentagon-affine-products/research.tex` and `tensor-certificate-review-20261022.md`.
