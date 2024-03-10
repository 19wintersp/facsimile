# Facsimile

**A simple demonstrative Lisp-like language.**

Interpreter for a simple language based on Lisp's paradigms and syntax. Usable
as a Rust crate as well as with a simple CLI.

Licensed under the MIT licence.

## Language overview

### Syntax

Facsimile uses an interpretation of Lisp's syntax as its foundation. The most
elementary part of the syntax is _atoms_: basic data values. Facsimile makes use
of "numbers" (single-precision IEEE-754 floats), "strings" (Unicode strings),
and booleans. Numbers are specified as atoms with the standard decimal
representation, and can also use exponents. Strings begin and end with either
`"` or `'`, and may contain escapes formed of two characters, the first being a
backslash. Booleans can be given by the atoms `true` and `false`. Symbols are
another form of atom in Facsimile, and exist as alphanumeric (with underscores)
identifiers. Whilst they can reference other data or functions, they can be
treated as data in the same way as other primitive types.

Most of the program structure is formed by lists. In accordance with Lisp's
programming paradigm, data and code are homogenous, and so lists can represent
actual lists - as a data structure consisting of an array of other data/code -
or code, depending on the context. Lists are formed of whitespace-separated
atoms enclosed in regular brackets (`(` and `)`). A special atom, `nil`, is
equivalent to an empty list (`()`). When representing code, the first element of
the list is either a symbol identifying the function to be called, or a lambda
function. The function specified is then applied, with the remaining elements of
the list acting as arguments. If any of these arguments are lists, they are also
evaluated as code before being passed to the function. If any of them are
symbols, the context attempts to resolve them as variables.

Comments are available either as single-line (preceded by `//`) or continuous
(commencing with `/*` and ending with `*/`).

### Special functions

The language provides several special functions for important features of most
programming. These special functions are still applied with the usual function
syntax (with lists, where the first element is a symbol of the function), though
do not behave in the same way.

Control flow is provided through the `if` function. It takes one or more flow
branches, each governed by a condition, as well as an optional final (`else`)
branch. Conditions are invoked in order, and the first one to return a truthy
will have its following block evaluated, with the `if` block itself returning
that block's result. If none of the conditionals resolve, the final block will
be evaluated in the same manner if provided, otherwise `nil` will be returned.

```
(if
  a           // if a {
  (print "a") //   println("a");
  b           // } else if b {
  (print "b") //   println("b");
  c           // } else if c {
  (print "c") //   println("c");
              // } else {        (blank line illustrative only)
  (print "x") //   println("x");
)             // }
```

Special short-circuiting `and` (aliased as `all`) and `or` (aliased as `any`)
functions are also provided, and operate in a simple manner: they require either
all or any (repectively) of their arguments to be truthy, and return the first
non-truthy or truthy (respectively) argument, or `nil`.

User-defined functions are created as either global functions (with `def`) or as
anonymous lambda functions (with `fun`). Both `def` and `fun` use the same
syntax, with the exception of `def` having the intended name of the function as
the first argument. The first (`fun`)/second (`def`) argument is a list of
symbols defining the arguments required by the function. Any remaining arguments
form the body of the function, and are evaluated in order. The result of
evaulating the last expression in the body is "returned" from the function to
its caller.

```
(def myfunction (a b c) // fn myfunction(a, b, c) {
  (print a)             //   println(a);
  (add a b c)           //   return a + b + c;
)                       // }
```

Functions can be called by their symbol, effectively delaying the dereferencing:

```
(def callwith1 (f n) (call f n 1))
(print 1 2 (callwith1 (quote add) 2))
```

Often, it may be necessary to pass a symbol or list to a function without it
being evaluated as a function call or variable reference respectively. To do so,
the `quote` function can be used, which simply returns its first argument,
preventing it from being evaluated. To do the same in construction of a list,
the `list` function is to be used.

```
(somefunction ("hello" "world"))         // error: "hello" isn't callable
(somefunction (quote ("hello" "world"))) // works
(somefunction (list "hello" "world"))    // works
```

Conversely, one may wish to combine many expressions together, similarly to how
function bodies work. This can be accomplished with `block`, which uses the same
rules concerning the final expression and order.

```
(if a
  (block
    (print "abc")
    (print "def")
  )
)
```

### Standard library

The standard library is automatically included in all contexts, and contains the
following functions:

- `not`
- `eq`
- `ne`
- `lt`
- `gt`
- `lte`
- `gte`
- `add`
- `sub`
- `mul`
- `div`
- `rem`
- `get`
- `num`
- `fmt`
- `cat`
- `print`
- `input`
- `srand`
- `rand`

The standard CLI also passes a special variable, `args`, which is a list of
strings containing the path of the entrypoint source file followed by any other
command-line arguments passed to the CLI afterwards.

## Maintenance

Changes have been made to this project for reasons. This is not an indication of
maintenance nor a sign of confidence in the code.
