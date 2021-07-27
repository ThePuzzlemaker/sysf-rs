# sysf

A work-in-progress (and bad) implementation of a System F typechecker (WIP) and evaluator (not started yet).

## TODOs

- Clean up code
- REPL
- Make it so you don't repeat type parameters in the lambda and the type
- Actual (type) variable scoping, probably using De Bruijn indices
- Conditionals, it's typechecked but not in the parser

## Syntax

See [`src/grammar.lalrpop`](src/grammar.lalrpop) for the syntax definition used in the parser. This is an overview:

### Terms
- `f.t`: Apply `f` (term) with `t` (term)
- `|x| t`: Lambda with variable `x` (`[a-zA-Z][a-zA-Z0-9_]`), returning `t` (term)
- `<'x> t`: Polymorphic lambda with type variable `'x` (including apostrophe, `'[a-zA-Z][a-zA-Z0-9_]`), returning `t` (term)
- `f[T]`: Apply polymorphic lambda `f` (term) with `T` (type)
- `true`: True
- `false`: False
- `v`: Variable reference `v` (`[a-zA-Z][a-zA-Z0-9_]`)
- `(t)`: Grouped term `t`

### Types
- `Bool`: Boolean
- `t :: T`: Annotate `t` (term) with type `T` (type)
- `A -> B`: Lambda from `A` (type) to `B` (type)
- `'x => T`: Polymorphic lambda with type variable `'x` (including apostrophe, `'[a-zA-Z][a-zA-Z0-9_]`), returning `T` (type)
- `(T)`: Grouped type `T`

## Examples

- Boolean identity: `|x| x :: Bool -> Bool`
- Polymorphic identity: `<'x> |x| x :: 'x => 'x -> 'x`
- Explicit polymorphic application of identity with a boolean: `(<'x> |x| x :: 'x => 'x -> 'x)[Bool].true`
- Inferred type application of identity with a boolean: `(<'x> |x| x :: 'x => 'x -> 'x).true`
- Polymorphic const: `<'x> <'y> |x| |y| x :: 'x => 'y => 'x -> 'y -> 'x`
- Application of polymorphic const with booleans: `(<'x> <'y> |x| |y| x :: 'x => 'y => 'x -> 'y -> 'x).true.false`

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