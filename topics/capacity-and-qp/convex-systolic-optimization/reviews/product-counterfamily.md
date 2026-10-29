# Independent audit of the closed family

Auditor: `/root/truncated_conjecture_audit`, 28 October 2026. This supplements `audit.md`; all source-convention and completeness checks there extend immediately to arbitrary parameters 0<A,B<1, A+B>1. Reviewed `/tmp/msc-math/restart/product-extremizers/truncated-square-independent/family-cover-proof.md` and the author's added capacity/ratio claims.

**Verdict: approved.** The family gives a strict improvement over BMP's proposed optimizer on a nonempty open parameter region. The exact capacity equals 1. The ratio is strictly below 1 throughout its domain. These are algebraic conclusions, not finite numerical evidence.

Let a=1-A, b=1-B, g=A+B-1, d=B-a^2 and D=AB-g^2. We have

0<g<min(A,B), B>a>a^2, d>0, D>0,
AB+ag=d, d-Aa=g.

The dual polygon is simply Q=[0,1]^2 intersect {Ax+By<=1}. This is equivalent to the author's cut-leg parametrization, since r=g/B, s=g/A and r+s-rs=g/(AB). Its area is 1-g^2/(2AB). The normal templates remain T=conv{0,(A,0),(A,B)}, -T, and both coordinate unit segments.

## Containment and exact capacity

Define Dv=(A^2a/d,-A^2B/d), L=-(a/A)Dv, R=L+e1, C=(A,B). (Dv denotes a vertex; D above denotes the denominator AB-g^2.) The proposed body is the intersection

0 <= phi(x,y)=Bx+ay <= B,
U(x,y)=y-(g/A)x-a <= 0,
G(x,y)=y-(AB/g)(x-A) >= 0.

Its four vertices are L,Dv,R,C in counterclockwise order. For a short verification of all four intersections:

- L: phi=0, U=0, G=AB/g>0;
- Dv: phi=0, U=-1<0, G=0;
- R: phi=B, U=-g/A<0, G=0;
- C: phi=B, U=0, G=B>0.

Thus no purported vertex disappears at an interior parameter value. The strip-crossing inequalities give precisely this bounded trapezoid. The source packet's displayed phi,U,G values for all four template placements are correct. In particular, for the translated -T, t=(A+aAg/d, aB(A+g)/d), the last value G(t-Ae1-Be2)=AB(1-B)/g is strictly positive. There is no missing parameter restriction in that containment argument.

The x-coordinates obey

L_x<0<(Dv)_x<A<R_x=L_x+1,
A-(Dv)_x=Ag/d,
R_x-A=ag/d.

Hence the x-width is exactly 1. The horizontal unit segment [L,R] fits; a larger horizontal segment cannot fit. The full normal-template theorem and scaling therefore give c_EHZ(K x_L Q)=1 exactly.

## Area and ratio

The horizontal diagonal LR has length 1. The lower altitude is AB/d, because

L_y-(Dv)_y=(aAB+A^2B)/d=AB/d.

The upper altitude is B-aAB/d=Bg/d. Thus

F=area(K)=B(A+g)/(2d).

Consequently its exact ratio is

rho=2Ad/[(A+g)(2AB-g^2)].

The denominator exceeds the numerator by (1-B)g^2>0, proving rho<1 on the entire open parameter domain. In fact this is also consistent with BMP's quadrilateral-factor theorem, since the new partner is itself a trapezoid; no contradiction to that established theorem arises.

## Independent general formula for BMP's K0

The comparator need not rely solely on BMP's printed ratio formula. Solving the Figure 11 incidences as in `audit.md` gives its four vertices

L0=(-Aa^2/D, aBb/D),
D0=(A^2a/D,-ABb/D),
R0=(A+Aag/D, ABa/D),
C=(A,B).

Their shoelace area is M=AB(1-g^2)/(2D). They give the template placement described by the figure throughout the full parameter domain. The horizontal chord through L0 has length 1. The side slopes dx/dy, cyclically from L0, are

-Aa/(Bb), g/B, -Aa/(Bb), A/g.

The vertical levels are (D0)_y<0<(L0)_y<(R0)_y<B. Hence the horizontal chord-length derivative is positive below (L0)_y and negative above it, because g/B-A/g=-D/(Bg)<0. Its maximal horizontal chord is exactly 1, so c(K0 x_L Q)=1 independently of the printed capacity formula.

Finally,

M-F = B a g^2 (g-A^2) / [2dD].

Every factor except g-A^2 is strictly positive. The new family strictly beats K0 exactly when g>A^2, equivalently B>1-A+A^2. This is a nonempty open interval of B for every 0<A<1, since 1-A+A^2<1.

## Exact example

For A=11/20, B=4/5, the new partner has vertices

L=(-891/4780,396/1195),
Dv=(1089/4780,-484/1195),
R=(3889/4780,396/1195),
C=(11/20,4/5).

Its area is 144/239 and its exact ratio is 2629/2727. Compared with BMP's K0, the area improvement is 8379/1517650 and the ratio improvement is 931/106353. Both are strictly positive. This supersedes the less efficient hexagonal rational witness as the strongest audited construction.

Limits: no global optimality of this new family is proved, and no Viterbo violation is claimed. The construction gives a lower bound on the true best systolic ratio and disproves the proposed maximizer. All normal-cover/capacity implications rely on the cited BMP theorem, whose applicability and conventions have been checked.
