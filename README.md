# Lustre_Project

Project around the Lustre programming language aiming at providing :
- [x] a parser
- [~] a type checker
- [~] an interpreter
- [x] a lsp implementation for IDE features for the language
- [ ] a graphical interface to build apps with Lustre in slint

## TODO

- [x] 0 -> pre x (mauvaise valeur de x)
- [ ] tableaux égaux pas être egaux
- [ ] affichages des hints de valeur de variables qui restent- [ ] clock support
- [ ] support for merge 
- [ ] support for reset 
- [x] support for 'if then else'

### Parser
- [ ] fix error parsing of 1 -> 1 -> a + b
- [ ] parse comments
- [ ] to a lexing phase first (maybe in a lazy way)
- [ ] parse tuple on the left of expression
- [x] parse tuple in expression
- [ ] parse vars
- [ ] more faulty parser
- [ ] loss less parser
- [ ] one more test for loss less parser : input ==nws parse | input | display_debug
- [x] parse arguments variants (x, y : int) instead of (x : int, y: int)

### Type checker
- [ ] checking clocks 
- [ ] test cyclic definition inside a node itself
- [ ] test non cyclic definitions of function between each other
- [x] type check functions call
- [x] good type check of 'pre' not initialized type

### LSP
- [x] better semantic tokens
- [x] inlay hints of type
- [x] test inlay hint if the test pass or not
- [x] constant propagate draw for output of function 

### Interpreter
- [ ] hightlight by the lsp
- [x] interpreter in the lsp
- [ ] Untyped Value for faster interpretation

### Compiler
- [ ] compile a CompileNode to a Rust program that can be compiled

### Graphical interface
- [ ] basic grid support with slint
- [ ] compilation of a program to a working grid application
- [ ] modifiying the code of a block modify the whole code
