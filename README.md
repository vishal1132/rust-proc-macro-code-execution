# Arbitrary code execution during compilation
To see it, just open this project in VSCode and trust the project, install rust-analyzer, if it's not already there. You will see a hacked.txt created, cat it and see.

This is an attempt to write something in rust after a little long, and this was amazing. This just tells me to use any proc macros from other crates with utmost attention and only after careful reading otherwise just opening the project could have catastrophic effects.


This is a learning drawn from [rust-docs](https://rust-analyzer.github.io/manual.html#security) and [do-not-compile-this-code](https://github.com/eleijonmarck/do-not-compile-this-code)