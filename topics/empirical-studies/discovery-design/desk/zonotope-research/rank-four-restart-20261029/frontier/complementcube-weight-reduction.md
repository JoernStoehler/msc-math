# Complement-cube eight-generator family: algebraic reduction and remaining gap

Retained 29 October 2026 as an unresolved proof target. This reduction has not
received a separate independent audit. It is not a theorem in the assembled note.
Further enumeration and optimization were stopped by the coordinator; this file
is a future discriminator, not an active assignment.

I did not prove the proposed inequality for all positive `a,b,c,d,e,f,d_i`, and
found no exact counterexample. The useful reduction is that arbitrary diagonal
weights introduce one global imbalance parameter beyond the previously
considered equal-scaling chamber. The final step is positivity of an explicit
quartic on `lambda>0`; I have not proved that positivity.

## Exact normalization and remaining parameter

Let `D=diag(d_i)` with `d_i>0`, `Z=diag(z_i)`, `z_i=sqrt(d_i)`, and
`A0=Z^{-1} A Z^{-1}`. Then

`diag(Z^{-1},Z^{-1}) W diag(Z^{-1},Z^{-1}) = [[A0,I],[-I,-A0^{-1}]]`.

Set `rho=Pf(A0)=p/(z1 z2 z3 z4)` and `Abar=A0/sqrt(rho)`, so
`Pf(Abar)=1` and `-Abar^{-1}=*Abar` in the fixed orientation. With
`lambda=rho^(1/4)`, the original W is

`diag(lambda Z,lambda^{-1} Z) [[Abar,I],[-I,*Abar]]
 diag(lambda Z,lambda^{-1} Z)`.

Thus the equal scaling case `lambda=1` is exactly `p^2=prod_i d_i`. It is a
proper subfamily of the requested arbitrary positive weights; the scalar
imbalance `lambda` must also be controlled.

## Laurent-quartic gap

For the Pfaffian-one core `Abar`, write

- `L_A = sum_{i<j} z_i z_j |Abar_ij|`,
- `L_B = sum_{i<j} z_i z_j |(*Abar)_ij|`,
- `S = sum_i z_i^2`, `q=prod_i z_i`, and `Q=sum_{i<j}z_i^2 z_j^2`,
- `E_A = sum_i z_i^2 sum_{j<k, i notin{j,k}} z_j z_k |Abar_jk|`, and `E_B`
  defined with `*Abar`,
- `R = sum_{i<j} z_i^2 z_j^2 Abar_ij (*Abar)_ij` (signed; for the prescribed
  positive matching chamber these products are nonnegative).

Counting four-sets by 4+0, 3+1, 2+2, 1+3, 0+4 gives exactly

`Phi = lambda^2 L_A + lambda^-2 L_B + S`,

`Psi = q(lambda^4+lambda^-4) + lambda^2 E_A + lambda^-2 E_B
       + L_A L_B + Q - 2R`.

Therefore

`Phi^2-4Psi = A lambda^4 + B lambda^-4 + C lambda^2
               + D lambda^-2 + E`,

where

`A=L_A^2-4q`, `B=L_B^2-4q`,
`C=2S L_A-4E_A`, `D=2S L_B-4E_B`,
`E=S^2-2L_A L_B-4Q+8R`.

The four-generator theorem applied to the weighted top and bottom blocks gives
`A,B>=0`. At `lambda=1`, proving the gap is the balanced-chamber problem.
For arbitrary `lambda`, the polynomial coefficients `C,D,E` can have negative
signs, so termwise AM-GM does not immediately finish the proof. For example,
with Pfaffian-one core entries `(a,b,c,d,-e,f)=(1,1,1,1/2,-1/4,1/4)` and
`z=(2,1,1,1)`, exact Laurent coefficients are `(A,B,C,D,E)=(41,17,30,-6,-41)`.
Thus `D` need not be nonnegative; requiring each coefficient to have the same
sign cannot prove the claim. Yet the quartic remains positive on this example.
Exploratory numerical scans are not retained as evidence in this packet and
are not needed for any of its proved component claims.

## Remaining proof obligation

For the original all-positive weights, it remains to prove
`A u^4 + C u^3 + E u^2 + D u + B >= 0` for every `u=lambda^2>0`, with the
coefficients linked by the Hodge-complement relations above, or to produce an
exact positive input and `u` making it negative. General quartic positivity
without those relations is false; the structure tying `A,B,C,D,E` to one
4-dimensional Pfaffian-one form is essential. No general SOS certificate was
found in this pass.
