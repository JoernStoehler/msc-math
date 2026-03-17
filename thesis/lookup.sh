#!/bin/bash
# Usage: ./lookup.sh lem:kkt
# Returns the rendered number for the given label from the compiled thesis.
grep "newlabel{$1}" build/main.aux | sed 's/.*{{\([^}]*\)}.*/\1/'
