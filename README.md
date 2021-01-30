<!-- -*-GFM-*- -->

# PyASTRs

A Python AST library in Rust.

**WARNING: this is a learning project at the moment and very much a work in progress.
Please do not rely on it for anything. I will remove this warning if it ever gets
to a point where I'm happy with it.**

## Goal

My goal is to create a library that works similarly to the lexer/parser in
[rust-analyzer]. This means:

1. Parse Python code
2. Construct a lossless untyped syntax (token) tree
3. Construct a typed AST

The goal of all of this is to potentially provide a base for performing analysis
and refactoring of Python code in a way that's flexible and performant.

Rust-analyzer being one of the fastest, most-featureful code analysis tools I
know of, much of its implementation will serve as an inspiration for this library.

## License

Even though this is at the moment a just-for-fun project, I have licensed it
under the LGPLv2, a permissive copyleft license.

**Open source projects under similar or more permissive licenses can therefore
use this library with no restrictions** (they are "works that use the library"
in license terms, and by virtue of being open source adhere to the requirements
in section 6 for distributing a combined work).

Proprietary projects can use this library, provided they do one of the following:
- Link to an existing version of the library on a user's system rather than
  distributing the library directly
- Use dynamic linking so that it is easy for the user to use a local or modified
  version of the library if they choose
- Along with any static or combined executable, provide the full source of this
  library and any needed project files as object or source so that the user can
  recompile and relink to a modified version of this library

Proprietary projects must also ensure that notice is provided that this library
is being used and that it is licensed under the LGPL.

Proprietary projects may not use this library if they are distributed under a
license that prevents the user from reverse engineering or modifying the project.

Modifying this library and including it in a project is only allowed if that
project is a library and is also licensed under the LGPL.
  
Of course, I am not a lawyer, and the real license takes precedence over my
interpretation. That being said, I'll provide my perspective on some common
cases in less legal language.
     
