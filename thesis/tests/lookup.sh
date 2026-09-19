#!/usr/bin/env bash
# Synthetic aux fixtures only; no TeX installation or thesis build required.
set -euo pipefail
test_dir=$(mktemp -d)
trap 'rm -rf "$test_dir"' EXIT
mkdir -p "$test_dir/thesis/build" "$test_dir/caller"
cp "$(dirname "$0")/../lookup.sh" "$test_dir/thesis/lookup.sh"
cd "$test_dir/caller"

expect_failure() {
    if bash "$test_dir/thesis/lookup.sh" "$@" >"$test_dir/out" 2>"$test_dir/err"; then
        echo "Unexpected success: lookup $*" >&2
        exit 1
    fi
    [[ ! -s "$test_dir/out" && -s "$test_dir/err" ]]
}

expect_failure
expect_failure ''
expect_failure one two
expect_failure missing-file
printf '%s\n' '\newlabel{lem:axb}{{99}{1}{}{lemma.99}{}}' \
    '\newlabel{lem:a.b}{{3.2}{7}{}{lemma.3.2}{}}' \
    '\newlabel{lem:a.b@cref}{{[lemma][2][]3.2}{[1][7][]7}}' \
    > "$test_dir/thesis/build/main.aux"
[[ $(bash "$test_dir/thesis/lookup.sh" 'lem:a.b') == '3.2' ]]
printf '%s\n' '\newlabel{-option}{{4}{7}{}{lemma.4}{}}' >> "$test_dir/thesis/build/main.aux"
[[ $(bash "$test_dir/thesis/lookup.sh" '-option') == '4' ]]
cd "$test_dir"
[[ $(bash thesis/lookup.sh 'lem:a.b') == '3.2' ]]
expect_failure 'lem:a'
expect_failure 'lem:.*'
echo 'lookup synthetic tests passed'
