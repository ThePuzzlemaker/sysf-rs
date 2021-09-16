# System F in Rust

This is an implementation of [Dunfield and Krishnaswami's "Complete and Easy Bidirectional Typechecking for Higher-Rank Polymorphism"](https://research.cs.queensu.ca/home/jana/papers/bidir/) in Rust.

Very WIP at the moment.

## TODOs

- [ ] Clean up code
- [ ] Proper REPL and CLI
- [ ] Language-level fixpoint and conditional, maybe sum types/product types or isorecursive types, and/or other extensions
- [ ] Evaluator
- [ ] Actually use files and stuff
- [ ] Definitions
- [ ] Maybe add simple module includes?

## Syntax

```ebnf
(* =:= Basic definitions =:= *)
alpha_upper = "A" | "B" | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K"
            | "L" | "M" | "N" | "O" | "P" | "Q" | "R" | "S" | "T" | "U" | "V"
            | "W" | "X" | "Y" | "Z" ;
alpha_lower = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k"
            | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v"
            | "w" | "x" | "y" | "z" ;
alpha       = alpha_upper | alpha_lower ;
digit       = "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" ;

(* Whitespace is ignored. *)
whitespace = "\r" | "\n" | "\r\n" | " " | "\t" ;

(* =:= Identifiers =:= *)

ident_start    = alpha ;
ident_continue = ident_begin | digit | "_" ;

ident = ident_begin, [ ident_cont ] ;
tyvar = "'", ident ;

(* =:= Types =:= *)

type = "unit" | "bool"   (* Primitives               *)
     | tyvar, "=>", type (* Universal quantification *)
     | type, "->", type  (* Arrow                    *)
     | tyvar             (* Type variable            *)
     | "(", type, ")"    (* Grouping                 *)

(* =:= Terms =:= *)

term = "true" | "false"       (* Booleans           *)
     | "()"                   (* Unit               *)
     | "\\", ident, ".", term (* Lambda-abstraction *)
     | term, term             (* Application        *)
     | term, "[", type, "]"   (* Type application   *)
     | term, ":", type        (* Annotation         *)
     | ident                  (* Variable           *)
     | "(", term, ")"         (* Grouping           *)
```

See [`src/grammar.lalrpop`](src/grammar.lalrpop) for the exact grammar used by the parser.

## Overall Process

1. Parse input.
2. Lower surface AST to core AST (i.e. convert named variables to De Bruijn indices)
3. Run typechecking! (WIP)
4. Evaluate (WIP)

## Debugging

This project uses the [`tracing`](https://lib.rs/tracing) crate, so you can use the `RUST_LOG` flag to control what tracing information is printed. This is currently only in the typechecker, but will be extended further at some point. Tracing can be disabled at compile-time (which will not compile any tracing infrastructure at all) by disabling the `trace` feature.

You can set the debugging mode by commenting out/uncommenting one of the lines at the start of `fn main()` in [`src/main.rs`](src/main.rs), though this will be moved to a runtime method at some point in the future.

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.