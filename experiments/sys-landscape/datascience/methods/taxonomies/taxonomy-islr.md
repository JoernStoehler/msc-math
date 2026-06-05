# Taxonomy: ISLR-Style Statistical Learning

Source intent:

- Frozen method-family snapshot adapted from the chapter-level organization of *An Introduction to Statistical Learning*.
- This is a browse surface, not a claim surface and not a repo-state summary.

## Supervised Learning

- `ISLR-REG-LM` Linear regression
- `ISLR-REG-RIDGE` Ridge regression
- `ISLR-REG-LASSO` Lasso
- `ISLR-REG-ELASTIC` Elastic net
- `ISLR-CLS-LOGIT` Logistic regression
- `ISLR-CLS-LDA` Linear discriminant analysis
- `ISLR-CLS-QDA` Quadratic discriminant analysis
- `ISLR-CLS-NB` Naive Bayes
- `ISLR-CLS-KNN` K-nearest neighbors

## Model Assessment And Resampling

- `ISLR-VAL-HOLDOUT` Holdout validation
- `ISLR-VAL-CV` Cross-validation
- `ISLR-VAL-BOOT` Bootstrap

## Selection And Regularization

- `ISLR-SEL-BESTSUBSET` Best subset selection
- `ISLR-SEL-STEPWISE` Stepwise selection
- `ISLR-SEL-PCR` Principal components regression
- `ISLR-SEL-PLS` Partial least squares

## Nonlinear And Additive Models

- `ISLR-NLIN-SPLINE` Regression splines
- `ISLR-NLIN-GAM` Generalized additive models

## Tree And Ensemble Methods

- `ISLR-TREE-DECISION` Decision trees
- `ISLR-TREE-BAGGING` Bagging
- `ISLR-TREE-RF` Random forest
- `ISLR-TREE-BOOST` Boosting

## Margin And Kernel Methods

- `ISLR-SVM-LINEAR` Linear support vector machine
- `ISLR-SVM-KERNEL` Kernel support vector machine

## Unsupervised Learning

- `ISLR-UNSUP-PCA` Principal component analysis
- `ISLR-UNSUP-KMEANS` K-means clustering
- `ISLR-UNSUP-HCLUST` Hierarchical clustering

## Neural And Deep Learning

- `ISLR-DL-MLP` Feedforward neural network / multilayer perceptron

## Multiple Testing And Inference

- `ISLR-INF-PVALUE` Classical p-value testing
- `ISLR-INF-FDR` False-discovery-rate control

## Likely Use In This Project

- Strongest overlap: supervised learning, tree/ensemble methods, unsupervised learning, validation, regularization.
- Weak overlap: multiple testing.
- Weak or likely overkill overlap: deep learning, nonlinear spline/GAM surfaces unless there is a clear low-dimensional parameter family.
