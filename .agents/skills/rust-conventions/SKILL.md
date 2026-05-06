# Rust Conventions

## Instrumental Objectives

- Code is read more often than written, so we can invest time into making it easy to read and understand. Gpt-5.5 is the sole reader and writer of code, so we can perfectly match its baseline expectations and knowledge.
- We want code to correspond to the mathematics, especially the formalisations.
- We want code to verifiable i.e. there should be traces that help agents check whether code is bug-free and matches the mathematics.
- We want code to maintainable by many agents across the project lifetime.
- Editing is cheap, so maintaining two specialized variants of an algorithm as two almost-identical files is often better for project success than maintaining one file with an abstracted and thus more complex code path. The advantages of abstraction only come in when the variants are genuine instances of a larger class, not when they merely share some common aspects or intermediate results.
- Performance matters only in hotspots, so never even bother stating performance predictions as an argument without profiling first.
- Coding is cheap, so one can just try different things in parallel and evaluate them afterwards.

## Suggestions

- Use common, well-known rust style and idioms, which gpt-5.5 expects and already familiar with without needing explanations
- Use descriptive symbol names instead of short-to-type ones, except where you can match mathematical notation without becoming ambiguous.
- Follow standard best practices for readability, such as code comments about the "why", and flat/simple control flow, wherever the practices transfer from humans to gpt-5.5 as well. Avoid practices that try to minimize total text length that needs to be kept in context, agents have near-unbounded working memory.
- Follow standard best practices for verifiability, such as writing tests and including reasoning traces that help understand why code is correct, when not obvious. 
- Use doc-comments to state the input and output contracts of functions, and mention whether they are asserted in the function/in a sub-function, or just assumed and left to the caller.

