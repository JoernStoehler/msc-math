# Independent audit: six-generator Pfaffian inequality

I independently checked the proof in `six-vertex/theorem.md` line by line,
including the polytope reduction, Pfaffian signs, strip-area calculation, and
degenerate cases. I found no mathematical gap. Conditional on the stated
five-generator theorem, the argument proves the six-generator case.

The key identities check in the convention
`b=(W45,-W35,W34)`, `r=(W13,W14,W15)`, `s=(W23,W24,W25)`:

* expansion along index 6 gives the one linear constraint `P·y=0`; the ten
  new Pfaffians are the coordinates `W_ij y_k-W_ik y_j+W_jk y_i`;
* a vertex of `B_1^5 intersect P^perp` has support at most two (support one
  if `P=0`), and convexity of the new Pfaffian sum extends the vertex bound to
  every feasible direction;
* for a support-two direction `y=t(alpha,beta,0,0,0)`, the constraint is
  `b·h=0`, where `h=alpha s-beta r`; the two old Pfaffians involving one of
  indices 1,2 contribute `|b·g|`, and the remaining three contribute
  `||c b-g cross h||_1` with `g=r+s`;
* the coordinate formulas for those remaining Pfaffians are
  `(cW45-(W14W25-W15W24), -cW35-(W15W23-W13W25),
  cW34-(W13W24-W14W23))`, so the vector identity is exact up to the
  displayed harmless sign in the middle coordinate.

For the planar lemma, the zonogon has area `4||r wedge s||_1`, and lies in
the rectangle `|x|<=A, |y|<=B` and the strip `|-beta x+alpha y|<=H`.
The support constraints imply
`|beta A-alpha B|<=H<=beta A+alpha B`. For positive `alpha,beta`, the
rectangle-strip intersection removes exactly the two opposite corner
triangles, of total area `(beta A+alpha B-H)^2/(alpha beta)`. Subtracting its
area from `4H(A+B-H)` gives
`(beta A-alpha B+(alpha-beta)H)^2/(alpha beta)`, so the bound follows.
At an endpoint `alpha beta=0`, direct determinant expansion gives the claimed
bound. Zero coordinates and `h=0` in the dual section are covered by the
singleton-extreme-point case.

One harmless typo was already corrected in the live draft: from the triangle
bounds one gets `C <= B_m(R-H)+|c|B_m+H(R-H) <= (B_m+H)(|c|+R-H)`, not equality
between the two last expressions.

The proof's value is a genuine method extension of the proved five-generator
case, but a finite case alone has limited standalone impact. The useful
structural mechanism is the one-segment reduction: an extremal new pairing
direction has support at most two. A plausible extension to arbitrary old
generator counts would need to control such sparse directions once the zero
pairings leave a three-dimensional block; the six-dimensional argument uses
special `3 x 3` duality at exactly that point. No arbitrary-count extension
follows from this audit.
