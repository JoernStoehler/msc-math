# Numerics Appendix Decision

The numerics-proofs appendix is deliberately omitted from `thesis/main.tex`.

The main numerics section needs only a concise trust-boundary account.  The
available developer-facing precision source, `formal/hk2017-qp-precision.tex`,
contains explicit gaps and unvalidated constants, and the current KKT route
demonstrations show that its stored residual estimate is not a total
exact-input error certificate.  Expanding that material into an appendix would
add apparent proof strength without supporting a retained thesis claim.

Reopen only if a named reader-facing theorem or quantitative claim requires a
proved bound that cannot be stated responsibly in the owning body section.
