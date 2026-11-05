## Contact loss predicts a constructed upper tail near HKO

Near HKO, the branches of the capacity formula give a more precise geometric signal than a broad correlation. Write $z=(a_0,\ldots,a_4,b_0,\ldots,b_4)\in\mathbb R^{20}$ for the dual normals of a pentagon product, $z_H$ for the fixed HKO reference and $V(z)$ for its four-dimensional volume. Let $\mathcal A_H$ contain the capacity-word branches $Q_w$ attaining the reference maximum. Define

$$D(\delta)=\max_{w\in\mathcal A_H}\left.\mathrm d\log\bigl(8VQ_w^2\bigr)\right|_{z_H}[\delta].$$

Each branch measures the logarithmic increase of inverse ratio; $D$ takes the largest first-order loss. The branch definitions and coordinate normalization are given in the [separate mathematical supplement](definitions-and-proof.tex). Fifty active words produce only twenty distinct linear forms. Their maximum is piecewise linear: tied contact orders exchange control as the geometry changes. For the direction $\delta a_0=a_0$, $\delta a_1=-a_1$, with all other changes zero,

$$D(t\delta)=\frac{2\sqrt5}{5}|t|.$$

Thus a deformation can lower the ratio linearly in either direction, producing a corner rather than a smooth quadratic peak.

Writing $\rho_H=(3+\sqrt5)/5$, the local law is

$$\log\frac{\operatorname{sys}(z_H+\delta)}{\rho_H}=-D(\delta)+O(\|\delta\|^2).$$

A strict gap excludes inactive words near the reference; Taylor expansion of the remaining finite analytic branches gives the uniform quadratic remainder. This uses the six-facet product reduction and supplies no numerical neighborhood radius. HKO local maximality was already established in a broader class; the content here is the explicit loss law.

Finite affine normalization makes a corresponding score $I$ invariant under translations, common dilation and product-preserving linear symplectic maps, with the same local law. The parameter-free prediction $\rho_H e^{-I}$ was frozen before capacity evaluation on forty-five distinct constructed perturbations. Among the twenty-three exact ratios above one, median relative error was $0.00656\%$ and worst error $0.12860\%$. Directions were reused across mechanisms and scales, so these are correlated local tests.

The same prediction failed on the historical sample of 1,024 random pentagon products: its upper-decile rank correlation was $-0.008$. An exact example also has ratio $0.83853$, above the reciprocal prediction $\rho_H/(1+I)=0.55906$, refuting that global upper bound and the smaller exponential one. The local mechanism therefore leaves broad upper-tail prediction and global maximality open.

[review: /root/contact_science_review]
