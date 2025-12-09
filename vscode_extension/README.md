# Lustre LSP Implementation

## Features

- [x] simple formatting 
<!-- \!\[feature X\]\(images/feature-x.png\) -->

## Requirements

If you have any requirements or dependencies, add a section describing those and how to install and configure them.

## Release Notes

Users appreciate release notes as you update your extension.

### 0.1.0

Basic Lustre implementation working.

### 0.0.1

Just testing some stuff.

---

<!-- ## Working with Markdown -->

<!-- You can author your README u>> Here ERROR "x0 fby x1" != "x1"
>> Here ERROR "1 -> x0 fby x1" != "x0 fby x1"
>> Here ERROR "1 -> 1 -> x0 fby x1" != "1 -> 1 -> x0 fby x1"
>> Here ERROR "node a() returns ();
let
	z = 1 -> 1 -> x0 fby x1;
tel" != "node a() returns ();
let
	z = 1 -> 1 -> x0 fby x1;
tel"
>> Here ERROR "node a() returns ();
let
	z = 1 -> 1 -> x0 fby x1;
tel
" != "node a() returns ();
let
	z = 1 -> 1 -> x0 fby x1;
tel
"
>> in | parse | display | parse :
node a() returns ();
let
	z = 1 -> 1 -> x0 fby x1;
tel

PASSED: in | parse_rest == ""
FAILED: in | parse | display | parse shallow== in | parse
PASSED: in | parse | display | parse | display == in | parse | display

thread 'parser::test::test::chain_operator_2_ok' panicked at analyzer/src/parser/test.rs:132:9:
explicit panic


failures:
    parser::test::test::arrow_plus_chain_ok
    parser::test::test::chain_operator_2_ok

test result: FAILED. 78 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p lustre_analyzer --lib`
➜  Lustre_Project git:(main) ✗ sing Visual Studio Code. Here are some useful editor keyboard shortcuts: -->

<!-- * Split the editor (`Cmd+\` on macOS or `Ctrl+\` on Windows and Linux). -->
<!-- * Toggle preview (`Shift+Cmd+V` on macOS or `Shift+Ctrl+V` on Windows and Linux). -->
<!-- * Press `Ctrl+Space` (Windows, Linux, macOS) to see a list of Markdown snippets. -->

<!-- ## For more information -->

<!-- * [Visual Studio Code's Markdown Support](http://code.visualstudio.com/docs/languages/markdown) -->
<!-- * [Markdown Syntax Reference](https://help.github.com/articles/markdown-basics/) -->

<!-- **Enjoy!** -->
