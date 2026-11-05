# Optional Chapter 8 insight: a conditional closure can still be a mixture

Selecting the factors jointly does not ensure that conditional extraction finds
the best compatible support pair. A fresh rational perturbation of the HKO
product has exact systolic ratio about $1.02888$, while the unchanged extraction
rule gives a capacity upper bound about $0.086\%$ too large. This example was
fixed before either its prediction or its exact capacity was computed.

The failure has a geometric explanation. The best extracted witness already
uses the globally correct closure in the first factor. Its second closure is
still a strict convex combination: a fraction $\lambda$ of the correct closure
and a fraction $1-\lambda$ of a worse one. Bilinearity gives the exact deficit

$$Q_* - Q_{\mathrm{selected}}
  =(1-\lambda)(Q_* - Q_{\mathrm{worse}})>0.$$

The opposite-factor response has therefore recovered the correct first support
without removing the second factor's mixture. A best response in the second
factor would finish this particular example, but this observation establishes
no universal repair. Both relaxed LP optimizers are unique, so changing a tie
between optimal couplings cannot repair the frozen rule here. Their strictly
positive basic entries and inactive reduced costs also make the example stable:
the strict failure persists under sufficiently small changes to the factors.

The small capacity error is compatible with useful approximate prediction.
It refutes exact recovery by this rule even on an open set of above-one products;
it does not refute one-percent accuracy, the earlier finite successes, or HKO
maximality. The constructed challenge stopped at its first exact failure after
ten of twelve predeclared capacities had been scored, so it does not estimate
a population failure rate.

Source and proof obligations: the accompanying README and separate scientific
packet bind the exact example, oracle, LP stability and mixture identity. This
is an optional research passage, not an addition to the selected manuscript.

[review: /root/intake_review]
