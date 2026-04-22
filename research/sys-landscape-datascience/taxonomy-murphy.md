# Taxonomy: Murphy-Style Probabilistic Machine Learning

Source intent:

- Frozen method-family snapshot adapted from a broad probabilistic machine-learning taxonomy in the style of Kevin Murphy.
- This tree is useful because it naturally includes latent-variable, density, anomaly, and sequential families that are awkward in narrower regression/classification taxonomies.

## Supervised Prediction

- `MUR-SUP-LINREG` Probabilistic linear regression
- `MUR-SUP-LOGREG` Probabilistic logistic classification
- `MUR-SUP-NB` Naive Bayes and related simple generative classifiers
- `MUR-SUP-TREE` Probabilistic tree and ensemble predictors

## Probabilistic Latent-Variable Models

- `MUR-LATENT-PCA` Probabilistic PCA
- `MUR-LATENT-FA` Factor analysis
- `MUR-LATENT-MIXTURE` Mixture models
- `MUR-LATENT-AUTOENC` Latent neural representations / autoencoders

## Density Estimation And Generative Modeling

- `MUR-DENS-KDE` Kernel density estimation
- `MUR-DENS-MIXTURE` Mixture-density models
- `MUR-DENS-FLOW` Normalizing flows and similar expressive density models

## Outlier And Novelty Detection

- `MUR-OUTLIER-DENSITY` Density-based anomaly detection
- `MUR-OUTLIER-ONECLASS` One-class classification
- `MUR-OUTLIER-ISOLATION` Isolation-style anomaly methods

## Bayesian And Hierarchical Modeling

- `MUR-BAYES-HIER` Hierarchical Bayes / partial pooling
- `MUR-BAYES-GP` Gaussian processes
- `MUR-BAYES-POSTERIOR` Posterior predictive uncertainty methods

## Sequential And State-Space Models

- `MUR-SEQ-HMM` Hidden Markov models
- `MUR-SEQ-KALMAN` Kalman and linear state-space models
- `MUR-SEQ-SWITCH` Switching state-space models
- `MUR-SEQ-RNN` Learned recurrent sequence models

## Causal / Decision / Optimization-Adjacent Families

- `MUR-DECISION-BANDIT` Bandits
- `MUR-DECISION-BO` Bayesian optimization
- `MUR-DECISION-RL` Reinforcement-learning-style decision methods

## Inference And Approximation

- `MUR-INF-MCMC` Markov chain Monte Carlo
- `MUR-INF-VI` Variational inference
- `MUR-INF-LAPLACE` Laplace and local approximations

## Likely Use In This Project

- Strongest overlap: latent-variable models, outlier detection, sequential/state-space models, Bayesian optimization.
- Likely high-cost or overkill overlap: expressive density models, deep latent models, approximate-inference-heavy Bayesian stacks.
- This taxonomy is mainly useful as a completeness check for "did we omit whole probabilistic families?" rather than as the first-pass thesis method surface.
