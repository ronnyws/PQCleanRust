# PQClean c2rust

This directory contains support for the reimplementation of the PQClean
code from C into Rust.
This reimplementation can be more of a transliteration, because of the
following characteristics of the PQClean code:

* It only allocates memory on the stack;
* The sizes of the stack allocations are known at compile-time;
* It uses a limited number of types (integers and arrays of integers);
* The functions have no side effects.

## Reimplementation plan.

This is action plan for the reimplementation.

### Transliteration function by function.

* The original code is left untouched;
* The output of the C function and Rust function is compared for equality;

## Questions

### Bounds checks

When are these removed?

### Rust environment

Which Rust version to aim for?

