# Taxonomy: Statistical Inference And Hypothesis Testing

Source intent:

- Frozen taxonomy snapshot for classical statistical inference adapted from broad statistics references such as Wasserman, *All of Statistics*, and standard mathematical-statistics chapter organization.
- This file is the external home for correlation checks, hypothesis tests, significance procedures, and resampling-based inferential methods.

## Correlation And Association

- `STAT-CORR-PEARSON` Pearson correlation
- `STAT-CORR-SPEARMAN` Spearman rank correlation
- `STAT-CORR-KENDALL` Kendall rank correlation
- `STAT-CORR-PARTIAL` Partial correlation

## Two-Sample And Multi-Sample Testing

- `STAT-TEST-T` t-tests and Welch-style mean comparison
- `STAT-TEST-MW` Mann--Whitney / rank-sum tests
- `STAT-TEST-ANOVA` ANOVA-style group comparison
- `STAT-TEST-KW` Kruskal--Wallis and related rank tests

## Categorical And Count Association

- `STAT-TEST-CHI2` Chi-square association tests
- `STAT-TEST-FISHER` Fisher exact tests

## Resampling And Randomization

- `STAT-RESAMP-PERM` Permutation tests
- `STAT-RESAMP-BOOT` Bootstrap confidence procedures

## Multiple Testing And Error Control

- `STAT-MULTI-FWER` Family-wise error control
- `STAT-MULTI-FDR` False-discovery-rate control

## Model Diagnostics And Uncertainty Summaries

- `STAT-DIAG-CI` Confidence intervals
- `STAT-DIAG-SE` Standard errors
- `STAT-DIAG-PVAL` p-values and test-statistic summaries

## Likely Use In This Project

- Strongest overlap: scalar heuristic checks, correlation claims, permutation-style sanity checks, and uncertainty summaries for exploratory patterns.
- Weak overlap: repeated large-scale multiple-testing machinery unless many feature families are screened formally.
