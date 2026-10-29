# Primary-source locator for geometric simple minimizers

29 October 2026. Reviewer `/root/hko_ascent_math_audit`.

The precise inspected source is Pazit Haim-Kislev, *On the symplectic size of
convex polytopes*, [arXiv:1712.03494v3](https://arxiv.org/pdf/1712.03494v3),
Theorem 1.5, printed/PDF page 3. It asserts existence of a minimum-action
closed characteristic with finitely many constant velocities, each a positive
multiple of a facet-normal characteristic direction, with each such direction
used on at most one interval. This is the geometric premise used by the
moderate-parameter graph search. It is a theorem number in the primary paper,
not a number transferred from the MSc thesis.

The [arXiv record](https://arxiv.org/abs/1712.03494) records first submission
on 10 December 2017 and v3 revision on 4 January 2019; the PDF title page is
dated 7 January 2019. The [publisher record](https://link.springer.com/article/10.1007/s00039-019-00486-4)
confirms *Geometric and Functional Analysis* 29, 440–463 (2019), DOI
10.1007/s00039-019-00486-4. Publisher full text is subscription-only in the
accessed page, so the theorem text and locator were checked in the author's
arXiv v3, while publication metadata was checked at Springer.

Both local bibliography entries named `HK2017` already contain the journal
publication year 2019. The identifier `HK2017` is only a BibTeX key reflecting
the first preprint year. For maximally precise printed provenance the note
can specify `arXiv:1712.03494v3 (Theorem 1.5)` without changing that key.

## Translation to the certificate conventions

The paper uses unit outward normals n_i. The certificate uses support-
normalized rows a_i=n_i/h_i. Multiplication by the positive scalar h_i
does not change which facet direction is used. A segment whose velocity is
a positive multiple of Jn_i lies on its associated facet: the characteristic
inclusion places n_i in the normal cone at the segment point, which means
that point maximizes n_i.x over the body.

Reparametrize such a segment to velocity 2Ja_i. Positivity of its original
speed and h_i>0 gives a positive new dwell time. This preserves the geometric
curve, its orientation and its action. Normalizing the total new period
gives positive beta weights with sum one and normal closure. Each direction
still occurs once. These are precisely the active geometric words used in
the transition proof; no assertion that every algebraic maximizing word has
a unique geometric placement is necessary.

The characteristic matrix in the moderate certificate is the negative of its
displayed pairing matrix J, consistently with
omega(a,b)=a^T J b. This makes the transition derivative
a_j.(-J a_i)=omega(a_i,a_j), as stated in the coverage proof.
