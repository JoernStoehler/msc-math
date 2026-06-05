# Taxonomy: ESL-Style Elements Of Statistical Learning

Source intent:

- Frozen method-family snapshot adapted from the broad statistical-learning organization of *The Elements of Statistical Learning*.
- This file keeps the more classical/high-capability taxonomy even when some items are too heavy for the thesis surface.

## Linear Methods

- `ESL-LIN-REG` Linear regression
- `ESL-LIN-CLS` Linear classification
- `ESL-LIN-SHRINK` Shrinkage and regularized linear models

## Basis Expansion And Additive Modeling

- `ESL-BASIS-POLY` Polynomial and basis-expansion models
- `ESL-BASIS-SPLINE` Splines
- `ESL-BASIS-GAM` Additive models

## Kernel, Local, And Prototype Methods

- `ESL-LOCAL-KNN` K-nearest neighbors
- `ESL-LOCAL-KERNEL` Kernel smoothing / local regression
- `ESL-LOCAL-PROTOTYPE` Prototype-based methods

## Discriminant And Generative Classification

- `ESL-GEN-LDA` Linear discriminant analysis
- `ESL-GEN-QDA` Quadratic discriminant analysis
- `ESL-GEN-MIXTURE` Mixture-model classification

## Model Selection And Regularization

- `ESL-SEL-SUBSET` Subset selection
- `ESL-SEL-RIDGE` Ridge-type regularization
- `ESL-SEL-LASSO` Lasso-type regularization
- `ESL-SEL-DIMRED` Dimension-reduction-assisted regression

## Tree, Ensemble, And Committee Methods

- `ESL-TREE-CART` CART
- `ESL-TREE-BAG` Bagging
- `ESL-TREE-RF` Random forest
- `ESL-TREE-BOOST` Boosting

## Margin, Kernel, And Optimization-Based Classification

- `ESL-MARGIN-SVM` Support vector machines
- `ESL-MARGIN-KERNEL` Kernel methods

## Neural Networks

- `ESL-NN-MLP` Multilayer perceptrons

## Unsupervised Learning

- `ESL-UNSUP-PCA` Principal component analysis
- `ESL-UNSUP-FACTOR` Factor models / latent factors
- `ESL-UNSUP-KMEANS` K-means
- `ESL-UNSUP-HCLUST` Hierarchical clustering
- `ESL-UNSUP-MIXTURE` Gaussian mixtures / soft clustering

## Density Estimation And Flexible Distributions

- `ESL-DENS-KERNEL` Kernel density estimation
- `ESL-DENS-MIXTURE` Mixture density models

## Model Assessment, Averaging, And Stability

- `ESL-VAL-CV` Cross-validation
- `ESL-VAL-BOOT` Bootstrap
- `ESL-VAL-MODELAVG` Model averaging

## Likely Use In This Project

- Strongest overlap: linear methods, regularization, trees/ensembles, unsupervised learning, validation.
- Potential but dangerous overlap: density estimation, mixture models, flexible nonlinear fits that may overfit or fail transfer.
- Weak overlap: basis-expansion and additive modeling unless the search surface is reduced to a small explicit parameter family.
