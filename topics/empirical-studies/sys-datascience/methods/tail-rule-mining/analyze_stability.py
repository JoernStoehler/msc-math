#!/usr/bin/env python3
# /// script
# requires-python = ">=3.12"
# dependencies = ["numpy", "scikit-learn"]
# ///

"""Expensive stability/permutation wrapper for tail-rule-mining."""

from __future__ import annotations

import analyze


def main() -> None:
    args = analyze.parse_args()
    if args.out_dir == analyze.HERE / "artifacts":
        args.out_dir = analyze.HERE / "artifacts-stability"
    if args.stability_runs == 0:
        args.stability_runs = 8
    if args.permutations == 0:
        args.permutations = 32
    analyze.run(args)


if __name__ == "__main__":
    main()
