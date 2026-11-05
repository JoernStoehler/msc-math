from pathlib import Path
import difflib, hashlib, json, re, subprocess
root=Path('/workspaces/msc-math-worktrees/thesis-2026-10-16')
scratch=Path('/tmp/msc-math/prose-local-search/20261017')
source=root/'thesis/chapters/08-data-science.tex'
text=source.read_text()
start=text.index('\\subsection{Competing branches in local search}')
end=text.index('\\subsection{A failed probe does not establish a local maximum}',start)
before=text[start:end]
table=before[before.index('\\begin{center}'):before.index('\\end{center}')+len('\\end{center}')]
after=r'''\subsection{Competing branches in local search}
\label{subsec:black-box-datascience-finite-budget-optimization}

The two branch-history implementations had the two largest median
best-so-far numerical systolic ratios in this comparison. Seven fixed
implementations shared the same 64 generic ten-facet starting bodies, a cap
of 128 full evaluations and a nominal one-second accumulated
evaluator-plus-optimizer budget. The comparison uses a heuristic
floating-point objective, rather than the certified capacity implementation
of Section~\ref{sec:numerics}. Its geometry and volume are floating-point
computations.
'''+table+r'''
The lower-envelope structure of the branch model in
Section~\ref{subsec:first-order-search-model} explains why increasing one
currently minimizing branch can expose another as the new minimum. The
finite-gap method therefore models branches near the current minimum.
The history methods also retain branches from earlier points; one predicts
additional branches from transitions. A numerically chosen branch window
is not guaranteed to contain every relevant branch.

CMA-ES~\cite{dsHansen2016} and coordinate polling supply population and
direct-search comparisons. These are rankings of the fixed implementations
under this allocation, not of optimally tuned algorithm families. The ranges
in the table show variation across starting bodies; all 448 best-so-far
ratios are below one. Appendix~\ref{app:ds-local-optimization} specifies the
available implementations, traces and budget interpretation.

'''
# A genuinely smaller alternative: replace only the vague model-entry sentence.
minimal=before.replace('The branch model in Section~\\ref{subsec:first-order-search-model}\nsupplies a more local source of proposed improvements. Its lower-envelope\nstructure explains why increasing one currently minimizing branch can\nexpose another as the new minimum.', 'Increasing one currently minimizing branch can expose another as the new\nminimum, as the lower-envelope model in\nSection~\\ref{subsec:first-order-search-model} explains.')
assert minimal!=before
for name,data in [('before.tex',before),('after.tex',after),('minimal-option.tex',minimal)]:
    (scratch/name).write_text(data)
updated=text[:start]+after+text[end:]
patch=''.join(difflib.unified_diff(text.splitlines(keepends=True),updated.splitlines(keepends=True),fromfile='a/thesis/chapters/08-data-science.tex',tofile='b/thesis/chapters/08-data-science.tex'))
(scratch/'proposed.patch').write_text(patch)
extract=lambda s,key:re.findall(r'\\'+key+r'\{([^}]+)\}',s)
assert table==after[after.index('\\begin{center}'):after.index('\\end{center}')+len('\\end{center}')]
for key in ['label','ref','cite']:
    assert sorted(extract(before,key))==sorted(extract(after,key)),key
assert text==source.read_text()
check=subprocess.run(['git','apply','--check',str(scratch/'proposed.patch')],cwd=root,capture_output=True,text=True)
assert check.returncode==0,check.stderr
receipt={
    'source':'thesis/chapters/08-data-science.tex',
    'source_sha256':hashlib.sha256(source.read_bytes()).hexdigest(),
    'start_line':text[:start].count('\n')+1,
    'end_line_exclusive':text[:end].count('\n')+1,
    'before_sha256':hashlib.sha256(before.encode()).hexdigest(),
    'after_sha256':hashlib.sha256(after.encode()).hexdigest(),
    'table_byte_identical':True,'label_ref_cite_multisets_equal':True,
    'patch_applies_to_current_source':True,
    'manuscript_unchanged_by_worker':True,
    'build_performed_by_worker':False,
    'human_judgment':None,
    'word_counts_whitespace_including_TeX':{'before':len(before.split()),'after':len(after.split())},
}
(scratch/'verification.json').write_text(json.dumps(receipt,indent=2)+'\n')
print(json.dumps(receipt,indent=2))
