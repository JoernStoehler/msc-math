# latexmk configuration for thesis
# Usage: cd thesis/ && latexmk

$pdf_mode = 1;
$pdflatex = 'pdflatex -shell-escape -synctex=1 -file-line-error -interaction=nonstopmode %O %S';
@default_files = ('main.tex');

$out_dir = 'build';
$aux_dir = 'build';

$recorder = 1;
$silent = 1;

@clean_ext = ('synctex.gz');
