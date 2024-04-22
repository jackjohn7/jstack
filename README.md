# JStack

JStack is a array/stack-based programming language
developed in Rust.

- All expressions return a value
  - Even closures (denoted by parens) return values
  - `out` keyword returns zero

```jstack
// outputs the sum of 5 and 10 to stdout
5 10 + out;

// string interpolation
7 8 + "The sum is $\n" fmt out; // writes "The sum is 15\n"

// closures scope modifications to stack and tape
7 8 {+ "val: $" fmt out} pop; // evaluates to 8

// closures can be passed into comparison structures
// comparison structures take positional arguments
8 8 eq({ "They match" } { "They do not match" }) out;

// closures can be named
#one { "they match" };
#two { "they don't match" };
8 8 eq(one two) out;
```
