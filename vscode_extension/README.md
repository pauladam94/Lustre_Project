# Lustre LSP Implementation

## Features

- Formatting
- Diagnostics
- Inlay Hint for types
- Running test

Here is some code that this extension can analyzed and run.

As an inlay hint right next to the `#[test]` we have information
that the test function have passed or not.

```lustre
node fibonaci() returns (z: int);
let
  x0 = 0 fby z;
  x1 = 1 fby x0;
  z = x0 + x1;
tel 

#[test]
node test() returns (z: bool);
let
  lhs = [1, 1, 2, 3, 5, 8];
  rhs = fibonaci(() ^ 6); 
  z = lhs == rhs;
tel 
```

Tests are nodes of such forms :

```
node name_test() return (z: bool);
let
  ...
tel
```

They pass if the compiler managed to propagate
constant so that the core of the node finally is only :

```
...
let
  z = true;
tel
```

## Release Notes

### 0.4.0

- better typing error
- bool + int not accepted anymore
- fix of ..[-1]

### 0.3.0

- better contant propagation
- support caret '^' syntax
- support index '[_]' syntax

### 0.2.0

- working arrays and tuple

### 0.1.0

- wasm extension (normally working everywhere)
- diagnostics
- better inlay hints
- support for arrays and tuple (not completely)
- basic lustre implementation working.

### 0.0.1

- testing some stuff.

