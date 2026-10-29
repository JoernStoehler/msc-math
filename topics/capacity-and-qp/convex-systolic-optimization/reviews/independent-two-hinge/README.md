# Independent literal check for Appendix C

The assembled mathematical reviewer independently implemented the four printed word families, KKT reconstruction, containment, quotient coordinates, action, and scalar signs using standard-library exact arithmetic. The checker does not import the submitted certificate. `check-two-hinge-literals.py` uses the arithmetic helper prefix in `check-weighted-witness.py` beside it.

Run from this directory:

```sh
python check-two-hinge-literals.py > /tmp/two-hinge-literals-fresh.out
cmp /tmp/two-hinge-literals-fresh.out two-hinge-literals.out
```

A fresh run from the retained copy matched the expected output. `two-hinge-independent-receipt.json` pins the original reviewed source hashes; its absolute scratch paths record provenance and are not execution dependencies. The analytic implication is reviewed separately in the assembled mathematical report.
