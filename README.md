# Lustre_Project

Project around the Lustre programming language aiming at providing :
- [x] a parser
- [x] a type checker
- [x] an interpreter
- [x] a lsp implementation for basic IDE features for the language
- [ ] a graphical interface to build apps with Lustre in slint

## TODO

### Parser
- [ ] parse comments
- [ ] support for 'if then else'
- [ ] parse tuple on the left of expression
- [ ] parse tuple in expression
- [ ] parse vars
- [ ] more faulty parser
- [ ] loss less parser
- [ ] one more test for loss less parser : input ==nws parse | input | display_debug
- [x] parse arguments variants (x, y : int) instead of (x : int, y: int)

### Type checker
- [x] type check functions call
- [x] good type check of 'pre' not initialized type

### LSP
- [x] better semantic tokens
- [x] inlay hints of type
- [ ] test inlay hint if the test pass or not

### Interpreter
- [ ] hightlight by the lsp
- [x] interpreter in the lsp

