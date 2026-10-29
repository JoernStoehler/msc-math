# Uniform augmented-minor bound near the HKO maximum

This note gives explicit pivot-weight and inverse bounds for the 20-row
family. Rows consist of two copies of each of the ten HKO normals. All words
covered by the pivot bound are simple in this twenty-row presentation, so
each base normal occurs at most twice. Write
\(N=\sqrt5-1=|n_i|=|m_i|\), and \(Q_*=\tau/5\).

## Exactly closed base weights

Let \(\beta\ge0\), \(\sum\beta=1\), \(\sum\beta_ra_r=0\), and let
\(s=\sum_{r\text{ q-factor}}\beta_r\). After normalizing each factor's
weights to sum one, the word objective is \(Q=s(1-s)B\), with \(B\)
bilinear in the two factor closure vectors. At the balanced split the same
vectors give a feasible repeated word with action \(B/4\le Q_*\), so
\(Q\le4Q_*s(1-s)\). If \(Q\ge Q_*/2\), then
\[
s,1-s\ge\frac{2-\sqrt2}{4}=:\lambda.
\]

Each normalized planar closure vector lies in the convex hull of the five
triangle vertices. Their barycentric coordinates are permutations of
\((\alpha,\alpha,\gamma)\), where
\[
\alpha=\frac{5-\sqrt5}{10},\qquad \gamma=\frac{\sqrt5}{5},
\]
so \(\alpha\) is the smallest coordinate. In any convex decomposition into
the five vertices, one coefficient is at least \(1/5\). Thus each factor
has a triangle of three distinct normals whose aggregate \(\beta\)-weights
are at least \(s\alpha/5\), hence at least \(\lambda\alpha/5\). Each
aggregate is split over at most two row occurrences, so one occurrence has
weight at least
\[
c_{\rm closed}=\frac{\lambda\alpha}{10}
=\frac{(2-\sqrt2)(5-\sqrt5)}{400}>0.004.
\]

Choose any two q normals from that triangle and all three p normals. These
five augmented columns \((a,1)\) are independent. Their determinant in
absolute value factors into a 2-by-2 determinant of two q normals and a
3-by-3 affine determinant of the p triangle. The former is
\(N^2|\sin(k\pi/5)|>1/2\) for distinct pentagon directions. The latter has
one of the two exact values
\((5\tau+\tau^3)/2\) or \(5\tau-\tau^3\), both greater than one. Hence
this base augmented minor is greater than \(1/2\).

## Explicit bound for nearly closed weights from the moving rows

Here is a robust extension that does not first repair closure. Suppose
\(\beta\ge0\), \(\sum\beta=1\), and the weights are feasible for the
perturbed rows \(a_r+tD_r\) with \(0\le t\le1/1000\). Let \(Q_t\) be their
perturbed action and suppose \(Q_t\ge Q_*/2\). Each \(|a_r|=N<5/4\),
and \(|D_r|\le\sqrt{1+(3N/5)^2}<5/4\), including retained rows with
\(D_r=0\). By bilinearity and
\(\sum_{r<s}\beta_r\beta_s\le1/2\),
\[
|Q_t-Q_0(\beta)|\le\frac{25}{16}t+\frac{25}{32}t^2<\frac1{60}<\frac{Q_*}{6}.
\]
Here \(Q_0(\beta)\) is the base-normal objective, without requiring base
closure. It follows that \(Q_0(\beta)>Q_*/3>1/30\).

For the normalized q and p occurrence-weight vectors \(x,y\),
\(Q_0=s(1-s)B(x,y)\). Since each cross pairing has magnitude at most
\(N^2<25/16\), \(|B|\le25/16\), and consequently
\[
s,1-s\ge s(1-s)>\frac8{375}.
\]

The base closure residual is bounded directly from perturbed feasibility,
including the retained old rows (whose \(D\) is zero):
\[
\left\|\sum_r\beta_ra_r\right\|
=t\left\|\sum_r\beta_rD_r\right\|
\le t\sum_r\beta_r\|D_r\|\le\frac54t<2t.
\]
Each factor component has norm at most this total residual. Therefore the
normalized factor distributions have mean residual at most
\[
\frac{2t}{8/375}\le\frac{3}{32}<\frac18
\quad(t\le1/1000).
\]

Passing to unit normals only decreases the normalized residual, since
\(N>1\). For either regular pentagon, take the three largest normalized occurrence
weights after aggregating by the five distinct normal directions. The two
largest directions have resultant norm at least \(\cos72^\circ>1/4\) in
unit-normal coordinates, while the remaining three have resultant norm at
most one. If their total mass is \(r\), the mean norm is at least
\((1-r)\cos72^\circ-r\). A mean norm below \(1/8\) forces \(r>1/16\);
since \(r\) is the sum of three coordinates each at most the third largest,
each of the top three coordinates exceeds \(1/48\). In original occurrence
weights, the selected three directions in either factor therefore have
aggregate mass greater than
\((8/375)/48=1/2250\). One of their at-most-two occurrences has weight
greater than
\[
c_{\rm near}=1/4500.
\]

Select two of these q directions and three p directions. Every distinct
q-pair determinant exceeds \(1/2\). For any three distinct pentagon
normals, the augmented planar determinant has one of the two values above;
both exceed one (use \(1/2<\tau<1\)). The resulting 5-by-5 augmented base
minor thus has determinant greater than \(1/2\), with every selected
occurrence weight at least \(1/4500\). The same choice works if the q/p
roles are exchanged.

For either base minor, each column norm is
\(\sqrt{1+N^2}=\sqrt{7-2\sqrt5}<8/5\). Each cofactor is less than
\((8/5)^4<6.56\), so the Frobenius norm of the adjugate is below \(32.8\).
Dividing by the determinant gives the uniform spectral bound
\[
\|M_0^{-1}\|_2<66.
\]
For the actual moving-row minor at \(t\le1/1000\), multilinearity bounds
the determinant change by
\(5(5/4)t(1.61)^4<1/8\). Thus its determinant exceeds \(3/8\), every
column norm is below \(1.61\), and its inverse norm is below
\(5(1.61)^4/(3/8)<94\).

This supplies a fixed positive pivot-weight threshold and inverse bound for
every normalized nonnegative feasible weight vector of the moving 20-row
bank with \(Q_t\ge Q_*/2\) and \(t\le1/1000\). It is deliberately
conservative. It does not by itself provide a uniform Taylor remainder for
the optimum over all branches; it is one input to such a remainder estimate.
